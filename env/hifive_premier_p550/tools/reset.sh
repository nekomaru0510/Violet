#!/bin/bash

LOG_FILE="/tmp/p550_reset.log"

cd "$(cd "$(dirname "$0")"; pwd)"

echo "Resetting HiFive Premier P550"

pkill openocd
stty -F /dev/ttyUSB3 115200
echo "reboot cold" > /dev/ttyUSB3


# if arg1 == "debug" then wait for "Violet debug bootloader" message
if [ "$1" == "debug" ]; then
    lsof -t /dev/ttyUSB2 | xargs --no-run-if-empty kill -9
    stty -F /dev/ttyUSB2 115200
    echo "Waiting for Violet debug bootloader..."
    ready=false
    while IFS= read -r -t 60 line; do
        printf '%s\n' "$line" >> "$LOG_FILE"
        if [[ "$line" == *"Violet debug bootloader"* ]]; then
            ready=true
            break
        fi
    done < /dev/ttyUSB2
    if [[ "$ready" != true ]]; then
        echo "Timed out waiting for the debug bootloader" >&2
        exit 1
    fi

    echo "Violet debug bootloader started"
fi

