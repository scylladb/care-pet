
# Scylla Cloud API token
variable "scylla_cloud_token" {
  description = "Scylla Cloud API token"
  type        = string
  default     = "<add token>"
}

# Environment name
variable "custom_name" {
  description = "Name for the Scylla Cloud environment"
  type        = string
  default     = "care-pet-demo"
}

# Virtual Private Cloud (VPC) IP range
variable "custom_vpc" {
  description = "CIDR block for the VPC"
  type        = string
  default     = "10.0.0.0/16"
}

# EC2 instance tenancy
variable "instance_tenancy" {
  description = "EC2 instance tenancy, default or dedicated"
  type        = string
  default     = "default"
}

# Amazon Machine Image (AMI) ID
variable "ami_id" {
  description = "AMI ID for the EC2 instance"
  type        = string
  default     = "ami-0a23f9b62c17c53fe"
}

# EC2 instance type
variable "instance_type" {
  description = "Type of the EC2 instance"
  type        = string
  default     = "i4i.xlarge"
}

# SSH private key for EC2 instance access
variable "ssh_private_key" {
  description = "SSH private key for EC2 instance access"
  type        = string
  default     = "<add path>"
}

# Number of Scylla Cloud instances to create
variable "scylla_node_count" {
  description = "Number of Scylla Cloud instances to create"
  type        = string
  default     = "3"
}

# Scylla Cloud instance type
variable "scylla_node_type" {
  description = "Type of Scylla Cloud instance"
  type        = string
  default     = "i3.xlarge"
}

# Total number of operations to run
variable "num_of_ops" {
  description = "Total number of operations to run"
  type        = string
  default     = "100M"
}

# Number of threads for the Cassandra stress tool
variable "num_threads" {
  description = "Number of threads for the Cassandra stress tool"
  type        = string
  default     = "256"
}

# Throttling for the Cassandra stress tool
variable "throttle" {
  description = "Throttling for the Cassandra stress tool (in ops/sec)"
  type        = string
  default     = "900000/s "
}

# Scylla Cloud user
variable "scylla_user" {
  description = "Scylla Cloud user"
  type        = string
  default     = "scylla"
}

locals {
  scylla_ips  = (join(",", [for s in scylladbcloud_cluster.scylladbcloud.node_private_ips : format("%s", s)]))
  scylla_pass = data.scylladbcloud_cql_auth.scylla.password

}
