use crate::args::MatrixNoPair;
use clap::Args;

#[derive(Args)]
pub struct SleepArgs
{
	/// Which matrix to target
	#[arg(short, long, value_enum)]
	pub matrix: MatrixNoPair,

	/// Sets the sleeping state
	#[arg(short, long, conflicts_with = "get", required_unless_present("get"))]
	pub set: Option<bool>,

	/// Gets the sleeping state
	#[arg(short, long, conflicts_with = "set")]
	pub get: bool,
}
