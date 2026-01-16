data "aws_region" "current" {}
data "aws_caller_identity" "current" {}

locals {
  name = {
    default = var.project
    api     = "${var.project}-api"
    public  = "${var.project}-public"
  }
  tags = {
    Project = var.project
  }
  log_group_name = "/ec2/${local.name.api}"
  log_group_arn  = "arn:aws:logs:${data.aws_region.current.name}:${data.aws_caller_identity.current.account_id}:log-group:${local.log_group_name}:*"
}

import {
  to = aws_cloudfront_distribution.api_distribution
  id = var.api_cf_id
}

# --- VPC ---
data "aws_vpc" "default" {
  default = true

  tags = merge(local.tags, {
    Name = "${local.name.default}-vpc"
  })
}

data "aws_subnet" "this" {
  filter {
    name = "vpc-id"
    values = [data.aws_vpc.default.id]
  }

  filter {
    name = "subnet-id"
    values = [var.subnet_id]
  }
}

# --- EC2 instance for API ---

### IAM Role for EC2 instance
data "aws_iam_policy_document" "ec2_trust" {
  statement {
    actions = ["sts:AssumeRole"]
    principals {
      type        = "Service"
      identifiers = ["ec2.amazonaws.com"]
    }
  }
}

resource "aws_iam_role" "ec2" {
  name               = "${local.name.api}-ec2-role"
  assume_role_policy = data.aws_iam_policy_document.ec2_trust.json

  tags = merge(local.tags, {
    Name = "${local.name.api}-ec2"
  })
}

data "aws_iam_policy_document" "ec2_permissions" {
  statement {
    actions   = ["s3:ListBucket"]
    resources = [var.app_config_s3_bucket_arn]

    condition {
      test     = "StringLike"
      variable = "s3:prefix"
      values   = ["${var.app_config_s3_prefix}*"]
    }
  }

  statement {
    actions   = ["s3:GetObject"]
    resources = ["${var.app_config_s3_bucket_arn}/${var.app_config_s3_prefix}*"]
  }

  statement {
    actions   = ["secretsmanager:GetSecretValue", "secretsmanager:DescribeSecret"]
    resources = var.secrets_arns
  }

  statement {
    sid    = "DynamoDbAccess"
    effect = "Allow"
    actions = [
      "dynamodb:GetItem",
      "dynamodb:PutItem",
      "dynamodb:UpdateItem",
      "dynamodb:DeleteItem",
      "dynamodb:Query",
      "dynamodb:Scan",
      "dynamodb:BatchGetItem",
      "dynamodb:BatchWriteItem",
      "dynamodb:DescribeTable"
    ]
    resources = var.dynamodb_arn
  }

  statement {
    sid    = "CloudWatchLogsWriteToOneGroup"
    effect = "Allow"
    actions = [
      "logs:CreateLogStream",
      "logs:DescribeLogStreams",
      "logs:PutLogEvents",
      "logs:PutRetentionPolicy"
    ]
    resources = [local.log_group_arn]
  }

  statement {
    sid      = "CloudWatchLogsCreateGroup"
    effect   = "Allow"
    actions  = ["logs:CreateLogGroup"]
    resources = ["*"]
  }
}

resource "aws_iam_policy" "ec2_permissions" {
  name   = "${local.name.api}-ec2-permissions"
  policy = data.aws_iam_policy_document.ec2_permissions.json

  tags = merge(local.tags, {
    Name = "${local.name.api}-ec2"
  })
}

resource "aws_iam_role_policy_attachment" "ec2_permissions" {
  role       = aws_iam_role.ec2.name
  policy_arn = aws_iam_policy.ec2_permissions.arn
}

resource "aws_iam_role_policy_attachment" "ssm_core" {
  role       = aws_iam_role.ec2.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore"
}

resource "aws_iam_instance_profile" "ec2" {
  name = "${local.name.api}-ec2-profile"
  role = aws_iam_role.ec2.name

  tags = merge(local.tags, {
    Name = "${local.name.api}-ec2"
  })
}

