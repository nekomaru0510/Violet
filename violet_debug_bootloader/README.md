# Violet debug bootloader

The root Dockerfile builds the P550 bootchain with Violet's debug bootloader as
the OpenSBI payload. The generated image is stored inside the container:

```text
/opt/riscv/violet-debug-bootloader/violet_debug_bootloader_ddr5_secboot.bin
```

Prepare [Linux guest boot settings](../docs/p550-setup.md#2-prepare-linux-guests)
before replacing the board's firmware. Follow SiFive's
[P550 Image Update Procedure](https://www.sifive.com/document-file/hifive-premier-p550-image-update-procedure)
to write the generated bootchain to flash. On boot, the console should print
`Violet debug bootloader`.
