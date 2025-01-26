use crate::args::MatrixNoPair;
use clap::Args;

#[derive(Args)]
pub struct BrightnessArgs
{
	/// Which matrix to target
	#[arg(short, long, value_enum)]
	pub matrix: MatrixNoPair,

	/// Sets the brightness
	#[arg(short, long, conflicts_with = "get", value_name = "BRIGHTNESS", required_unless_present("get"))]
	pub set: Option<u8>,

	/// Gets the brightness
	#[arg(short, long, required = false, conflicts_with = "set")]
	pub get: bool,
}
