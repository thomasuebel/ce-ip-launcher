#!/usr/bin/bash
set -o errexit
set -o nounset

source projectname

docker container rm "${RUST_PROJECT_NAME}-build"
docker rmi -f "${RUST_PROJECT_NAME}-build"
docker build . -t "${RUST_PROJECT_NAME}-build":latest
docker create -e WIN_THEME=true -v $(pwd):/home/rust/src --name "${RUST_PROJECT_NAME}-build" "${RUST_PROJECT_NAME}-build"
docker start "${RUST_PROJECT_NAME}-build" -ai
