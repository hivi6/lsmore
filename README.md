# lsmore

A basic ls tree command, to learn basics of rust.

## Usage

```bash
Usage:
    lsmore [FLAGS] <path>

About:
    This is a program that is like ls, but shows files in tree

FLAGS:
    --help          This screen
    --depth <size>  Depth of the tree
```

Example:

```bash
$ lsmore src
OUTPUT
└── src
    ├── main.rs
    └── .main.rs.swp
```