### Security Group for API instance
resource "aws_security_group" "api_sg" {
  name        = "${local.name.default}-sg"
  description = "Security group for API instance"
  vpc_id      = data.aws_vpc.default.id

  ### Allow inbound HTTP traffic from anywhere
  ingress {
    description = "Allow HTTP from anywhere"
    from_port   = var.app_port
    to_port     = var.app_port
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
    ipv6_cidr_blocks = ["::/0"]
  }

  ### Allow inbound SSH traffic from anywhere (for management)
  ingress {
    description = "Allow SSH from anywhere"
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
    ipv6_cidr_blocks = ["::/0"]
  }

  ### Allow all outbound traffic
  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
    ipv6_cidr_blocks = ["::/0"]
  }

  tags = merge(local.tags, {
    Name = "${local.name.default}-sg"
  })
}

### AMI - Ubuntu 24.04 LTS
data "aws_ami" "ubuntu" {
  filter {
    name   = "image-id"
    values = [var.ami_id]
  }
}

### EC2 instance running the API container
resource "aws_instance" "api" {
  ami                    = data.aws_ami.ubuntu.id
  instance_type          = var.instance_type
  vpc_security_group_ids = [aws_security_group.api_sg.id]

  iam_instance_profile = aws_iam_instance_profile.ec2.name

  metadata_options {
    http_tokens                 = "required"
    http_put_response_hop_limit = 2
  }

  user_data = templatefile("${path.module}/user_data.sh", {
    image_uri                   = var.image_uri
    container_name              = local.name.api
    host_port                   = 80
    container_port              = 8000
    s3_config_uri               = var.app_config_s3_config_uri
    container_config_mount_path = "/app/config/config.toml"
    logs_dir                    = "/var/log/${local.name.api}"
    log_group_name              = local.log_group_name
    aws_region                  = var.aws_region
    extra_docker_args           = "--log-driver=journald"
  })

  user_data_replace_on_change = true

  tags = merge(local.tags, {
    Name = "${local.name.api}-ec2"
  })

  depends_on = [aws_security_group.api_sg]
}

# --- CloudFront Distribution ---

### Certificate for CloudFront (must be in us-east-1)
data "aws_acm_certificate" "api_certificate" {
  domain      = var.cf_dns_zone
  types       = ["AMAZON_ISSUED"]
  most_recent = true

  provider    = aws.us
}

data "aws_cloudfront_cache_policy" "api_cache_policy" {
  name = "Managed-CachingDisabled"
}

data "aws_cloudfront_origin_request_policy" "api_origin_request_policy" {
  name = "Managed-AllViewerAndCloudFrontHeaders-2022-06"
}

data "aws_cloudfront_response_headers_policy" "api_response_headers_policy" {
  name = "Managed-CORS-with-preflight-and-SecurityHeadersPolicy"
}

data "aws_wafv2_web_acl" "api_web_acl" {
  provider = aws.us

  name = var.api_web_acl_name
  scope = "CLOUDFRONT"
}

resource "aws_cloudfront_distribution" "api_distribution" {
  enabled = true
  aliases = [var.api_domain_name]

  origin {
      domain_name = aws_instance.api.public_dns
      origin_id   = "api-origin"
      custom_origin_config {
        http_port              = var.app_port
        https_port             = 443
        origin_protocol_policy = "http-only"
        origin_ssl_protocols   = ["TLSv1.2", "SSLv3", "TLSv1", "TLSv1.1"]
      }
  }

  is_ipv6_enabled     = true
  default_root_object = ""
  default_cache_behavior {
    allowed_methods  = ["GET", "HEAD", "OPTIONS", "PUT", "POST", "PATCH", "DELETE"]
    cached_methods   = ["GET", "HEAD"]
    target_origin_id = "api-origin"
    compress = true

    viewer_protocol_policy = "redirect-to-https"

    origin_request_policy_id = data.aws_cloudfront_origin_request_policy.api_origin_request_policy.id
    response_headers_policy_id = data.aws_cloudfront_response_headers_policy.api_response_headers_policy.id
    cache_policy_id = data.aws_cloudfront_cache_policy.api_cache_policy.id
  }

  web_acl_id = data.aws_wafv2_web_acl.api_web_acl.arn

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }
  viewer_certificate {
    acm_certificate_arn            = data.aws_acm_certificate.api_certificate.arn
    ssl_support_method             = "sni-only"
    minimum_protocol_version       = "TLSv1.2_2021"
  }

  tags = merge(local.tags, {
    Name = "${local.name.api}-cf"
  })
}

