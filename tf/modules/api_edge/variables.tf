variable "name_prefix" {
  type = string
}

variable "tags" {
  type = map(string)
}

variable "cloudflare_zone_id" {
  type = string
}

variable "api_domain_name" {
  type = string
}

variable "certificate_domain" {
  type = string
}

variable "api_web_acl_name" {
  type = string
}

variable "origin_domain_name" {
  type = string
}

variable "origin_http_port" {
  type = number
}
