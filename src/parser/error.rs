use std::io;

#[derive(Debug)]
pub enum SolidError {
    IO(io::Error),
    Unparsable,
}
