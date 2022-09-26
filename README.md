# GrubSplit
GrubSplit is an autosplitter for Hollow Knight that supports macOS and Linux.
It is written in Rust and is available under the MIT License.

See [LICENSE](./LICENSE) and [THIRD\_PARTY](./THIRD_PARTY) for license details.

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
- Memory reading performance improvements (caching, latency analysis, etc.)
- Basic autosplitting functionality on macOS

### v0.3
- Improved timer integrations, possibly including LiveSplit integration

### v0.4
- Basic autosplitting functionality on Linux

### v0.5
- Bug hardening
- Compatibility with older Hollow Knight versions
