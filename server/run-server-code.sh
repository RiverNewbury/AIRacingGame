#!/bin/sh
#
# Launches the server and gives it 'test.py' to run. By default, rocket seems
# to use port 8000, so we're using that here. This script might break if that
# changes

set -e

cargo build
target/debug/ai-racing-server &

server_pid=$!
trap "kill $server_pid" EXIT

# Because the server sometimes takes a while to start up, we'll sleep for a bit
# to give it a chance.
sleep 0.5

curl -X POST --data-binary '@test.py' "http://localhost:8000/run/test_user"
