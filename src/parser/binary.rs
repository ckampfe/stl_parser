use crate::coordinate::Coordinate;
use crate::facet::Facet;
use crate::solid::Solid;
use nom::combinator::map;
use nom::number::complete::{le_f32, le_u16, le_u32};
use nom::sequence::tuple;
use nom::{self, IResult};
use std::fs::File;
use std::io::{self, BufReader, Read};

/// This constant is defined by the binary STL format.
const FACET_SIZE: usize = 4 * 3 * 4; // 4 vectors/facet * 3 dimensions/vector * 4 bytes/dimension = 48 bytes/facet
/// This constant was chosen because it is the chunk size used in AWS S3 streaming file uploads.
/// That seems like a pretty well vetted chunk size.
const MAX_CHUNK_SIZE: usize = 5 * 1024;
/// However, the MAX_CHUNK_SIZE doesn't fit a fixed number of facets cleanly into it.
/// So instead of using 5 MiB exactly, I used the number closest to 5 MiB that will fit an integer number of facets.
const CHUNK_SIZE: usize = MAX_CHUNK_SIZE - (MAX_CHUNK_SIZE % FACET_SIZE); // 5 MiB - (5 MiB % FACET_SIZE) = 5088 bytes
const FACETS_PER_CHUNK: usize = CHUNK_SIZE / FACET_SIZE; // 5088 bytes/chunk / 48 bytes/facet = 106 facets/chunk

/// Returns the number of CHUNK_SIZE chunks needed to process all facets in the entire binary STL file.
/// For a file with 2000 facets, the portion of the full file containing facets is 33024 bytes.
/// So to process facets for this example file, with a CHUNK_SIZE of 4992 bytes, we need to process 6 chunks and one
/// nonstandard chunk of 3072 bytes.
fn chunks_to_process(num_facets: usize) -> (usize, usize) {
    let total_size = num_facets * FACET_SIZE; // 2000 facets * 48 bytes/facet = 96000 bytes
    let num_chunks = total_size / CHUNK_SIZE; // 96000 bytes / 5088 bytes/chunk = 18 chunks
    let last_chunk_size = total_size % CHUNK_SIZE; // 96000 bytes % 5088 bytes/chunk = 4416 bytes
    (num_chunks, last_chunk_size)
}

fn vector_3d(input: &[u8]) -> IResult<&[u8], (f32, f32, f32)> {
    tuple((le_f32, le_f32, le_f32))(input)
}

fn facet(input: &[u8]) -> IResult<&[u8], Facet> {
    map(
        tuple((vector_3d, vector_3d, vector_3d, vector_3d)),
        |(normal_vector, v1, v2, v3)| Facet {
            normal_vector,
            vertices: (
                Coordinate::from(v1),
                Coordinate::from(v2),
                Coordinate::from(v3),
            ),
        },
    )(input)
}

#[derive(Debug)]
pub enum SolidError {
    IO(io::Error),
    UnparsableAttributeByteCount,
    UnparsableNumFacets,
    UnparsableFacet,
}

pub fn solid<T>(reader: &mut T) -> IResult<(), Solid, SolidError>
where
    T: Read,
{
    let mut header = [0u8; 80];
    if let Err(error) = reader.read_exact(&mut header) {
        return Err(nom::Err::Failure(SolidError::IO(error)));
    };

    let mut num_facets_buffer = [0u8; 4];
    if let Err(error) = reader.read_exact(&mut num_facets_buffer) {
        return Err(nom::Err::Failure(SolidError::IO(error)));
    };
    let mut num_facets = 0;
    match le_u32::<nom::error::VerboseError<&[u8]>>(num_facets_buffer.as_ref()) {
        Ok((_, nf)) => {
            num_facets = nf;
        }
        Err(_) => return Err(nom::Err::Failure(SolidError::UnparsableNumFacets)),
    };

    let (num_chunks, last_chunk_size) = chunks_to_process(num_facets as usize);
    let mut all_facets = vec![];

    for _ in 0..num_chunks {
        let mut chunk_buffer = [0u8; CHUNK_SIZE];
        if let Err(error) = reader.read_exact(&mut chunk_buffer) {
            return Err(nom::Err::Failure(SolidError::IO(error)));
        }

        let mut input = chunk_buffer.as_ref();

        for _ in 0..FACETS_PER_CHUNK {
            match facet(&input) {
                Ok((rest, f)) => {
                    all_facets.push(f);
                    input = rest;
                }
                Err(_) => {
                    return Err(nom::Err::Failure(SolidError::UnparsableFacet));
                }
            }
        }
    }

    if last_chunk_size > 0 {
        let mut chunk_buffer = [0u8; CHUNK_SIZE+2];
        if let Err(error) = reader.read(&mut chunk_buffer) {
            return Err(nom::Err::Failure(SolidError::IO(error)));
        }

        let mut input = chunk_buffer.as_ref();

        for _ in 0..last_chunk_size / FACET_SIZE {
            match facet(&input) {
                Ok((rest, f)) => {
                    all_facets.push(f);
                    input = rest;
                }
                Err(_) => {
                    return Err(nom::Err::Failure(SolidError::UnparsableFacet));
                }
            }
        }

        if let Err(_) = le_u16::<nom::error::VerboseError<&[u8]>>(input) {
            return Err(nom::Err::Failure(SolidError::UnparsableNumFacets));
        };
    } else {
        let mut chunk_buffer = [0u8; 2];
        if let Err(error) = reader.read(&mut chunk_buffer) {
            return Err(nom::Err::Failure(SolidError::IO(error)));
        }

        if let Err(_) = le_u16::<nom::error::VerboseError<&[u8]>>(chunk_buffer.as_ref()) {
            return Err(nom::Err::Failure(SolidError::UnparsableNumFacets));
        };
    }

    Ok((
        (),
        Solid {
            name: None,
            facets: all_facets,
        },
    ))
}

pub fn solid_from_file(file: &File) -> IResult<(), Solid, SolidError> {
    let mut reader = BufReader::new(file);
    solid(&mut reader)
}

pub fn solid_from_filepath(filepath: &str) -> IResult<(), Solid, SolidError> {
    match File::open(filepath) {
        Ok(file) => solid_from_file(&file),
        Err(error) => Err(nom::Err::Failure(SolidError::IO(error))),
    }
}
