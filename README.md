# Violet

## Overview
Violet is a hypervisor for the RISC-V architecture.
To adapt to various OS configurations and execution environments, Violet is structured as follows:
```
===================
proj/********* (binary crate)
|                  | 
| reference        V
|                  addon/***** (library crate)
|                  |
V                  V
violet(library crate)   
===================
```
The directories under [`proj`]("/workspaces/Violet/proj") are a group of binary crates, and by referencing the library crate [`violet`]("/workspaces/Violet/violet"), you can describe the desired VM configuration, as well as the processes during startup and traps.
[`addon`]("/workspaces/Violet/addon") is an extension feature of Violet, where you can describe custom functionalities while referencing [`violet`]("/workspaces/Violet/violet"). \
*Note: As a sample, it includes a prototype of an extension feature that runs an OS operating in M-mode as a guest OS (VS-mode).*

## Environment Setup

**Step1** \
Set up the environment using the Dockerfile located in the project (under `proj`). If you are using the Remote-Containers extension of VSCode, perform one of the following:
* Copy the Dockerfile to this directory.
* Specify the path to the Dockerfile in the "dockerFile" member of [`.devcontainer/devcontainer.json`]("/workspaces/Violet/.devcontainer/devcontainer.json").

*Note: By default, the Dockerfile that builds the all-in-one environment is placed in this directory. (However, it takes time to build.)* \
*Note: If you are not using Docker, please refer to the Dockerfile for environment setup.*

**Step2** \
Add the Rust riscv64 toolchain in the constructed environment:

```
% rustup target add riscv64imac-unknown-none-elf
```

## Build and Test Instructions
Please refer to the README.md of each project (under [`proj`]("/workspaces/Violet/proj")) for build and test instructions.
To run tests for the [`violet`]("/workspaces/Violet/violet") crate, execute [`cargo test`]("Go to definition") in the [`violet`]("/workspaces/Violet/violet") directory:

```
# cd violet
# cargo test
```

## Documentation Generation
To generate the documentation for the [`violet`]("/workspaces/Violet/violet") crate, execute `cargo doc` in the `violet` directory:
```
# cd violet
# cargo doc
```

## Contribution Guide
There are no specific guidelines yet.
For small changes, feel free to submit a pull request.
For proposals of significant changes, please create an issue or contact us.

## FAQ & Contact
mail: violetdev@googlegroups.com

## License
This software is released under the MIT License, see LICENSE.txt.
