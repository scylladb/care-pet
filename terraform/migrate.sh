#!/bin/bash

# Define default values for the parameters
username=""
password=""
hosts=""

# Parse the command-line arguments
while getopts u:p:h: option
do
    case "${option}"
    in
        u) username=${OPTARG};;
        p) password=${OPTARG};;
        h) hosts=${OPTARG};;
    esac
done

# Turn the hosts argument into an array
IFS=',' read -ra host_array <<< "$hosts"

# Get the first host from the array
host=${host_array[0]}


# Run the cqlsh command with all the arguments
cqlsh -u "$username" -p "$password" "$host" -f "migrate.cql"
