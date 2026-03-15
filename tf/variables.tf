variable "aws_region" {
  description = "Primary AWS region for the workload."
  type        = string
  default     = "eu-central-1"
}

variable "project" {
  description = "Project identifier used in resource names."
  type        = string
  default     = "portfolio-api"
}

variable "aws_application" {
  description = "Value for awsApplication tag on all resources."
  type        = string
}

variable "environment" {
  description = "Environment name."
  type        = string
  default     = "prod"
}

variable "cf_api_token" {
  description = "Cloudflare API token with DNS edit permissions for cf_dns_zone."
  type        = string
  sensitive   = true
}

variable "cf_dns_zone" {
  description = "Cloudflare DNS zone name, e.g. example.com."
  type        = string
}

variable "api_domain_name" {
  description = "API FQDN, e.g. api.example.com."
  type        = string
}

variable "public_domain_name" {
  description = "Public site FQDN, e.g. example.com."
  type        = string
}

variable "public_bucket_name" {
  description = "Existing S3 bucket name that hosts the public frontend assets."
  type        = string
}

variable "public_origin_access_control_id" {
  description = "CloudFront Origin Access Control id used to reach the public S3 bucket."
  type        = string
}

variable "api_web_acl_name" {
  description = "Name of the CloudFront-scope WAFv2 ACL for API distribution."
  type        = string
}

variable "public_web_acl_name" {
  description = "Name of the CloudFront-scope WAFv2 ACL for public distribution."
  type        = string
}

variable "app_image_uri" {
  description = "Container image URI for the API EC2 workload."
  type        = string
}

variable "instance_type" {
  description = "EC2 instance type for API."
  type        = string
  default     = "t3.micro"
}

variable "ami_id_override" {
  description = "Optional explicit AMI id for the API instance. If null, latest Ubuntu 24.04 from SSM is used."
  type        = string
  default     = null
}

variable "app_port" {
  description = "Public application port on the EC2 instance."
  type        = number
  default     = 80
}

variable "preferred_subnet_id" {
  description = "Optional subnet id for API instance placement. If null, first default VPC subnet is selected."
  type        = string
  default     = null
}

variable "app_config_s3_bucket_name" {
  description = "S3 bucket storing application runtime config."
  type        = string
}

variable "app_config_s3_prefix" {
  description = "S3 key prefix for app config objects (without leading slash)."
  type        = string
}

variable "app_config_s3_object_key" {
  description = "Config object filename within app_config_s3_prefix."
  type        = string
  default     = "config.toml"
}

variable "secrets_arns" {
  description = "Secrets Manager ARNs the API may read."
  type        = list(string)
  default     = []
}

variable "dynamodb_arns" {
  description = "Additional DynamoDB table ARNs the API may access."
  type        = list(string)
  default     = []
}

variable "alert_email" {
  description = "Email destination for health-check alerts."
  type        = string
}

variable "health_check_path" {
  description = "Path called by the health-check Lambda on the API instance."
  type        = string
  default     = "/health"
}

variable "health_lambda_image_tag" {
  description = "ECR image tag deployed for the health-check Lambda."
  type        = string
  default     = "latest"
}

variable "health_lambda_timeout_seconds" {
  description = "Lambda timeout in seconds."
  type        = number
  default     = 15
}

variable "log_retention_days" {
  description = "CloudWatch Logs retention in days for API logs."
  type        = number
  default     = 30
}

variable "lambda_log_retention_days" {
  description = "CloudWatch Logs retention in days for Lambda logs."
  type        = number
  default     = 30
}

variable "ecr_image_retention_count" {
  description = "How many tagged Lambda images to keep in ECR."
  type        = number
  default     = 30
}

variable "github_repository" {
  description = "GitHub repository in owner/repo format used by Actions."
  type        = string
}

variable "github_oidc_branches" {
  description = "Branches allowed to assume the GitHub OIDC ECR push role."
  type        = list(string)
  default     = ["main"]
}
