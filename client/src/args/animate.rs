use crate::args::MatrixNoPair;
use clap::Args;

#[derive(Args)]
pub struct AnimateArgs
{
	/// Which matrix to target
	#[arg(short, long, value_enum)]
	pub matrix: MatrixNoPair,

	/// Sets the vertical scroll
	#[arg(short, long, conflicts_with = "get", required_unless_present("get"))]
	pub set: Option<bool>,

	/// Get the vertical scroll status
	#[arg(short, long, conflicts_with = "set")]
	pub get: bool,
}
