#!/bin/bash

## ダサいので、このスクリプトは、cargoやrust-dbgなどで代用できるようになったら消す

function Debug()
{
    qemu-system-riscv64 -nographic -machine sifive_u -m 1M -kernel target/riscv64gc-unknown-none-elf/debug/violet -S -gdb tcp::1234
}

function Monitor()
{
    riscv64-unknown-elf-gdb target/riscv64gc-unknown-none-elf/debug/violet -x ./config/gdb/connect
}




if [ $# -eq 0 ]; then
    Debug
elif [ $1 == "-m" ]; then
    Monitor
fi
