terraform {
  required_version = ">= 1.5.0"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 5"
    }
  }
}

provider "aws" {
  region = "eu-central-1"
  shared_credentials_files = ["C:\\Users\\David\\.aws\\credentials"]
}

provider "aws" {
  alias = "us"
  region = "us-east-1"
  shared_credentials_files = ["C:\\Users\\David\\.aws\\credentials"]
}

provider "cloudflare" {
  api_token = var.cf_api_token
}