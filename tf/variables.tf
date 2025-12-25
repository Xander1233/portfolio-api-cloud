variable "aws_region" {
  type    = string
  default = "eu-central-1"
}

variable "project" {
  type    = string
  default = "portfolio-api"
}

variable "api_domain_name" {
  description = "FQDN for the API, e.g. api.example.com"
  type        = string
}

variable "public_domain_name" {
  description = "FQDN for the public frontend, e.g. example.com"
  type        = string
}

variable "hosted_zone_id" {
  description = "Route53 Hosted Zone ID that matches the domain"
  type        = string
}

variable "instance_type" {
  type    = string
  default = "t4g.nano"
}

variable "app_port" {
  type    = number
  default = 80
}

variable "vpc_id" {
  description = "VPC ID where resources will be deployed"
  type        = string
}

variable "subnet_id" {
  description = "Subnet ID within the VPC"
  type        = string
}

variable "ami_id" {
  description = "AMI ID for the EC2 instance"
  type        = string
  default     = "ami-004e960cde33f9146" # Ubuntu 24.04 LTS in eu-central-1
}

variable "s3_bucket" {
  description = "S3 Bucket name for public frontend hosting"
  type        = string
}

variable "cf_dns_zone" {
  description = "Cloudflare DNS Zone (e.g. example.com)"
  type        = string
}

variable "cf_api_token" {
  description = "Cloudflare API Token with DNS edit permissions"
  type        = string
  sensitive   = true
}

variable "api_cf_id" {
  description = "Cloudfront Distribution ID for the API"
  type        = string
}

variable "public_web_acl_name" {
  description = "Name of the AWS WAFv2 Web ACL for the public frontend"
  type        = string
}

variable "api_web_acl_name" {
  description = "Name of the AWS WAFv2 Web ACL for the api"
  type        = string
}

variable "public_origin_ac" {
  description = "Origin Access Control for the public S3 bucket"
  type        = string
}

variable "app_config_s3_bucket_arn" {
  type        = string
  description = "e.g. arn:aws:s3:::my-bucket"
}

variable "app_config_s3_prefix" {
  type        = string
  description = "e.g. myapp/prod/ (must end with / typically)"
}

variable "app_config_s3_config_uri" {
  type        = string
  description = "S3 URI for the app config, e.g. s3://my-bucket/myapp/prod/"
}

variable "secrets_arns" {
  type        = list(string)
  description = "List of Secrets Manager secret ARNs the app may read"
}

variable "image_uri" {
  type        = string
  description = "GHCR image URI for the EC2 instance"
}

variable "dynamodb_arn" {
  type        = list(string)
  description = "ARN of the DynamoDB table used by the application"
}
