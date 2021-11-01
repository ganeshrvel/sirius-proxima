#!/bin/bash

. ./env.config
. ./scripts/build-base.sh

ROCKET_ENV=debug cargo build --target=${TARGET_ARCH}

sshpass -p "$SSH_PASSWORD" scp "${EXEC_SOURCE_PATH}" "${TARGET_HOST}:${TARGET_PATH}"
sshpass -p "$SSH_PASSWORD" scp "${CONFIG_SOURCE_PATH}" "${TARGET_HOST}:${TARGET_PATH}"
sshpass -p "$SSH_PASSWORD" scp -rp "${SCRIPTS_SOURCE_PATH}" "${TARGET_HOST}:${TARGET_PATH}"

sshpass -p "$SSH_PASSWORD" ssh -t ${TARGET_HOST} "${TARGET_PATH}/scripts/run-dev.sh"
