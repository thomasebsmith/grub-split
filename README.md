# GrubSplit
GrubSplit is an autosplitter for Hollow Knight that supports macOS and Linux.
It is written in Rust and is available under the MIT License.

GrubSplit is a work in progress and is not functional yet.

See [LICENSE](./LICENSE) and [THIRD\_PARTY](./THIRD_PARTY) for license details.

## Quick Start
```sh
$ git clone https://github.com/thomasebsmith/grub-split.git
$ cd grub-split
# Replace <pid> with the process ID of your Hollow Knight game:
$ ./scripts/run.sh -- <pid>
```

## Roadmap
### v0.1 (in progress)
- Ability to analyze Mono images, classes, and objects on macOS
- Ability to log information about Hollow Knight game events on macOS with the
  latest version of Hollow Knight

### v0.2
- Memory reading performance improvements (caching, latency analysis, etc.)
- Basic autosplitting functionality on macOS

### v0.3
- Improved timer integrations, possibly including LiveSplit integration

### v0.4
- Basic autosplitting functionality on Linux

### v0.5
- Bug hardening
- Compatibility with older Hollow Knight versions (including 1.2.2.1 and
  1.4.3.2)
