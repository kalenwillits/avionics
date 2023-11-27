#!/bin/bash

cargo build --release;
scp target/release/avionics deck@192.168.1.24:~/Avionics/avionics;
