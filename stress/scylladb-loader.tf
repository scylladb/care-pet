####################################################################
# Creating 3 EC2 Instances:
####################################################################

# This block creates 3 EC2 instances based on the specified AMI, instance type, subnet ID, and security groups. 
# It also creates tags to identify the instances and sets timeouts for creating the instances.

resource "aws_instance" "instance" {
  count           = length(aws_subnet.public_subnet.*.id)
  ami             = var.ami_id
  instance_type   = var.instance_type
  subnet_id       = element(aws_subnet.public_subnet.*.id, count.index)
  security_groups = [aws_security_group.sg.id, ]
  key_name        = "care-pet-demo"
  tags = {
    "Name"      = "${var.custom_name}-Loader-${count.index}"
    "CreatedBy" = "care-pet-demo"
  }

  timeouts {
    create = "10m"
  }

  # This block provisions files to each instance. It copies three files from the current directory 
  # to the remote instance: thanos-attack-0.yml, cassandra-stress.service, and cassandra-stress-benchmark.service.

  provisioner "file" {
    source      = "thanos-attack-${count.index}.yml"
    destination = "/home/scyllaadm/care-pet-stress-1m.yaml"
  }
  provisioner "file" {
    source      = "cassandra-stress.service"
    destination = "/home/scyllaadm/cassandra-stress.service"
  }
  provisioner "file" {
    source      = "cassandra-stress-benchmark.service"
    destination = "/home/scyllaadm/cassandra-stress-benchmark.service"
  }

  # This block runs remote-exec commands on each instance. It stops the scylla-server, creates a start.sh script, 
  # creates a benchmark.sh script, sets permissions on the scripts, moves two files to /etc/systemd/system/, 
  # runs daemon-reload, and starts the cassandra-stress service.

  provisioner "remote-exec" {
    inline = [
      "sudo systemctl stop scylla-server |tee scylla.log",
      "echo '/usr/bin/cassandra-stress user profile=./care-pet-stress-1m.yaml n=${var.num_of_ops} cl=local_quorum no-warmup \"ops(insert=1)\" -rate threads=${var.num_threads} fixed=450000/s -mode native cql3 user=${var.scylla_user} password=${local.scylla_pass} -log file=populating.log  -node ${local.scylla_ips}' > start.sh",
      "echo '/usr/bin/cassandra-stress user profile=./care-pet-stress-1m.yaml duration=24h no-warmup cl=local_quorum \"ops(insert=4,simple1=2)\" -rate threads=${var.num_threads} fixed=${var.throttle} -mode native cql3 user=${var.scylla_user} password=${local.scylla_pass} -log file=benchmarking.log -node ${local.scylla_ips}' > benchmark.sh",
      "sudo chmod +x start.sh benchmark.sh",
      "sudo mv /home/scyllaadm/cassandra-stress.service /etc/systemd/system/cassandra-stress.service ",
      "sudo mv /home/scyllaadm/cassandra-stress-benchmark.service /etc/systemd/system/cassandra-stress-benchmark.service ", "sudo systemctl daemon-reload ",
      "sudo systemctl start cassandra-stress.service",
    ]
  }

  # This connection block sets up an SSH connection to each EC2 instance using the scyllaadm user and the private key 
  # The coalesce function is used to select the public IP address of ScyllaDB Nodes
  connection {
    type        = "ssh"
    user        = "scyllaadm"
    private_key = file("$ssh_private_key")
    host        = coalesce(self.public_ip, self.private_ip)
    agent       = true
  }

}


# Creating 3 Elastic IPs
resource "aws_eip" "eip" {
  count            = length(aws_instance.instance.*.id)               # Create an Elastic IP for each EC2 instance
  instance         = element(aws_instance.instance.*.id, count.index) # Associate the Elastic IP with the current EC2 instance
  public_ipv4_pool = "amazon"                                         # Use the Amazon pool for public IPv4 addresses
  vpc              = true                                             # Create a VPC Elastic IP address

  tags = { # Add tags to the Elastic IP resource
    "Name" = "${var.custom_name}-EIP-${count.index}"
  }
}

# Creating EIP association with EC2 Instances
resource "aws_eip_association" "eip_association" {
  count         = length(aws_eip.eip)                              # Associate each Elastic IP with an EC2 instance
  instance_id   = element(aws_instance.instance.*.id, count.index) # Associate the current Elastic IP with the current EC2 instance
  allocation_id = element(aws_eip.eip.*.id, count.index)           # Associate the current Elastic IP with the current allocation ID
}

