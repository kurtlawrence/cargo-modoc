[![Build Status](https://travis-ci.com/kurtlawrence/cargo-modoc.svg?branch=master)](https://travis-ci.com/kurtlawrence/cargo-modoc)
[![Latest Version](https://img.shields.io/crates/v/cargo-modoc.svg)](https://crates.io/crates/cargo-modoc)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/cargo-modoc)
[![codecov](https://codecov.io/gh/kurtlawrence/cargo-modoc/branch/master/graph/badge.svg)](https://codecov.io/gh/kurtlawrence/cargo-modoc)

Generate Rust module documentation from markdown files.

Apply module documentation to a source file from a markdown file. 
This binary takes a markdown file and prepends a source file with the
module documentation comments (`//!`). There is a configuration file at
the root directory (`modoc.config`) which defines which markdown files
are to prepend which source files, in a single input multiple output fashion.

To get started, use cargo to install the binary.

```sh
cargo install cargo-modoc
```

A configuration file needs to be set up.

```toml
# Commends are allowed
"README.md" = [ "src/main.rs", "src/lib.rs" ]
```

> A single markdown file can apply to multiple source files.

Run the binary to write the markdown contents to the source files.

```sh
cargo modoc
```

The binary will read the contents of the markdown file and prepend 
the source file with the line comments (`//!`). 
If there are any lines in the source file which start with `//!` 
then these lines are not included (hence the documentation is _overwritten_).
