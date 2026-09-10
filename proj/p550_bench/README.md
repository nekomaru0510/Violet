# p550_bench

Interrupt latency benchmark on P550.

Complete [P550 setup](../../docs/p550-setup.md), then run from the repository root:

```sh
cd benchmarks/interrupt-bench
cargo build --locked
cd ../../proj/p550_bench
cargo build --locked
./tools/run.sh
```

For debugging, use `./tools/debug.sh` and attach GDB to `localhost:3333`.
