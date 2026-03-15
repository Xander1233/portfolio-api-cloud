data "aws_acm_certificate" "api" {
  provider = aws.us

  domain      = var.certificate_domain
  types       = ["AMAZON_ISSUED"]
  most_recent = true
}

data "aws_cloudfront_cache_policy" "api" {
  name = "Managed-CachingDisabled"
}

data "aws_cloudfront_origin_request_policy" "api" {
  name = "Managed-AllViewerAndCloudFrontHeaders-2022-06"
}

data "aws_cloudfront_response_headers_policy" "api" {
  name = "Managed-CORS-with-preflight-and-SecurityHeadersPolicy"
}

data "aws_wafv2_web_acl" "api" {
  provider = aws.us
  name     = var.api_web_acl_name
  scope    = "CLOUDFRONT"
}

resource "aws_cloudfront_distribution" "api" {
  enabled         = true
  aliases         = [var.api_domain_name]
  web_acl_id      = data.aws_wafv2_web_acl.api.arn
  price_class     = "PriceClass_All"
  is_ipv6_enabled = true

  origin {
    domain_name = var.origin_domain_name
    origin_id   = "api-origin"

    custom_origin_config {
      http_port              = var.origin_http_port
      https_port             = 443
      origin_protocol_policy = "http-only"
      origin_ssl_protocols   = ["TLSv1.2"]
    }
  }

  default_cache_behavior {
    allowed_methods            = ["GET", "HEAD", "OPTIONS", "PUT", "POST", "PATCH", "DELETE"]
    cached_methods             = ["GET", "HEAD"]
    target_origin_id           = "api-origin"
    compress                   = true
    viewer_protocol_policy     = "redirect-to-https"
    cache_policy_id            = data.aws_cloudfront_cache_policy.api.id
    origin_request_policy_id   = data.aws_cloudfront_origin_request_policy.api.id
    response_headers_policy_id = data.aws_cloudfront_response_headers_policy.api.id
  }

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }

  viewer_certificate {
    acm_certificate_arn      = data.aws_acm_certificate.api.arn
    ssl_support_method       = "sni-only"
    minimum_protocol_version = "TLSv1.2_2021"
  }

  tags = merge(var.tags, {
    Name = "${var.name_prefix}-api-cf"
  })
}

resource "cloudflare_dns_record" "api" {
  zone_id = var.cloudflare_zone_id
  name    = var.api_domain_name
  content = aws_cloudfront_distribution.api.domain_name
  type    = "CNAME"
  ttl     = 1
  proxied = true
  comment = "API CloudFront distribution"
}
