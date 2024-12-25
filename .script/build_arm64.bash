#!/bin/bash

cd "$(dirname "$0")/.." || exit

# Build the ARM64 version of the app
# kevinmidtown is my dockerhub username, email is kindlychung@gmail.com
docker build -t kevinmidtown/simplehttp-rs:arm64 . 
docker push kevinmidtown/simplehttp-rs:arm64
