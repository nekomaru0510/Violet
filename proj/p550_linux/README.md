# p550_linux

Single-vCPU Linux via U-Boot on physical core 0.

Complete [P550 setup](../../docs/p550-setup.md), then run from this directory:

```sh
cargo build --locked
./tools/run.sh
```

Use `./tools/debug.sh` to upload without starting execution and attach GDB to
`localhost:3333`.
