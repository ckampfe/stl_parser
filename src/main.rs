extern crate clap;
extern crate nom;

use parser::error::SolidError;
use std::process::exit;

pub mod bounding_box;
pub mod coordinate;
pub mod facet;
pub mod parser;
pub mod solid;

fn handle_parse_error(error: SolidError) {
    match error {
        SolidError::Unparsable => {
            println!("The solid file is unparsable by this program");
            exit(1);
        }
        SolidError::IO(io_error) => {
            println!("Could not read from the file due to\n{}", io_error);
            exit(1);
        }
    }
}

fn main() {
    let matches = clap::App::new("stlp")
        .version("1.0.0")
        .author("Jim Berlage <james.berlage@gmail.com>")
        .about("Command line utility to get the complexity of STL files")
        .arg(
            clap::Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("The path to the file to be analyzed")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("ascii")
                .short("a")
                .long("ascii")
                .help("Signals that the file is ascii, not binary"),
        )
        .get_matches();
    let filepath = matches.value_of("file").unwrap();
    let is_ascii = matches.is_present("ascii");

    match parser::solid_from_filepath(filepath, is_ascii) {
        Ok(((), solid)) => {
            let (num_facets, surface_area, bounding_box) = solid.analyze();
            println!("Number of Triangles: {}", num_facets);
            println!("Surface Area: {:.4}", surface_area);
            println!("Bounding Box: {}", bounding_box);
        },
        Err(nom::Err::Failure(error)) => handle_parse_error(error),
        Err(nom::Err::Error(error)) => handle_parse_error(error),
        Err(nom::Err::Incomplete(_)) => {
            println!("An unexpected error occurred");
            exit(1);
        }
    }
}
