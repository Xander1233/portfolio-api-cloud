module "api_compute" {
  source = "./modules/api_compute"

  name_prefix                      = local.name_prefix
  tags                             = local.tags
  aws_region                       = data.aws_region.current.region
  vpc_id                           = data.aws_vpc.default.id
  subnet_id                        = local.selected_subnet_id
  cloudfront_origin_prefix_list_id = data.aws_ec2_managed_prefix_list.cloudfront_origin.id
  app_port                         = var.app_port
  instance_type                    = var.instance_type
  ami_id                           = var.ami_id_override
  app_image_uri                    = var.app_image_uri
  app_config_s3_bucket_arn         = data.aws_s3_bucket.app_config.arn
  app_config_s3_prefix             = local.config_prefix
  app_config_s3_config_uri         = local.app_config_s3_uri
  secrets_arns                     = var.secrets_arns
  dynamodb_arns                    = var.dynamodb_arns
  log_retention_days               = var.log_retention_days
}

module "api_edge" {
  source = "./modules/api_edge"

  providers = {
    aws    = aws
    aws.us = aws.us
  }

  name_prefix        = local.name_prefix
  tags               = local.tags
  cloudflare_zone_id = data.cloudflare_zone.this.id
  api_domain_name    = var.api_domain_name
  certificate_domain = var.cf_dns_zone
  api_web_acl_name   = var.api_web_acl_name
  origin_domain_name = module.api_compute.instance_public_dns
  origin_http_port   = var.app_port
}

module "public_edge" {
  source = "./modules/public_edge"

  providers = {
    aws    = aws
    aws.us = aws.us
  }

  name_prefix                     = local.name_prefix
  tags                            = local.tags
  cloudflare_zone_id              = data.cloudflare_zone.this.id
  public_domain_name              = var.public_domain_name
  certificate_domain              = var.cf_dns_zone
  public_web_acl_name             = var.public_web_acl_name
  public_bucket_name              = var.public_bucket_name
  public_origin_access_control_id = var.public_origin_access_control_id
}

module "health_check" {
  source = "./modules/health_check"

  vpc_id                    = data.aws_vpc.default.id
  subnet_ids                = [local.selected_subnet_id]
  api_private_ip            = module.api_compute.instance_private_ip
  app_port                  = var.app_port
  name_prefix               = local.name_prefix
  tags                      = local.tags
  alert_email               = var.alert_email
  health_check_path         = var.health_check_path
  lambda_image_tag          = var.health_lambda_image_tag
  lambda_timeout_seconds    = var.health_lambda_timeout_seconds
  lambda_log_retention_days = var.lambda_log_retention_days
  ecr_image_retention_count = var.ecr_image_retention_count
}

resource "aws_vpc_security_group_ingress_rule" "api_from_health_lambda" {
  description                  = "Allow health-check Lambda to call API directly"
  security_group_id            = module.api_compute.security_group_id
  referenced_security_group_id = module.health_check.lambda_security_group_id
  ip_protocol                  = "tcp"
  from_port                    = var.app_port
  to_port                      = var.app_port
}

module "github_oidc_ecr" {
  source = "./modules/github_oidc_ecr"

  name_prefix          = local.name_prefix
  tags                 = local.tags
  github_repository    = var.github_repository
  github_oidc_branches = var.github_oidc_branches
  ecr_repository_arn   = module.health_check.ecr_repository_arn
}
