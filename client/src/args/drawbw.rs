use crate::args::MatrixNoPair;
use clap::Args;
use clap_num::maybe_hex;

#[derive(Args)]
pub struct DrawBWArgs
{
	/// Which matrix to target
	#[arg(short, long, value_enum)]
	pub matrix: MatrixNoPair,

	/// Comma separated bitmap values to set the LEDs as black and white [permits hex and decimal values: "-b 0 0xFF 40"]
	#[arg(short, long, value_names = ["VAL1", "VAL2"], required = true, num_args = 39, value_parser = maybe_hex::<u8>)]
	pub bitmap: Vec<u8>,
}
