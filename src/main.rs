extern crate clap;
extern crate nom;

pub mod bounding_box;
pub mod coordinate;
pub mod facet;
pub mod parser;
pub mod solid;

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

    match parser::binary::solid_from_filepath(filepath) {
        Ok(((), solid)) => {
            let (num_facets, surface_area, bounding_box) = solid.analyze();
            println!("Number of Triangles: {}", num_facets);
            println!("Surface Area: {:.4}", surface_area);
            println!("Bounding Box: {}", bounding_box);
        }
        Err(error) => {
            dbg!(error);
            panic!("Oh no!")
        },
    }
}
