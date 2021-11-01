#!/bin/bash

. ./scripts/-base.sh

sshpass -p "$SSH_PASSWORD" ssh -t ${TARGET_HOST} "${TARGET_PATH}/scripts/-build-run-release.sh"
