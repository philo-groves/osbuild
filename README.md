# osbuild
A general purpose builder for operating systems. This project aims to simplify the process of preparing, building, packing, and booting operating systems. The primary target of this project are operating systems which are created with C, C++, and/or Rust. However, the default implementation may be changed or extended as required.

In my previous incarnations of an operating system builder, there were core assumptions about the builder which were baked into every facet of the implementation:
- Bootloader of choice
- Language of choice
- Emulator of choice
- Reliance on external tools (e.g. isotools, git)
- Lots of "it only works on my computer"

In this design, I would like to challenge these assumptions to create a builder that is easily configurable and extensible. This will be an interesting research endeavor for myself, and a personal design challenge.

## Interface Design

This project provides a command line interface (CLI) as the primary interface. Through the `osbuild` CLI,  standardized set of commands can be used to build operating systems of all types.

### Example Commands

```
# Build with the configuration in the given YAML file
osbuild ./builder.yml

# Validate the properties in the given YAML file (no build)
osbuild ./builder.yml --validate

# Delete all resources related to the YAML file (no build)
osbuild ./builder.yml --clean
```

## Builder Configuration

To help the builder determine the proper build procedure and resources, the user of the builder must include a YAML file in their project.

### Required Properties

The following properties are required in the YAML file.:
- `name`: Name of the operating system
- `bootloader`: Options are "limine", "grub", or "custom"

### Example Configuration

```
# Required properties
name: Grovean
bootloader: limine

# Optional properties
emulator: qemu
phases:
  prepare:
    - command: sh ./scripts/before-prepare.sh
      when: before
    - command: sh ./scripts/after-prepare.sh
      when: after
  build:
    - command: sh ./scripts/before-build.sh
      when: before
    - command: sh ./scripts/after-build.sh
      when: after
  pack:
    - command: sh ./scripts/before-pack.sh
      when: before
    - command: sh ./scripts/after-pack.sh
      when: after
  boot:
    - command: sh ./scripts/before-boot-first.sh
      priority: 1
      when: before
    - command: sh ./scripts/before-boot-second.sh
      priority: 0
      when: before
    - command: sh ./scripts/after-boot.sh
      when: after
```