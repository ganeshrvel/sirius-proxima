#!/bin/bash

. ./env.config

# cd to the root directory of marx
SCRIPT=$(readlink -f "$0")
SCRIPTPATH=$(dirname "$SCRIPT")
cd $SCRIPTPATH
cd ../

ROCKET_PROFILE=debug ./marx

