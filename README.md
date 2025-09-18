# osbuild
A general purpose builder for operating systems. This project aims to simplify the process of preparing, building, packing, and booting operating systems. The primary target of this project are operating systems which are created with C, C++, and/or Rust. However, the default implementation may be changed or extended as required.

In my previous incarnations of an operating system builder, there were core assumptions about the builder which were baked into every facet of the implementation:
- Bootloader of choice
- Programming language of choice
- Virtual machine of choice
- Reliance on other installed tools (e.g. isotools, git)
- Lots of "it only works on my computer"

In this design, I would like to challenge these assumptions to create a builder that is easily configurable and extensible. This will be an interesting research endeavor for myself, and a personal design challenge.

## Requirements

- Docker: This is the only requirement beyond the `osbuild` itself. Most build processes use Docker to prevent the need for additional installed dependencies and to prevent cases of "it only works on my computer"

## Interface Design

This project provides a command line interface (CLI) as the primary interface. Through the `osbuild` CLI,  standardized set of commands can be used to build operating systems of all types.

### Example Commands

```
# Build with the configuration in the given YAML file
osbuild ./os-compose.yml

# Validate the properties in the given YAML file (no build)
osbuild validate ./os-compose.yml

# Delete all resources related to the builder (no build)
osbuild clean ./os-compose.yml

# Display helpful information about command options
osbuild --help
```

## Configuration File

To help the builder determine the proper build procedure and resources, the user of the builder must include a YAML file in their project.

### Required Properties

The following properties are required in the YAML file.:
- `name`: Name of the operating system
- `bootloader`: Options are "limine", "grub", or "none"
    - Note: If you choose "none", you may still provide another bootloader through custom phase commands; recommended after the Build phase, or before the Pack phase.
- `vm`: Options are "qemu", "kvm", "virtualbox", or "none"
    - Note: If you choose "none", you may still provide another virtual machine thourhg custom phase commands; recommended before the Boot phase.

### Example Configuration

```
---
# Required fields
name: Grovean
bootloader: limine
vm: qemu

# Optional fields
build:
  prepare:
    prevent-defaults:
      - parse-runner-arguments
    actions:
      - commands: /scripts/before-prepare.sh
        triggers:
          - what: phase
            when: before
      - commands: /scripts/after-prepare.sh
        triggers:
          - what: parse-configuration-file
            when: after
  compile:
    actions:
      - commands: /scripts/before-build.sh
        triggers:
          - what: phase
            when: before
      - commands: /scripts/after-build.sh
        triggers:
          - what: phase
            when: after
  pack:
    prevent-defaults: all
    actions:
      - commands:
          - /scripts/before-pack-1.sh
          - /scripts/before-pack-2.sh
        triggers:
          - what: phase
            when: before
      - commands: /scripts/after-pack.sh
        triggers:
          - what: phase
            when: after
  boot:
    actions:
      - commands: /scripts/before-boot-first.sh
        priority: 1
        triggers:
          - what: phase
            when: before
      - commands: /scripts/before-boot-second.sh
        priority: 0
        triggers:
          - what: phase
            when: before
      - commands: /scripts/after-shutdown.sh
        triggers:
          - what: phase
            when: after

```

## Builder Design

Each build follows a series of phases to completion. Every phase contains a series of default behaviors; any of these behaviors may be disabled individually in the project configuration file.

### Phase 1: Prepare

Perform any actions which are required before the source code build operation.

#### Default Behavior

1. `parse-configuration-file`: Validate and read the configuration file that is supplied to the builder.
2. `parse-runner-arguments`: Validate and read the arguments which are provided by Cargo-based runners.

### Phase 2: Compile

Perform the source code build (compile) operation.

#### Default Behavior

1. `compile-source-code`: Build and compile the source code for the project. By default, the build procedure supports autodetection for Cargo and Make projects. 

### Phase 3: Pack

Perform the packing procedure for compiled source code and related OS assets into a bootable image format.

#### Default Behavior

1. `attach-bootloader`: Attach the configured bootloader to the operating system resources.
2. `attach-filesystem`: Optionally executes if enabled and configured. Attach one or more directories as a filesystem.
3. `create-bootable-image`: Create a bootable image from the compiled source code, bootloader, and other resources (e.g. static files). By default, the image is generated in ISO format.

### Phase 4: Run (optional)

Launch the operating system in a virtual machine.

#### Default Behavior

1. `run-virtual-machine`: Run the bootable image in the configured virtual machine. By default, all supported virtual machines run within Docker containers, and will not need to be installed on your system.

## Feature Progress

- ❌ = Not Started
- ⏳ = In Progress
- ✅ = Done

### Features
- ❌ Command Line Interface (CLI)
- ❌ Configuration File Parsing
- ⏳ Phased Build Procedure
- ❌ Build Automation Tools
    - ❌ Cargo (Rust-only)
    - ❌ Make (C, C++, Rust)
- ❌ Bootloaders
    - ❌ Limine
    - ❌ Grub
- ❌ Virtual Machines
    - ❌ QEMU
    - ❌ KVM
    - ❌ VirtualBox
- ❌ CPU Architectures
    - ❌ x86
    - ❌ x64
    - ❌ arm32
    - ❌ arm64