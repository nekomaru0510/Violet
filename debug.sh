#!/bin/bash

## ダサいので、このスクリプトは、cargoやrust-dbgなどで代用できるようになったら消す

function Debug()
{
    qemu-system-riscv32 -nographic -machine sifive_u -m 1M -kernel target/riscv32imac-unknown-none-elf/debug/violet -S -gdb tcp::1234
}

function Monitor()
{
    riscv32-unknown-elf-gdb target/riscv32imac-unknown-none-elf/debug/violet -x ./config/gdb/connect
}




if [ $# -eq 0 ]; then
    Debug
elif [ $1 == "-m" ]; then
    Monitor
fi
