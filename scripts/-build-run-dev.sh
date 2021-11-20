#!/bin/bash

# cd to the root directory of proxima
SCRIPT=$(readlink -f "$0")
SCRIPTPATH=$(dirname "$SCRIPT")
cd "$SCRIPTPATH" || exit
cd ../

./scripts/-resets.sh

CHRONO_TZ_TIMEZONE_FILTER="(Asia/Kolkata|UTC)" /home/pi/.cargo/bin/cargo run
