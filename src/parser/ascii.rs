use crate::facet::Facet;
use crate::solid::Solid;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::{alphanumeric1, multispace0, multispace1};
use nom::combinator::{map, map_parser, not, opt};
use nom::multi::separated_list;
use nom::number::complete::float;
use nom::sequence::tuple;
use nom::IResult;

fn vector_3d(input: &[u8]) -> IResult<&[u8], (f32, f32, f32)> {
    map(
        tuple((float, multispace1, float, multispace1, float)),
        |(i, _, j, _, k)| (i, j, k),
    )(input)
}

fn vertex(input: &[u8]) -> IResult<&[u8], (f32, f32, f32)> {
    map(
        tuple((tag_no_case("vertex"), multispace1, vector_3d)),
        |(_, _, vector)| vector,
    )(input)
}

fn vertices(input: &[u8]) -> IResult<&[u8], Vec<(f32, f32, f32)>> {
    if let Ok((rest, (_, vs, _))) = tuple((
        multispace1,
        separated_list(multispace1, vertex),
        multispace1,
    ))(input)
    {
        Ok((rest, vs))
    } else {
        map(multispace1, |_| vec![])(input)
    }
}

fn normal_vector(input: &[u8]) -> IResult<&[u8], (f32, f32, f32)> {
    map(
        tuple((tag_no_case("normal"), multispace1, vector_3d)),
        |(_, _, vector)| vector,
    )(input)
}

fn outer_loop(input: &[u8]) -> IResult<&[u8], Vec<(f32, f32, f32)>> {
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
    if let Ok((rest, (_, fs, _))) =
        tuple((multispace1, separated_list(multispace1, facet), multispace1))(input)
    {
        Ok((rest, fs))
    } else {
        map(multispace1, |_| vec![])(input)
    }
}

fn solid_name_excluding_facet(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    map(not(tag_no_case("facet")), |()| input.clone().to_vec())(input)
}

fn valid_solid_name(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    map_parser(alphanumeric1, solid_name_excluding_facet)(input)
}

fn solid_name(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    map(tuple((multispace1, valid_solid_name)), |(_, name)| name)(input)
}

fn solid(input: &[u8]) -> IResult<&[u8], Solid> {
    map(
        tuple((
            tag_no_case("solid"),
            opt(solid_name),
            facets,
            tag_no_case("endsolid"),
        )),
        |(_, name, fs, _)| Solid { name, facets: fs },
    )(input)
}

fn parse(input: &[u8]) -> IResult<&[u8], Solid> {
    map(tuple((multispace0, solid, multispace0)), |(_, s, _)| s)(input)
}
