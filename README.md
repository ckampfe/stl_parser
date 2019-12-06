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

There are some examples in the `examples` directory.  All are ASCII, except for `examples/knot.stl`.

You should see something like:

```bash
$ ./stlp --file examples/knot.stl
Number of Triangles: 2000
Surface Area: 994939.7500
Bounding Box: {x: -86.57634, y: -92.81212, z: -64.507576}, {x: -86.57634, y: 92.81212, z: -64.507576}, {x: -86.57634, y: -92.81212, z: 64.507576}, {x: -86.57634, y: 92.81212, z: 64.507576}, {x: 86.57634, y: -92.81212, z: -64.507576}, {x: 86.57634, y: 92.81212, z: -64.507576}, {x: 86.57634, y: -92.81212, z: 64.507576}, {x: 86.57634, y: 92.81212, z: 64.507576}
$
$ ./stlp --file examples/magnolia.stl --ascii
Number of Triangles: 1247
Surface Area: 255595.2969
Bounding Box: {x: -78.0177, y: -39.8119, z: -77.6679}, {x: -78.0177, y: 9.2315, z: -77.6679}, {x: -78.0177, y: -39.8119, z: 77.4179}, {x: -78.0177, y: 9.2315, z: 77.4179}, {x: 83.9627, y: -39.8119, z: -77.6679}, {x: 83.9627, y: 9.2315, z: -77.6679}, {x: 83.9627, y: -39.8119, z: 77.4179}, {x: 83.9627, y: 9.2315, z: 77.4179}
```

## Assumptions

This project assumes that triangles do not overlap when measuring complexity, to keep the surface area calculation simple.  It may not be a good fit for models with directly overlapping facets.

The ASCII file format is assumed to allow for any number of whitespace characters separating items, which makes writing a streaming parser difficult.  This means that ASCII files are fully loaded into memory to analyze.  For large files, try to use th binary STL format, as that is buffered in ~5 MiB chunks.

[`nom`](https://github.com/Geal/nom) is used to parse files, due to it's ability to easily work with text and binary formats.  However, UTF-8 parsing is not supported.  Hopefully the ASCII part of the name "ASCII STL file" indicates to users that UTF-8 parsing may not work, but it bears mentioning that this tool will choke on files that include non-ASCII characters.

## Design

The parser is designed to work best with the binary file format, simply because binary file formats can be much larger.  The binary file format reads the file in in chunks, and is generally less of a memory hog.  I entertained making the ASCII parser streaming as well, but to do that properly you would need to have a fixed size for vertices.  The current spec allows for any amount of whitespace in an ASCII STL file, and the same float can take up a variable amount of space ("1.0e+2" vs "100" vs "100.0").  So streaming chunks is infeasible unless you are prepared to write a lot of code around handling incomplete parser states.  Without knowing how common large ASCII STL files are relative to large binary STL files, I made a guess that large files are almost always in binary format.

Architecturally, there is a `parser::ascii` namespace and a `parser::binary` namespace.  Models are placed in their own file, and some shared concepts are in their own file as well (3D coordinates live in the `coordinate` namespace, for example).  Each parser exposes

```rust
solid_from_filepath(filepath: &str) -> nom::IResult<(), solid::Solid, parser::error::SolidError>
```

Which handles the core of taking a file and turning it into a representation we can use.

Analysis was kept separate from parsing because it kept the code single purpose (although it would be faster to update the analysis when each facet is parsed instead of processing each facet twice).

CLI-related functionality is kept in main.  There's not enough there to really justify giving it its own namespace.

## Enhancements

You could speed this parser up (at the cost of some extensibility) by updating the bounding box, the number of triangles, and the surface area by adding in each new facet at the moment it is parsed.  I would personally only undertake that if data supported it, if processing solids with millions of facets was a bottleneck for any reason.

Error handling is generally weak.  To turn this into a library, and not just a CLI tool, it would be useful to have some way of knowing why a particular file is invalid.  Right now, all parse errors get rolled into `parser::error::SolidError::Unparsable`, which works for the CLI but would need to be enhanced for production.  Other error handling is OK.

If you can restrict input to binary files, this could be adapted to take in files over the web, since `parser::binary::solid<T>` takes any kind of reader and reads in chunks.