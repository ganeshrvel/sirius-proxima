#!/bin/bash

CHRONO_TZ_TIMEZONE_FILTER="(Asia/Kolkata|UTC)" cargo build --release && ./target/release/sirius-proxima
