# p550_linux_smp

Four-vCPU Linux via U-Boot on physical cores 0–3.

Complete [P550 setup](../../docs/p550-setup.md), then run from this directory:

```sh
cargo build --locked
./tools/run.sh
```

Use `./tools/debug.sh` to upload without starting execution and attach GDB to
`localhost:3333`.
