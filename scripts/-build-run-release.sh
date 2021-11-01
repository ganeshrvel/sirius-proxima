#!/bin/bash

# cd to the root directory of marx
SCRIPT=$(readlink -f "$0")
SCRIPTPATH=$(dirname "$SCRIPT")
cd $SCRIPTPATH
cd ../


./scripts/-resets.sh

/home/pi/.cargo/bin/cargo build --release
ROCKET_PROFILE=release ./target/release/marx
