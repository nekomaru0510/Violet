# Single FreeRTOS Sample

## Overview
A standalone FreeRTOS is running.
Utilizing the vmmode addon, FreeRTOS running on the VM.
(Note: As this is a prototype, some M-mode operations are not supported.)

| Category     | Details                     |
| ------------ | --------------------------- |
| Environment  | QEMU8.0.0 (virt)            |
| Architecture | RISC-V (64bit, H-extension) |
| GuestOS      | FreeRTOS                    |

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
% ./debug.sh
```

**Monitoring with gdb**
```
% ./monitor.sh
```
