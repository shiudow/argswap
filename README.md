# argswap

A simple and flexible command-line utility to reorder, swap, or drop arguments before executing a target command.

## Installation

You can install `argswap` directly from your GitHub repository using Cargo:

```bash
cargo install --git https://github.com
```

## Usage

Specify the modifications using options, followed by `--` and the target command.

```bash
argswap [options] -- <command> [arguments...]
```

### Options

* `-i`, `--input <indices>`: Target argument indices to move (0-indexed). Supporting ranges (e.g., `0-2` or `2-0`).
* `-o`, `--output <indices>`: Destination indices for the moved arguments.
* `-d`, `--drop <indices>`: Indices of arguments to remove before execution.
* `-s`, `--swap <indices>`: Indices to swap with their adjacent next argument (e.g., `-s 0` swaps index 0 and 1).

### Examples

**1. Reorder arguments (`-i`, `-o`)**
```bash
# 0:echo, 1:foo, 2:hoo -> Swaps 1st and 2nd arguments
\$ argswap -i 1,2 -o 2,1 -- echo foo hoo
hoo foo
```

**2. Drop specific arguments (`-d`)**
```bash
# 0:echo, 1:foo, 2:hoo, 3:bar -> Drops 'foo' (1) and 'bar' (3)
\$ argswap -d 1,3 -- echo foo hoo bar
hoo
```

**3. Swap adjacent arguments (`-s`)**
```bash
# 0:echo, 1:foo, 2:hoo, 3:bar, 4:baz
# Swaps 1st with 2nd (foo <-> hoo), and 3rd with 4th (bar <-> baz)
# Note: 0 (echo) remains unchanged
\$ argswap -s 1,3 -- echo foo hoo bar baz
echo hoo foo baz bar
```

## Versioning

Semantic Versioning 2.0
