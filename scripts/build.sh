#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

. ./scripts/env.config

readonly TARGET_HOST=$SSH_ADDRESS
readonly TARGET_PATH=/home/pi/marx
readonly TARGET_ARCH=arm-unknown-linux-musleabi
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/marx

cargo build --release --target=${TARGET_ARCH}
sshpass -p "$SSH_PASSWORD" scp "${SOURCE_PATH}" "${TARGET_HOST}:${TARGET_PATH}"
sshpass -p "$SSH_PASSWORD" ssh -t ${TARGET_HOST} ${TARGET_PATH}/marx
