variable "name_prefix" {
  type = string
}

variable "tags" {
  type = map(string)
}

variable "github_repository" {
  type = string
}

variable "github_oidc_branches" {
  type = list(string)
}

variable "ecr_repository_arn" {
  type = string
}
