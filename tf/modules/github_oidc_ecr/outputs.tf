output "role_arn" {
  value = aws_iam_role.github_ecr_push.arn
}

output "role_name" {
  value = aws_iam_role.github_ecr_push.name
}
