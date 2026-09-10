# p550_dual

Linux on physical core 1 and FreeRTOS on core 0.

Complete [P550 setup](../../docs/p550-setup.md), then run from this directory:

```sh
cargo build --locked
./tools/run.sh
```

Use `./tools/debug.sh` to upload without starting execution and attach GDB to
`localhost:3333`.

The Dockerfile builds the required FreeRTOS image:

```text
/opt/riscv/FreeRTOS/FreeRTOS/Demo/RISC-V_RV64_SiFive_HiFive-Premier-P550_GCC/build/RTOSDemo.bin
```
