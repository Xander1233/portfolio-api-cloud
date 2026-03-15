variable "name_prefix" {
  type = string
}

variable "tags" {
  type = map(string)
}

variable "cloudflare_zone_id" {
  type = string
}

variable "public_domain_name" {
  type = string
}

variable "certificate_domain" {
  type = string
}

variable "public_web_acl_name" {
  type = string
}

variable "public_bucket_name" {
  type = string
}

variable "public_origin_access_control_id" {
  type = string
}
