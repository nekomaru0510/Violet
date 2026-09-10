#!/bin/bash

cd "$(cd "$(dirname "$0")"; pwd)"

openocd -f openocd_mcpu.cfg