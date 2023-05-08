ScyllaDB Cloud has a [Terraform provider](https://github.com/scylladb/terraform-provider-scylladbcloud) which means that you can spin up new
ScyllaDB Cloud clusters easily using Terraform. Follow the instructions below to
set up the care-pet sample application in a ScyllaDB Cloud environment using Terraform.

You'll set up Terraform to:
1. Create a new ScyllaDB Cloud cluster (you need a [ScyllaDB Cloud account](https://cloud.scylladb.com/account/sign-up))
1. Execute a CQL file that creates a new keyspace and tables for the care-pet project


## Get started
Go the the `terraform` directory and run `terraform init`
```bash
cd terraform/
terraform init
```

Now you are ready to apply the changes that are configure in the `main.tf` file:
```bash
terraform apply
```

You'll be asked to provide your computer's IP address, you can use
a [site like this](https://whatismyipaddress.com/) to get your public IPv4 address (Terraform is configured to add your address to the
ScyllaDB Cloud allowlist so you can connect to your database from your lacal machine):
```bash
var.ip_address
  The IP address of your computer that you use to connect to ScyllaDB Cloud

  Enter a value:
```

You'll also be asked to provide your ScyllaDB Cloud API token (more info [in docs](https://cloud.docs.scylladb.com/stable/api-docs/api-get-started.html)):
```bash
var.scylla_api_token
  Your own ScyllaDB Cloud API token

  Enter a value:
```

Finally, you'll be asked if you want to perform the actions configured in Terraform, just type `yes`:
```bash
Do you want to perform these actions?
  Terraform will perform the actions described above.
  Only 'yes' will be accepted to approve.

  Enter a value: yes

  scylladbcloud_cluster.care_pet: Creating...
```

Spinning up the cluster takes about ~10 minutes. While the process is underway, you can go to your
ScyllaDB [Cloud dashboard](https://cloud.scylladb.com/clusters/list) and verify that the cluster is getting set up:

![cluster setting up](./cloud_screen.png)


