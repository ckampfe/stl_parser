# STL (STereo Lithography) File Complexity Analyzer

This command-line tool accepts an STL file (ASCII or binary) and outputs measures of its complexity.

- [Installation](#Installation)
- [Usage](#Usage)
- [Assumptions](#Assumptions)
- [Design](#Design)
- [Enhancements](#Enhancements)

## Installation

You must have [Rust](https://www.rust-lang.org/) installed, as this project is written in Rust.  This was developed with stable Rust on version 1.37.0.

When you have that, run:

```bash
make stlp
```

## Usage

```
stlp 1.0.0
Jim Berlage <james.berlage@gmail.com>
Command line utility to get the complexity of STL files

USAGE:
    stlp [FLAGS] --file <FILE>

FLAGS:
    -a, --ascii      Signals that the file is ascii, not binary
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <FILE>    The path to the file to be analyzed
```

There are some tests in the `tests` directory.  All are ASCII, except for `tests/knot.stl`.

You should see something like:

```bash
$ ./stlp --file tests/knot.stl
Number of Triangles: 2000
Surface Area: 994939.7500
Bounding Box: {x: -86.57634, y: -92.81212, z: -64.507576}, {x: -86.57634, y: 92.81212, z: -64.507576}, {x: -86.57634, y: -92.81212, z: 64.507576}, {x: -86.57634, y: 92.81212, z: 64.507576}, {x: 86.57634, y: -92.81212, z: -64.507576}, {x: 86.57634, y: 92.81212, z: -64.507576}, {x: 86.57634, y: -92.81212, z: 64.507576}, {x: 86.57634, y: 92.81212, z: 64.507576}
$
$ ./stlp --file tests/magnolia.stl --ascii
Number of Triangles: 1247
Surface Area: 255595.2969
Bounding Box: {x: -78.0177, y: -39.8119, z: -77.6679}, {x: -78.0177, y: 9.2315, z: -77.6679}, {x: -78.0177, y: -39.8119, z: 77.4179}, {x: -78.0177, y: 9.2315, z: 77.4179}, {x: 83.9627, y: -39.8119, z: -77.6679}, {x: 83.9627, y: 9.2315, z: -77.6679}, {x: 83.9627, y: -39.8119, z: 77.4179}, {x: 83.9627, y: 9.2315, z: 77.4179}
```

## Assumptions

This project assumes that triangles do not overlap when measuring complexity, to keep the surface area calculation simple.  It may not be a good fit for models with directly overlapping facets.

The ASCII file format is assumed to allow for any number of whitespace characters separating items, which makes writing a streaming parser difficult.  This means that ASCII files are fully loaded into memory to analyze.  For large files, try to use th binary STL format, as that is buffered in ~5 MiB chunks.

[`nom`](https://github.com/Geal/nom) is used to parse files, due to it's ability to easily work with text and binary formats.  However, UTF-8 parsing is not supported.  Hopefully the ASCII part of the name indicates that UTF-8 parsing may not work, but this tool will choke on files that include non-ASCII characters.