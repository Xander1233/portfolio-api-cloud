terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "6.17.0"
    }
  }
}
locals {
  api_name       = "${var.name_prefix}-api"
  log_group_name = "/ec2/${local.api_name}"
  config_prefix  = trim(var.app_config_s3_prefix, "/")
}

data "aws_ssm_parameter" "ubuntu_24_04_ami" {
  name = "/aws/service/canonical/ubuntu/server/24.04/stable/current/amd64/hvm/ebs-gp3/ami-id"
}

data "aws_iam_policy_document" "ec2_trust" {
  statement {
    actions = ["sts:AssumeRole"]

    principals {
      type        = "Service"
      identifiers = ["ec2.amazonaws.com"]
    }
  }
}

data "aws_iam_policy_document" "ec2_permissions" {
  statement {
    sid       = "ReadConfigBucketPrefix"
    actions   = ["s3:ListBucket"]
    resources = [var.app_config_s3_bucket_arn]

    condition {
      test     = "StringLike"
      variable = "s3:prefix"
      values   = ["${local.config_prefix}/*", local.config_prefix]
    }
  }

  statement {
    sid       = "ReadConfigObjects"
    actions   = ["s3:GetObject"]
    resources = ["${var.app_config_s3_bucket_arn}/${local.config_prefix}/*"]
  }

  dynamic "statement" {
    for_each = length(var.secrets_arns) > 0 ? [1] : []

    content {
      sid = "ReadSecrets"
      actions = [
        "secretsmanager:DescribeSecret",
        "secretsmanager:GetSecretValue"
      ]
      resources = var.secrets_arns
    }
  }

  statement {
    sid    = "DynamoDbAccess"
    effect = "Allow"
    actions = [
      "dynamodb:BatchGetItem",
      "dynamodb:BatchWriteItem",
      "dynamodb:DeleteItem",
      "dynamodb:DescribeTable",
      "dynamodb:GetItem",
      "dynamodb:PutItem",
      "dynamodb:Query",
      "dynamodb:Scan",
      "dynamodb:UpdateItem"
    ]
    resources = var.dynamodb_arns
  }

  statement {
    sid    = "WriteApplicationLogs"
    effect = "Allow"
    actions = [
      "logs:CreateLogStream",
      "logs:DescribeLogStreams",
      "logs:PutLogEvents"
    ]
    resources = ["${aws_cloudwatch_log_group.api.arn}:*"]
  }
}

resource "aws_cloudwatch_log_group" "api" {
  name              = local.log_group_name
  retention_in_days = var.log_retention_days

  tags = merge(var.tags, {
    Name = local.log_group_name
  })
}

resource "aws_iam_role" "ec2" {
  name               = "${local.api_name}-ec2-role"
  assume_role_policy = data.aws_iam_policy_document.ec2_trust.json

  tags = merge(var.tags, {
    Name = "${local.api_name}-ec2-role"
  })
}

resource "aws_iam_policy" "ec2_permissions" {
  name   = "${local.api_name}-ec2-permissions"
  policy = data.aws_iam_policy_document.ec2_permissions.json

  tags = merge(var.tags, {
    Name = "${local.api_name}-ec2-permissions"
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
  name = "${local.api_name}-ec2-profile"
  role = aws_iam_role.ec2.name

  tags = merge(var.tags, {
    Name = "${local.api_name}-ec2-profile"
  })
}

resource "aws_security_group" "api" {
  name        = "${var.name_prefix}-sg"
  description = "Security group for API instance"
  vpc_id      = var.vpc_id

  tags = merge(var.tags, {
    Name = "${var.name_prefix}-sg"
  })
}

resource "aws_vpc_security_group_ingress_rule" "app_from_cloudfront" {
  security_group_id = aws_security_group.api.id

  description = "App traffic from CloudFront origin-facing network"
  ip_protocol = "tcp"
  from_port   = var.app_port
  to_port     = var.app_port

  prefix_list_id = var.cloudfront_origin_prefix_list_id
}

resource "aws_vpc_security_group_egress_rule" "https_outbound_ipv4" {
  security_group_id = aws_security_group.api.id

  description = "HTTPS outbound IPv4"
  ip_protocol = "tcp"
  from_port   = 443
  to_port     = 443

  cidr_ipv4 = "0.0.0.0/0"
}

resource "aws_vpc_security_group_egress_rule" "https_outbound_ipv6" {
  security_group_id = aws_security_group.api.id

  description = "HTTPS outbound IPv6"
  ip_protocol = "tcp"
  from_port   = 443
  to_port     = 443

  cidr_ipv6 = "::/0"
}

resource "aws_vpc_security_group_egress_rule" "http_outbound_ipv4" {
  security_group_id = aws_security_group.api.id

  description = "HTTP outbound IPv4"
  ip_protocol = "tcp"
  from_port   = 80
  to_port     = 80

  cidr_ipv4 = "0.0.0.0/0"
}

resource "aws_vpc_security_group_egress_rule" "http_outbound_ipv6" {
  security_group_id = aws_security_group.api.id

  description = "HTTP outbound IPv6"
  ip_protocol = "tcp"
  from_port   = 80
  to_port     = 80

  cidr_ipv6 = "::/0"
}

resource "aws_vpc_security_group_egress_rule" "dns_udp_outbound_ipv4" {
  security_group_id = aws_security_group.api.id

  description = "DNS UDP outbound"
  ip_protocol = "udp"
  from_port   = 53
  to_port     = 53

  cidr_ipv4 = "0.0.0.0/0"
}

resource "aws_vpc_security_group_egress_rule" "dns_tcp_outbound_ipv4" {
  security_group_id = aws_security_group.api.id

  description = "DNS TCP outbound"
  ip_protocol = "tcp"
  from_port   = 53
  to_port     = 53

  cidr_ipv4 = "0.0.0.0/0"
}

resource "aws_instance" "api" {
  ami                         = coalesce(var.ami_id, data.aws_ssm_parameter.ubuntu_24_04_ami.value)
  instance_type               = var.instance_type
  subnet_id                   = var.subnet_id
  associate_public_ip_address = true
  vpc_security_group_ids      = [aws_security_group.api.id]
  iam_instance_profile        = aws_iam_instance_profile.ec2.name

  metadata_options {
    http_tokens                 = "required"
    http_put_response_hop_limit = 2
  }

  user_data = templatefile("${path.module}/templates/user_data.sh.tpl", {
    image_uri                   = var.app_image_uri
    container_name              = local.api_name
    host_port                   = var.app_port
    container_port              = 8000
    s3_config_uri               = var.app_config_s3_config_uri
    container_config_mount_path = "/app/config/config.toml"
    logs_dir                    = "/var/log/${local.api_name}"
    log_group_name              = local.log_group_name
    aws_region                  = var.aws_region
    extra_docker_args           = "--log-driver=journald"
  })

  user_data_replace_on_change = false

  tags = merge(var.tags, {
    Name = "${local.api_name}-ec2"
  })
}
