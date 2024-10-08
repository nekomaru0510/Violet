#!/bin/bash

LOG_FILE="/tmp/violet_test.log"
SEARCH_STRING="All tests passed"

function log_checker () {
    FILE_PATH=$1
    SEARCH_STRING=$2

    # Check if the file exists
    if [ ! -f "$FILE_PATH" ]; then
        echo "File not found: $FILE_PATH"
        return 1
    fi

    # Get the last 3 lines of the file and check for the search string
    if tail -n 3 "$FILE_PATH" | grep -q "$SEARCH_STRING"; then
        return 0
    else
        return 1
    fi
}

cd $(cd $(dirname $0); pwd)

# Run cargo build to ensure the project is built
cargo build

# Run the project with cargo run and redirect the output to a log file
timeout --foreground -s 9 10s cargo test > ${LOG_FILE} 2>&1
pkill -f qemu-system-riscv64

log_checker ${LOG_FILE} "${SEARCH_STRING}"

if [ $? -eq 0 ]; then
    echo "All tests passed"
    exit 0
else
    echo "Some tests failed"
    exit 1
fi
