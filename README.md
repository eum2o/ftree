[![Build status](https://img.shields.io/github/actions/workflow/status/eum2o/ftree/rust.yml?branch=master)](https://github.com/eum2o/ftree/actions)
[![Latest version](https://img.shields.io/crates/v/e2o-ftree.svg)](https://crates.io/crates/e2o-ftree)
[![GitHub Release](https://img.shields.io/github/v/release/eum2o/ftree?label=download&link=https%3A%2F%2Fgithub.com%2Feum2o%2Fftree%2Freleases)](https://github.com/eum2o/ftree/releases)
![Crates.io License](https://img.shields.io/crates/l/e2o-ftree?color=%238b55d7)


# ftree

`ftree` is a simple command-line tool for visualizing directory structures. It creates a tree-like representation of the
file system that's easy to read and share.

## Example Output

```
./
├── top level folder/
│   ├── MyTpye1.java
│   ├── MyType2.java
│   ├── nested folder 1/
│   │   └── filewithoutext
│   ├── nested folder empty/
│   └── nested folder 2/
│       ├── file1.txt
│       └── file2.txt
├── readme.md
└── meta.data
```

## Installation

### Using Cargo

You can install `ftree` using [Cargo](https://github.com/rust-lang/cargo):

```
cargo install e2o-ftree
```

For more information about the crate, visit [https://crates.io/crates/e2o-ftree](https://crates.io/crates/e2o-ftree).

### Pre-built Executables

Alternatively, you can download pre-built executables for various platforms from the GitHub releases page:

[https://github.com/eum2o/ftree/releases](https://github.com/eum2o/ftree/releases)

## Usage

After installation, you can use the tool by running:

```
ftree [OPTIONS] [DIRECTORY]
```

### Arguments:
- `[DIRECTORY]`: The directory to visualize. If not specified, defaults to the current directory.

### Options:
- `--git`: Exclude git-related files and directories from the output.
- `-h, --help`: Print help information.

### Examples:
* `ftree`: Visualize the current directory
* `ftree /home/user`: Visualize a specific directory
* `ftree relative/path/to/folder`: Visualize a relative path
* `ftree --git`: Visualize the current directory, excluding git-related files
* `ftree --git /home/user`: Visualize a specific directory, excluding git-related files

## Found a Bug or Got a Feature Request?

If you encounter any problems or have any suggestions, please open an issue
on [ftree/issues](https://github.com/eum2o/ftree/issues).

## Contributing

Please refer to [CONTRIBUTING.md](CONTRIBUTING.md).