data "aws_s3_bucket" "public" {
  bucket = var.public_bucket_name
}

data "aws_acm_certificate" "public" {
  provider = aws.us

  domain      = var.certificate_domain
  types       = ["AMAZON_ISSUED"]
  most_recent = true
}

data "aws_cloudfront_cache_policy" "public" {
  name = "Managed-CachingOptimized"
}

data "aws_wafv2_web_acl" "public" {
  provider = aws.us
  name     = var.public_web_acl_name
  scope    = "CLOUDFRONT"
}

data "aws_cloudfront_origin_access_control" "public" {
  id = var.public_origin_access_control_id
}

resource "aws_cloudfront_distribution" "public" {
  enabled         = true
  aliases         = [var.public_domain_name, "www.${var.public_domain_name}"]
  web_acl_id      = data.aws_wafv2_web_acl.public.arn
  price_class     = "PriceClass_All"
  is_ipv6_enabled = true

  origin {
    domain_name              = data.aws_s3_bucket.public.bucket_domain_name
    origin_id                = "public-origin"
    origin_path              = "/public"
    origin_access_control_id = data.aws_cloudfront_origin_access_control.public.id
  }

  default_root_object = "index.html"

  default_cache_behavior {
    allowed_methods        = ["GET", "HEAD"]
    cached_methods         = ["GET", "HEAD"]
    target_origin_id       = "public-origin"
    compress               = true
    viewer_protocol_policy = "redirect-to-https"
    cache_policy_id        = data.aws_cloudfront_cache_policy.public.id
  }

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }

  viewer_certificate {
    acm_certificate_arn      = data.aws_acm_certificate.public.arn
    ssl_support_method       = "sni-only"
    minimum_protocol_version = "TLSv1.2_2021"
  }

  custom_error_response {
    error_caching_min_ttl = 60
    error_code            = 403
    response_code         = 200
    response_page_path    = "/index.html"
  }

  tags = merge(var.tags, {
    Name = "${var.name_prefix}-public-cf"
  })
}

resource "cloudflare_dns_record" "public" {
  zone_id = var.cloudflare_zone_id
  name    = var.public_domain_name
  content = aws_cloudfront_distribution.public.domain_name
  type    = "CNAME"
  ttl     = 1
  proxied = true
  comment = "Public CloudFront distribution"
}

resource "cloudflare_dns_record" "public_www" {
  zone_id = var.cloudflare_zone_id
  name    = "www.${var.public_domain_name}"
  content = aws_cloudfront_distribution.public.domain_name
  type    = "CNAME"
  ttl     = 1
  proxied = true
  comment = "Public CloudFront distribution (www)"
}
