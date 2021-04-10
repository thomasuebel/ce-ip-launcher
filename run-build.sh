#!/usr/bin/bash

sudo docker container rm ce-ip-launcher-build
sudo docker rmi -f ce-ip-launcher-build
sudo docker build . -t ce-ip-launcher-build:latest
sudo docker create -v $(pwd):/home/rust/src --name ce-ip-launcher-build ce-ip-launcher-build
sudo docker start ce-ip-launcher-build -ai

