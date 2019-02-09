# procgrep

Rust command line tool to grep for processes.

This is a work in progress; it currently only works on macOS and is lacking many features.

## Installation

```
git clone https://github.com/RobbieClarken/procgrep.git
cargo install --path procgrep
```

## Usage

```
$ procgrep vim
84902
82219
67524
$ procgrep --tty /dev/ttys003
87976
61281
```
