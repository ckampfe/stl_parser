use crate::coordinate::Coordinate;
use crate::facet::Facet;
use crate::parser::error::SolidError;
use crate::solid::Solid;
use nom::bytes::complete::{tag_no_case, take_while1};
use nom::character::is_space;
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::{map, map_parser, not, opt};
use nom::multi::separated_list;
use nom::number::complete::float;
use nom::sequence::tuple;
use nom::{AsBytes, IResult};
use std::fs::File;
use std::io::{BufReader, Read};

fn vector_3d(input: &[u8]) -> IResult<&[u8], (f32, f32, f32)> {
    map(
        tuple((float, multispace1, float, multispace1, float)),
        |(i, _, j, _, k)| (i, j, k),
    )(input)
}

fn vertex(input: &[u8]) -> IResult<&[u8], Coordinate> {
    map(
        tuple((tag_no_case("vertex"), multispace1, vector_3d)),
        |(_, _, vector)| Coordinate::from(vector),
    )(input)
}

fn vertices(input: &[u8]) -> IResult<&[u8], (Coordinate, Coordinate, Coordinate)> {
    map(
        tuple((
            multispace1,
            vertex,
            multispace1,
            vertex,
            multispace1,
            vertex,
            multispace1,
        )),
        |(_, a, _, b, _, c, _)| (a, b, c),
    )(input)
}

fn normal_vector(input: &[u8]) -> IResult<&[u8], (f32, f32, f32)> {
    map(
        tuple((tag_no_case("normal"), multispace1, vector_3d)),
        |(_, _, vector)| vector,
    )(input)
}

fn outer_loop(input: &[u8]) -> IResult<&[u8], (Coordinate, Coordinate, Coordinate)> {
    map(
        tuple((
            tag_no_case("outer"),
            multispace1,
            tag_no_case("loop"),
            vertices,
            tag_no_case("endloop"),
        )),
        |(_, _, _, vs, _)| vs,
    )(input)
}

pub fn facet(input: &[u8]) -> IResult<&[u8], Facet> {
    map(
        tuple((
            tag_no_case("facet"),
            multispace1,
            normal_vector,
            multispace1,
            outer_loop,
            multispace1,
            tag_no_case("endfacet"),
        )),
        |(_, _, nv, _, vs, _, _)| Facet {
            normal_vector: nv,
            vertices: vs,
        },
    )(input)
}

fn facets(input: &[u8]) -> IResult<&[u8], Vec<Facet>> {
    map(
        tuple((multispace1, opt(tuple((separated_list(multispace1, facet), multispace1))))),
        |(_, maybe_facets)| match maybe_facets {
            None => vec![],
            Some((fs, _)) => fs
        }
    )(input)
}

fn solid_name_excluding_facet(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    map(not(tag_no_case("facet")), |()| input.clone().to_vec())(input)
}

fn valid_solid_name(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    map_parser(take_while1(|c| !is_space(c)), solid_name_excluding_facet)(input)
}

fn solid_name(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    map(tuple((multispace1, valid_solid_name)), |(_, name)| name)(input)
}

pub fn solid(input: &[u8]) -> IResult<(), Solid, SolidError> {
    match map(
        tuple((
            // Leading whitespace
            multispace0,
            tag_no_case("solid"),
            // This solid name is used.
            // Note that binary files do not have a name field, so this is maybe less useful than you would think.
            opt(solid_name),
            facets,
            tag_no_case("endsolid"),
            // We do not check for consistency between the two names.
            // This one is included to ensure parsing works, but the result is thrown away.
            opt(solid_name),
            // Trailing whitespace
            multispace0,
        )),
        |(_, _, name, fs, _, _, _)| Solid { name, facets: fs },
    )(input)
    {
        Ok((_, s)) => Ok(((), s)),
        Err(_) => Err(nom::Err::Failure(SolidError::Unparsable)),
    }
}

pub fn solid_from_file(file: &File) -> IResult<(), Solid, SolidError> {
    let mut buffer = vec![];
    let mut reader = BufReader::new(file);
    if let Err(error) = reader.read_to_end(&mut buffer) {
        return Err(nom::Err::Failure(SolidError::IO(error)));
    }
    solid(buffer.as_bytes())
}

pub fn solid_from_filepath(filepath: &str) -> IResult<(), Solid, SolidError> {
    match File::open(filepath) {
        Ok(file) => solid_from_file(&file),
        Err(error) => Err(nom::Err::Failure(SolidError::IO(error))),
    }
}
