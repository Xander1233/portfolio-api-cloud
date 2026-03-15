variable "name_prefix" {
  type = string
}

variable "tags" {
  type = map(string)
}

variable "aws_region" {
  type = string
}

variable "vpc_id" {
  type = string
}

variable "subnet_id" {
  type = string
}

variable "cloudfront_origin_prefix_list_id" {
  type = string
}

variable "app_port" {
  type = number
}

variable "instance_type" {
  type = string
}

variable "ami_id" {
  type    = string
  default = null
}

variable "app_image_uri" {
  type = string
}

variable "app_config_s3_bucket_arn" {
  type = string
}

variable "app_config_s3_prefix" {
  type = string
}

variable "app_config_s3_config_uri" {
  type = string
}

variable "secrets_arns" {
  type = list(string)
}

variable "dynamodb_arns" {
  type = list(string)
}

variable "log_retention_days" {
  type = number
}
