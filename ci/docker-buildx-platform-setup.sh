#! /bin/bash

case "$1" in
  "linux/amd64")
    echo x86_64-unknown-linux-gnu > /rust_target.txt
    ;;
  "linux/arm64")
    apt-get update && apt-get install -y gcc-aarch64-linux-gnu g++-aarch64-linux-gnu libc6-dev-arm64-cross
    echo aarch64-unknown-linux-gnu > /rust_target.txt
    ;;
  "linux/arm/v6")
    apt-get update && apt-get install -y gcc-arm-linux-gnueabihf libc6-dev-armel-cross
    echo arm-unknown-linux-gnueabihf > /rust_target.txt
    ;;
  "linux/arm/v7")
    apt-get update && apt-get install -y gcc-arm-linux-gnueabihf libc6-dev-armel-cross
    echo armv7-unknown-linux-gnueabihf > /rust_target.txt
    ;;
  *)
    exit 1
    ;;
esac
