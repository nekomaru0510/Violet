#!/bin/bash

## ダサいので、このスクリプトは、cargoやrust-dbgなどで代用できるようになったら消す

function Debug()
{
    qemu-system-riscv64 -nographic -machine sifive_u -m 1M -kernel target/riscv64gc-unknown-none-elf/debug/violet -S -gdb tcp::1234
}

function Monitor()
{
    riscv64-unknown-elf-gdb ../target/riscv64imac-unknown-none-elf/debug/violet -x ./gdb/connect
}

function MonitorLinux()
{
    riscv64-unknown-elf-gdb /opt/riscv/linux/vmlinux -x ./gdb/connect
}

function MonitorOpenSBI()
{
    riscv64-unknown-elf-gdb /opt/riscv/opensbi/build/platform/generic/firmware/fw_jump.elf -x ./gdb/connect
}

if [ $# -eq 0 ]; then
    Debug
elif [ $1 == "-m" ]; then
    Monitor
elif [ $1 == "-ml" ]; then
    MonitorLinux    
elif [ $1 == "-ms" ]; then
    MonitorOpenSBI  
fi
