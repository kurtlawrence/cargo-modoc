//! [![Build Status](https://travis-ci.com/kurtlawrence/cargo-modoc.svg?branch=master)](https://travis-ci.com/kurtlawrence/cargo-modoc)
//! [![Latest Version](https://img.shields.io/crates/v/cargo-modoc.svg)](https://crates.io/crates/cargo-modoc)
//! [![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/cargo-modoc)
//! [![codecov](https://codecov.io/gh/kurtlawrence/cargo-modoc/branch/master/graph/badge.svg)](https://codecov.io/gh/kurtlawrence/cargo-modoc)
//! 
//! Generate module documentation from markdown files.
//! 
//! Apply module documentation to a source file from a markdown file. This binary takes a markdown file and prepends a source file with the module documentation comments (`//!`). There is a configuration file at the root directory (`readme.config`) which defines which markdown files are to prepend which source files, in a single input multiple output fashion.
//! 
//! To get started, use cargo to install the binary.
//! 
//! ```sh
//! cargo install cargo-my-readme
//! ```
//! 
//! A configuration file needs to be set up.
//! 
//! ```text
//! "README.md" = [ "src/main.rs", "src/lib.rs" ]
//! ```
//! 
//! > A single markdown file can apply to multiple source files.
//! 
//! Run the binary to write the markdown contents to the source files.
//! 
//! ```sh
//! cargo my-readme
//! ```
//! 
//! The binary will read the contents of the markdown file and prepend the source file with the line comments (`//!`). If there are any lines in the source file which start with `//!` then these lines are not included (hence the documentation is _overwritten_).
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
	let config = parse_config(BufReader::new(
		fs::File::open("modoc.config")
			.expect("failed to find 'modoc.config' file in root directory"),
	));

	dbg!(&config);

	for (md, v) in config {
		for write_file in v {
			dbg!(format!("reading {}, writing {}", &md, &write_file));
			write_md_comments(&md, &write_file).expect(&format!(
				"failed to write md comments of '{}' into '{}'",
				md, write_file
			));
		}
	}
}

fn write_md_comments<P: AsRef<Path>, Q: AsRef<Path>>(md_file: P, write_file: Q) -> io::Result<()> {
	let string = {
		let md = fs::read_to_string(md_file)?;
		let rs = fs::read(&write_file)?;
		let md_rdr = BufReader::new(md.trim().as_bytes());
		let rs_rdr = BufReader::new(&rs[..]);

		let mut rs_wtr = String::new();
		for line in md_rdr.lines() {
			let line = line?;
			rs_wtr.push_str("//! ");
			rs_wtr.push_str(&line);
			rs_wtr.push('\n');
		}

		for line in rs_rdr.lines() {
			let line = line?;
			if !line.starts_with("//!") {
				rs_wtr.push_str(&line);
				rs_wtr.push('\n');
			}
		}

		rs_wtr
	};

	fs::write(write_file, string)
}

fn parse_config<R: BufRead>(config: R) -> Vec<(String, Vec<String>)> {
	config
		.lines()
		.map(|x| x.unwrap())
		.filter(|x| x.len() > 0)
		.map(|line| {
			let (f, s) = parse_line(&line);
			(
				f.to_string(),
				s.into_iter().map(|x| x.to_string()).collect(),
			)
		})
		.collect()
}

fn parse_line(line: &str) -> (&str, Vec<&str>) {
	let mut equals = line.split("=");
	let first = equals.next().expect("expecting stuff before the '='");
	let second = equals.next().expect("expecting stuff before the '='");
	assert_eq!(equals.next(), None, "expecting only one equals");

	let first = parse_string(first);
	let second = parse_array(second)
		.into_iter()
		.map(|x| parse_string(x))
		.collect();

	(first, second)
}

fn parse_string(string: &str) -> &str {
	let s = string.trim();
	assert_eq!(&s[..1], "\"", "expecting string in '\"'");
	assert_eq!(&s[s.len() - 1..], "\"", "expecting string in '\"'");
	&s[1..s.len() - 1]
}

fn parse_array(string: &str) -> Vec<&str> {
	let s = string.trim();
	assert_eq!(&s[..1], "[", "expecting '['");
	assert_eq!(&s[s.len() - 1..], "]", "expecting ']'");
	s[1..s.len() - 1].split(",").collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_string_test() {
		assert_eq!(parse_string(r#"   "hello"   "#), "hello");
	}

	#[test]
	fn parse_array_test() {
		assert_eq!(
			parse_array(r#"   [ "testings"   ,  " asdf "     ]   "#),
			[r#" "testings"   "#, r#"  " asdf "     "#]
		);
	}

	#[test]
	fn parse_line_test() {
		assert_eq!(
			parse_line(r#""test.md" = [ "one.rs", "two.rs" ] "#),
			("test.md", vec!["one.rs", "two.rs"])
		);
	}
}
