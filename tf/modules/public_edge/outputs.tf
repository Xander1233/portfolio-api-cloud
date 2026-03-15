output "distribution_id" {
  value = aws_cloudfront_distribution.public.id
}

output "distribution_domain_name" {
  value = aws_cloudfront_distribution.public.domain_name
}
