# p550_bench_vmmode

CSR and CLINT emulation benchmark on P550, with the guest running in VS-mode.

Complete [P550 setup](../../docs/p550-setup.md), then run from the repository root:

```sh
cd benchmarks/vmmode-bench
cargo build --locked
cd ../../proj/p550_bench_vmmode
cargo build --locked
./tools/run.sh
```

For debugging, use `./tools/debug.sh` and attach GDB to `localhost:3333`.

## Read results

The guest reserves 256 MiB for its sample buffers and runtime. UART output prints
GDB dump commands with guest addresses. Add `0x40000000` to both addresses when
reading through the board debugger, since guest `0x80000000` maps to host
`0xc0000000`. Use the addresses printed by the current build.
