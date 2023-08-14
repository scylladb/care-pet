# 1 million ops/sec demo
This demo showcases how ScyllaDB can handle 1 million operations per second. Follow the instructions below and see it yourself!

## Infrastructure elements
* ScyllaDB Cloud cluster (for hosting the database in the cloud)
* Amazon Web Services (AWS) EC2 instance (for hosting the machines that will make the requests toward the database)

## Requirements
* AWS account and CLI credentials (more information on acquiring the credentials [here](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html) and [here](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-configure.html))
* ScyllaDB Cloud API token (get your API token [here](https://cloud.docs.scylladb.com/stable/api-docs/api-get-started.html))
* Terraform installed on your machine (installation instructions [here](https://developer.hashicorp.com/terraform/tutorials/aws-get-started/install-cli))

## Get started
Clone this repository - if you haven't already - and navigate to the `stress` folder:
```bash
git clone https://github.com/scylladb/care-pet.git
cd stress/
```

In this repository you can find the Terraform configuration files needed to set up the demo. The configuration will create the following resources in AWS and ScyllaDB Cloud:
* ScyllaDB Cloud cluster
* VPC
* Subnets
* Security groups
* EC2 instance

Make sure that you have sufficient AWS permissions to create these items.

Start setting up infrastructure with Terraform:
```bash
terraform init
terraform plan
terraform apply
```

Setting up the infrastructure takes 10+ minutes. 

After completion, SSH (use your private key file and the proper EC2 instance address) into the loader instance and start making requests to the database:
```bash
ssh -i "private_key.pem" scyllaadm@ec2-11-11-111-11.eu-north-1.compute.amazonaws.com
```

Run the script:
```bash
sudo systemctl start cassandra-stress-benchmark
```

At this point, your ScyllaDB Cloud cluster will start getting requests. Check the results on the Grafana dashboard in ScyllaDB Cloud:
<add screenshot>