#!/bin/bash

RISCV_PATH="/opt/riscv"
PROJ_PATH="${RISCV}/FreeRTOS/FreeRTOS/Demo/RISC-V-Qemu-virt64-test_GCC/"

cp -r ${RISCV}/FreeRTOS/FreeRTOS/Demo/RISC-V-Qemu-virt64_GCC ${RISCV}/FreeRTOS/FreeRTOS/Demo/RISC-V-Qemu-virt64-test_GCC
rm ${RISCV}/FreeRTOS/FreeRTOS/Demo/RISC-V-Qemu-virt64-test_GCC/*.c
##rm ${RISCV}/FreeRTOS/FreeRTOS/Demo/RISC-V-Qemu-virt64-test_GCC/*.h

cp /workspaces/Violet/tools/vht/src/*.c ${PROJ_PATH}
cp /workspaces/Violet/tools/vht/platform/freertos/*.c ${PROJ_PATH}
cp /workspaces/Violet/tools/vht/include/*.h ${PROJ_PATH}
cp ${RISCV}/FreeRTOS/FreeRTOS/Demo/RISC-V-Qemu-virt64_GCC/FreeRTOSConfig.h ${PROJ_PATH}

sed -i -e "s/main.c main_blinky.c riscv-virt.c ns16550.c/main.c vht_cmds.c vht_shell.c vht_os.c/g" ${PROJ_PATH}/Makefile

cd ${PROJ_PATH}
make clean && \
make PICOLIBC=1 DEBUG=1 && \
riscv64-unknown-elf-objcopy -O binary build/RTOSDemo.axf build/RTOSDemo.bin
