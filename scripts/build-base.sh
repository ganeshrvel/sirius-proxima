#!/bin/bash

. ./env.config

readonly TARGET_HOST=$SSH_ADDRESS
readonly TARGET_PATH="/home/pi/marx"
readonly TARGET_ARCH="arm-unknown-linux-musleabi"

readonly EXEC_SOURCE_PATH="./target/${TARGET_ARCH}/release/marx"
readonly CONFIG_SOURCE_PATH="./Rocket.toml"
readonly SCRIPTS_SOURCE_PATH="./scripts"
