use crate::args::MatrixNoPair;
use clap::Args;

#[derive(Args)]
pub struct VersionArgs
{
	/// Which matrix to target
	#[arg(short, long, value_enum)]
	pub matrix: MatrixNoPair,
}
