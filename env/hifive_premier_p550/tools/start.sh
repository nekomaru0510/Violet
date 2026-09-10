#!/bin/bash

cd "$(cd "$(dirname "$0")"; pwd)"

pkill openocd
while true; do
  openocd -f openocd_mcpu.cfg
  # sleep 1 # 実行間隔（秒）を設定（任意）
done
