#!/bin/bash

# cd to the root directory of proxima
SCRIPT=$(readlink -f "$0")
SCRIPTPATH=$(dirname "$SCRIPT")
cd "$SCRIPTPATH" || exit
cd ../


./scripts/-resets.sh

/home/pi/.cargo/bin/cargo build --release
ROCKET_PROFILE=release ./target/release/proxima
