variable "name_prefix" {
  type = string
}

variable "tags" {
  type = map(string)
}

variable "vpc_id" {
  type = string
}

variable "subnet_ids" {
  type = list(string)
}

variable "api_private_ip" {
  type = string
}

variable "app_port" {
  type = number
}

variable "alert_email" {
  type = string
}

variable "health_check_path" {
  type = string
}

variable "lambda_image_tag" {
  type = string
}

variable "lambda_timeout_seconds" {
  type = number
}

variable "lambda_log_retention_days" {
  type = number
}

variable "ecr_image_retention_count" {
  type = number
}
