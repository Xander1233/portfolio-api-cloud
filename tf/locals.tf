locals {
  name_prefix = var.project
  api_name    = "${var.project}-api"

  selected_subnet_id = coalesce(var.preferred_subnet_id, sort(data.aws_subnets.default_vpc.ids)[0])
  config_prefix      = trim(var.app_config_s3_prefix, "/")
  app_config_s3_uri  = "s3://${data.aws_s3_bucket.app_config.bucket}/${local.config_prefix}/${var.app_config_s3_object_key}"

  tags = {
    Project        = var.project
    Environment    = var.environment
    ManagedBy      = "terraform"
    awsApplication = var.aws_application
  }
}
