# HiFive Premier P550 setup

| Project | Guests |
| --- | --- |
| [p550_linux](../proj/p550_linux/) | Single-vCPU Linux |
| [p550_linux_smp](../proj/p550_linux_smp/) | Four-vCPU Linux |
| [p550_freertos](../proj/p550_freertos/) | FreeRTOS |
| [p550_dual](../proj/p550_dual/) | Linux and FreeRTOS |
| [p550_bench](../proj/p550_bench/) | Interrupt latency benchmark |
| [p550_bench_vmmode](../proj/p550_bench_vmmode/) | CSR and CLINT emulation benchmark |

## 1. Build the development environment

Run from the repository root:

```sh
docker build -t violet-p550 .
docker run --rm -it --privileged --security-opt apparmor=unconfined \
  -v "$PWD:/workspaces/Violet" violet-p550 bash
```

The Dockerfile builds the tools, guest images and debug bootchain. U-Boot is
stored in `/opt/riscv/yocto/poky/dist/build/tmp-glibc/deploy/images/hifive-premier-p550/`.

## 2. Prepare Linux guests

Skip this step for FreeRTOS and benchmarks. Before installing the debug bootloader, boot the
board's normal Linux installation and install `device-tree-compiler`.
Locate the DTB used by the working GRUB entry and substitute its actual path:

```sh
GUEST_DTB=/boot/dtbs/6.X.X.X-premier/eswin/eic7700-hifive-premier-p550.dtb
dtc -I dtb -O dts -o /tmp/p550-violetvm.dts "$GUEST_DTB"
```

In `/tmp/p550-violetvm.dts`, remove the exact `h` extension from
`riscv,isa-extensions` in every `/cpus/cpu@X` node. `dtc` may render the list as
`"i\0m\0a...\0h..."`; remove the `h` item and its separator. If `riscv,isa` also
advertises H, remove it there too. Keep the other extensions and properties.
This edit applies to the guest DTB; Violet's host DTB still needs H-extension.

```sh
dtc -I dts -O dtb -o /tmp/p550-violetvm.dtb /tmp/p550-violetvm.dts
sudo install -m 644 /tmp/p550-violetvm.dtb "$(dirname "$GUEST_DTB")/p550-violetvm.dtb"
```

Add a `Violet VM` GRUB entry by copying the working entry into the distribution's
custom-entry configuration (usually `/etc/grub.d/40_custom`). Change only its
`devicetree` filename to `p550-violetvm.dtb`; preserve kernel, initrd, boot
arguments and GRUB's path convention. Run `sudo update-grub` on Ubuntu.
Keep the normal entry. Repeat the DTB preparation when changing kernel versions.

The guest kernel and root filesystem stay on the board's storage; the launcher
uploads Violet and U-Boot.

## 3. Install the debug bootloader

Follow the [debug bootloader instructions](../violet_debug_bootloader/README.md).
Connect the board's USB/JTAG/UART interface. The launchers use `/dev/ttyUSB2`
for the console (115200 baud) and `/dev/ttyUSB3` for management. Adjust the scripts
if your device numbering differs.

## 4. Build and launch

Inside the container, choose a project directory. For example:

```sh
cd proj/p550_linux
cargo build --locked
./tools/run.sh
```

For Linux, select `Violet VM` in guest GRUB. For debugging, use `./tools/debug.sh`;
it uploads the images and leaves the target halted, with GDB on `localhost:3333`.
