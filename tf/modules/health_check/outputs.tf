output "lambda_function_name" {
  value = aws_lambda_function.health_check.function_name
}

output "ecr_repository_name" {
  value = aws_ecr_repository.health_lambda.name
}

output "ecr_repository_url" {
  value = aws_ecr_repository.health_lambda.repository_url
}

output "ecr_repository_arn" {
  value = aws_ecr_repository.health_lambda.arn
}

output "lambda_security_group_id" {
  value = aws_security_group.lambda.id
}
