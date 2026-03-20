output "api_url" {
  description = "API URL."
  value       = "https://${var.api_domain_name}"
}

output "public_urls" {
  description = "Public frontend URLs."
  value       = ["https://${var.public_domain_name}", "https://www.${var.public_domain_name}"]
}

output "api_instance_id" {
  description = "API EC2 instance id."
  value       = module.api_compute.instance_id
}

output "health_lambda_name" {
  description = "Health check Lambda function name."
  value       = module.health_check.lambda_function_name
}

output "health_lambda_ecr_repository_name" {
  description = "ECR repository name for health-check Lambda image."
  value       = module.health_check.ecr_repository_name
}

output "health_lambda_ecr_repository_url" {
  description = "ECR repository URL for health-check Lambda image."
  value       = module.health_check.ecr_repository_url
}

output "github_oidc_ecr_role_arn" {
  description = "IAM role ARN for GitHub Actions OIDC ECR push."
  value       = module.github_oidc_ecr.role_arn
}

