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
# Required properties
name: Grovean
bootloader: limine
vm: qemu

# Optional properties

build:
  prepare:
    commands:
      - command: ./scripts/before-prepare.sh
        when: before
      - command: ./scripts/after-prepare.sh
        when: after
  compile:
    commands:
      - command: ./scripts/before-build.sh
        when: before
      - command: ./scripts/after-build.sh
        when: after
  pack:
    prevent-defaults: true
    commands:
      - command: ./scripts/before-pack.sh
        when: before
      - command: ./scripts/after-pack.sh
        when: after
  boot:
    commands:
      - command: ./scripts/before-boot-first.sh
        priority: 1
        when: before
      - command: ./scripts/before-boot-second.sh
        priority: 0
        when: before
      - command: ./scripts/after-shutdown.sh
        when: after
```

## Builder Design

Each build follows a series of phases to completion. 

### Phase 1: Prepare

Perform any actions which are required before the source code build operation.

### Phase 2: Compile

Perform the source code build (compile) operation.

If a Makefile is detected within the project, Make will be executed. If a Rust project without a Makefile is detected, Cargo will be executed.

### Phase 3: Pack

Perform the packing procedure for compiled source code and related OS assets into an executable format (e.g. ISO).

### Phase 4: Run (optional)

Launch the operating system in a virtual machine.

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