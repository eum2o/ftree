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
ftree <path>
```

Replace `<path>` with the directory you want to visualize.

## Found a Bug or Got a Feature Request?

If you encounter any problems or have any suggestions, please open an issue
on [ftree/issues](https://github.com/eum2o/ftree/issues).