# --- Cloudflare DNS Records ---
data "cloudflare_zone" "cf_zone" {
  filter = {
    name = var.cf_dns_zone
  }
}

resource "cloudflare_dns_record" "api_dns" {
  zone_id = data.cloudflare_zone.cf_zone.id
  name    = var.api_domain_name
  content = aws_cloudfront_distribution.api_distribution.domain_name
  type    = "CNAME"
  ttl     = 1
  comment = "AWS CloudFront distribution for API"
  proxied = true
}

# --- Public Frontend

# S3 Bucket for static website hosting
data "aws_s3_bucket" "public" {
  bucket = var.s3_bucket
}

# Cloudfront Distribution for Public Frontend

### Certificate for CloudFront (must be in us-east-1)
data "aws_acm_certificate" "public_certificate" {
  domain      = var.cf_dns_zone
  types       = ["AMAZON_ISSUED"]
  most_recent = true

  provider    = aws.us
}

data "aws_cloudfront_cache_policy" "public_cache_policy" {
  name = "Managed-CachingOptimized"
}

data "aws_wafv2_web_acl" "public_web_acl" {
  provider = aws.us

  name = var.public_web_acl_name
  scope = "CLOUDFRONT"
}

data "aws_cloudfront_origin_access_control" "public_access_control" {
  id = var.public_origin_ac
}

resource "aws_cloudfront_distribution" "public_distribution" {
  enabled = true
  aliases = [var.public_domain_name, "www.${var.public_domain_name}"]

  origin {
    domain_name = data.aws_s3_bucket.public.bucket_domain_name
    origin_id   = "public-origin"
    origin_path = "/public"

    origin_access_control_id = data.aws_cloudfront_origin_access_control.public_access_control.id
  }

  is_ipv6_enabled     = true
  default_root_object = "index.html"
  default_cache_behavior {
    allowed_methods  = ["GET", "HEAD"]
    cached_methods   = ["GET", "HEAD"]
    target_origin_id = "public-origin"
    compress = true

    viewer_protocol_policy = "redirect-to-https"

    cache_policy_id = data.aws_cloudfront_cache_policy.public_cache_policy.id
  }

  web_acl_id = data.aws_wafv2_web_acl.public_web_acl.arn

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }
  viewer_certificate {
    acm_certificate_arn            = data.aws_acm_certificate.public_certificate.arn
    ssl_support_method             = "sni-only"
    minimum_protocol_version       = "TLSv1.2_2021"
  }

  custom_error_response {
    error_caching_min_ttl = 60
    error_code            = 403
    response_code         = 200
    response_page_path    = "/index.html"
  }

  tags = merge(local.tags, {
    Name = "${local.name.public}-cf"
  })
}

resource "cloudflare_dns_record" "public_dns" {
  zone_id = data.cloudflare_zone.cf_zone.id
  name    = var.public_domain_name
  content = aws_cloudfront_distribution.public_distribution.domain_name
  type    = "CNAME"
  ttl     = 1
  comment = "AWS CloudFront distribution for Public Frontend"
  proxied = true
}

resource "cloudflare_dns_record" "public_www_dns" {
  zone_id = data.cloudflare_zone.cf_zone.id
  name    = "www.${var.public_domain_name}"
  content = aws_cloudfront_distribution.public_distribution.domain_name
  type    = "CNAME"
  ttl     = 1
  comment = "AWS CloudFront distribution for Public Frontend (www)"
  proxied = true
}
