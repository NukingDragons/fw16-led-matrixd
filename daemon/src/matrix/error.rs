use std::{
	error::Error,
	fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub enum MatrixError
{
	InvalidPort(String),
}

impl Error for MatrixError {}

impl Display for MatrixError
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error>
	{
		match self
		{
			Self::InvalidPort(port) => write!(f, "port \"{}\" is not a valid FW16 USB LED matrix", port),
		}
	}
}
