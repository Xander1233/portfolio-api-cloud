output "api_url" {
  value = "https://${var.api_domain_name}"
}

output "public_url" {
  value = "https://${var.public_domain_name}, https://www.${var.public_domain_name}; Bucket: ${data.aws_s3_bucket.public.bucket}"
}

output "instance_id" {
  value = aws_instance.api.id
}
