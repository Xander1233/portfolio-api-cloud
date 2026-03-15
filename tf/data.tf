data "aws_caller_identity" "current" {}

data "aws_region" "current" {}

data "aws_vpc" "default" {
  default = true
}

data "aws_subnets" "default_vpc" {
  filter {
    name   = "vpc-id"
    values = [data.aws_vpc.default.id]
  }
}

data "aws_ec2_managed_prefix_list" "cloudfront_origin" {
  name = "com.amazonaws.global.cloudfront.origin-facing"
}

data "cloudflare_zone" "this" {
  filter = {
    name = var.cf_dns_zone
  }
}

data "aws_s3_bucket" "app_config" {
  bucket = var.app_config_s3_bucket_name
}
