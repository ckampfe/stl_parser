use crate::solid::Solid;
use error::SolidError;
use nom::IResult;

pub mod ascii;
pub mod binary;
pub mod error;

/// Given a filepath and the type of STL file, will attempt to parse the Solid within it.
/// For performance on large files, use binary files (not ASCII) as ASCII files must be loaded in-memory.
pub fn solid_from_filepath(filepath: &str, is_ascii: bool) -> IResult<(), Solid, SolidError> {
    if is_ascii {
        ascii::solid_from_filepath(filepath)
    } else {
        binary::solid_from_filepath(filepath)
    }
}
