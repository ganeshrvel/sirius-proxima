#!/bin/bash

# cd to the root directory of marx
SCRIPT=$(readlink -f "$0")
SCRIPTPATH=$(dirname "$SCRIPT")
cd $SCRIPTPATH
cd ../


./scripts/-resets.sh

ROCKET_CONFIG=/home/pi/marx-src/Rocket.toml ROCKET_PROFILE=debug /home/pi/.cargo/bin/cargo run
