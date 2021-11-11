#!/bin/bash

# cd to the root directory of proxima
SCRIPT=$(readlink -f "$0")
SCRIPTPATH=$(dirname "$SCRIPT")
cd "$SCRIPTPATH" || exit
cd ../


./scripts/-resets.sh

ROCKET_CONFIG=/home/pi/proxima-src/Rocket.toml ROCKET_PROFILE=debug /home/pi/.cargo/bin/cargo run
