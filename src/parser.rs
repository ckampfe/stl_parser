use crate::solid::Solid;
use error::SolidError;
use nom::IResult;

pub mod ascii;
pub mod binary;
pub mod error;

pub fn solid_from_filepath(filepath: &str, is_ascii: bool) -> IResult<(), Solid, SolidError> {
    if is_ascii {
        ascii::solid_from_filepath(filepath)
    } else {
        binary::solid_from_filepath(filepath)
    }
}
