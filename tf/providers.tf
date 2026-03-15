provider "aws" {
  region = var.aws_region
}

provider "aws" {
  alias  = "us"
  region = "us-east-1"
}

provider "cloudflare" {
  api_token = var.cf_api_token
}
