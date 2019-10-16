#!/bin/sh -e

BIO_VERSION=$(cat ../../VERSION)

docker build --no-cache --build-arg BIO_VERSION="$BIO_VERSION" -t biomesh/bio:"$BIO_VERSION" .
docker build --no-cache --build-arg BIO_VERSION="$BIO_VERSION" -t biomesh/default-studio-x86_64-linux:"$BIO_VERSION" ./default
