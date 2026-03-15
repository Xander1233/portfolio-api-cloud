output "instance_id" {
  value = aws_instance.api.id
}

output "instance_public_dns" {
  value = aws_instance.api.public_dns
}

output "instance_private_ip" {
  value = aws_instance.api.private_ip
}

output "security_group_id" {
  value = aws_security_group.api.id
}
