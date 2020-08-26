#!/usr/bin/env bash

set -euo pipefail

DIR=$(dirname -- "$0")
pushd "$DIR/.."

DEPENDENCIES="./target/app-1.0-SNAPSHOT.jar:$(cat ./target/dependencies)"

java -classpath $DEPENDENCIES com.carepet.Migrate