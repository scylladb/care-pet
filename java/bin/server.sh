#!/usr/bin/env bash

set -euo pipefail

DIR=$(dirname -- "$0")
pushd "$DIR/.." > /dev/null

DEPENDENCIES="./target/app-1.0-SNAPSHOT.jar:$(cat ./target/dependencies)"

# java -classpath "$DEPENDENCIES" -agentlib:jdwp=transport=dt_socket,server=y,address=localhost:5005 com.carepet.server.App $@
java -classpath "$DEPENDENCIES" com.carepet.server.App $@
