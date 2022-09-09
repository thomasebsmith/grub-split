# GrubSplit
GrubSplit is an autosplitter for Hollow Knight that supports macOS and Linux.
It is written in Rust and licensed under the MIT License.

## Quick Start
```sh
$ git clone https://github.com/thomasebsmith/grub-split.git
$ cd grub-split
$ ./scripts/run.sh -- <pid> # (replace <pid> with a process ID to attach to)
```

## Roadmap
### v0.1 (in progress)
- Ability to analyze Mono memory on macOS
- Ability to log information about Hollow Knight game events on macOS

### v0.2
- Basic autosplitting functionality on macOS

### v0.3
- Improved timer integrations, possibily including LiveSplit integration

### v0.4
- Basic autosplitting functionality on Linux

### v0.5
- Bug hardening
- Compatibility with older Hollow Knight versions
