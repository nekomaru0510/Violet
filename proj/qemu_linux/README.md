# Single Linux Sample

## Overview
A standalone Linux is running.

| Category     | Details                     |
| ------------ | --------------------------- |
| Environment  | QEMU8.0.0 (virt)            |
| Architecture | RISC-V (64bit, H-extension) |
| GuestOS      | Linux5.17                   |

## Environment Setup
Please refer to the Dockerfile

## Build
```
% cargo build
```

## Run
```
% cargo run
```

## Debugging
Split the terminal into two and execute the following commands:

**Debug Execution**
```
% ./tools/debug.sh
```

**Monitoring with gdb**
```
% ./tools/monitor.sh
```