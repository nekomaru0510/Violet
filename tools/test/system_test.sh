#!/bin/bash

LOG_FILE="/tmp/violet_test.log"
SEARCH_STRING="All tests passed"

function test_runner () {
    PROJ_PATH=$1
    ./${PROJ_PATH}/tools/test.sh
    if [ $? -ne 0 ]; then
        exit 1
    fi
}

cd $(cd $(dirname $0); pwd)/../../proj

# Run tests from projects
test_runner "qemu_linux"
test_runner "qemu_freertos"
# test_runner "qemu_system_test"

if [ $? -eq 0 ]; then
    echo "All tests passed"
    exit 0
else
    echo "Some tests failed"
    exit 1
fi
