variable "scylla_api_token" {
  type = string
  description = "Your own ScyllaDB Cloud API token"
  nullable = false
}

variable "region" {
  type = string
  default = "us-east-1"
  description = "AWS region to use for deployment (deafults to us-east-1)"
}

variable "ip_address" {
  type = string
  description = "The IP address of your computer that you use to connect to ScyllaDB Cloud"
  nullable = false
}