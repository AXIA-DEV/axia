#!/bin/bash

# Run a development instance of the Ethereum to Axlib relay. Needs running
# Axlib and Ethereum nodes in order to work.

RUST_LOG=rpc=trace,bridge=trace ./target/debug/ethereum-poa-relay eth-to-sub
