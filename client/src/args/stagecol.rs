use crate::args::Matrix;
use clap::{value_parser, Args};
use clap_num::maybe_hex;

#[derive(Args)]
pub struct StageColArgs
{
	/// Which matrix to target
	#[arg(short, long, value_enum)]
	pub matrix: Matrix,

	/// Which column to stage [0-8, or 0-17 if matrix is set to "pair"]
	#[arg(short, long, required = true, value_parser = value_parser!(u8).range(0..=17))]
	pub column: u8,

	/// Comma separated grayscale values to set the LEDs as [permits hex and decimal values: "-v 0 0xFF 40..."]
	#[arg(short, long, value_names = ["VAL1", "VAL2"], required = true, num_args = 34, value_parser = maybe_hex::<u8>)]
	pub values: Vec<u8>,
}
