#!/bin/bash

# Run a development instance of the Alphanet Axlib bridge node.
# To override the default port just export WESTEND_PORT=9945
#
# Note: This script will not work out of the box with the bridges
# repo since it relies on a Axia binary.

WESTEND_PORT="${WESTEND_PORT:-9944}"

RUST_LOG=runtime=trace,runtime::bridge=trace \
./target/debug/axia --chain=alphanet-dev --alice --tmp \
    --rpc-cors=all --unsafe-rpc-external --unsafe-ws-external \
    --port 33033 --rpc-port 9933 --ws-port $WESTEND_PORT \
