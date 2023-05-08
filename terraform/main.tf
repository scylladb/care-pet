terraform {
	required_providers {
		scylladbcloud = {
			source = "registry.terraform.io/scylladb/scylladbcloud"
		}
	}
}

provider "scylladbcloud" {
	token = trim(var.scylla_api_token, " ")
}

# Create a cluster on AWS cloud.
resource "scylladbcloud_cluster" "care_pet" {
    name       = "CarePet"
    cloud      = "AWS"
    region     = trim(var.region, " ")
    node_count = 3
    node_type  = "t3.micro"
    enable_vpc_peering = false
    enable_dns         = true
}

output "scylladbcloud_cluster_id" {
    value = scylladbcloud_cluster.care_pet.id
}

output "scylladbcloud_cluster_datacenter" {
    value = scylladbcloud_cluster.care_pet.datacenter
}

# Add a CIDR block to allowlist for the specified cluster.
resource "scylladbcloud_allowlist_rule" "example" {
  depends_on = [scylladbcloud_cluster.care_pet]
	cluster_id = scylladbcloud_cluster.care_pet.id
	cidr_block = "${trim(var.ip_address, " ")}/32" 
}

output "scylladbcloud_allowlist_rule_id" {
	value = scylladbcloud_allowlist_rule.example.rule_id
}

# Fetch credential information for cluster
data "scylladbcloud_cql_auth" "cql_auth" {
  depends_on = [scylladbcloud_cluster.care_pet]
	cluster_id = scylladbcloud_cluster.care_pet.id
}

resource "null_resource" "execfile" {
  depends_on = [scylladbcloud_cluster.care_pet]
  provisioner "local-exec" {
    command = "${path.module}/migrate.sh -u ${data.scylladbcloud_cql_auth.cql_auth.username} -p ${data.scylladbcloud_cql_auth.cql_auth.password} -h ${data.scylladbcloud_cql_auth.cql_auth.seeds} -f ${path.module}/migrate.cql"
  }
}



