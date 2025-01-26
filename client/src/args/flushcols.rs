use crate::args::MatrixNoPair;
use clap::Args;

#[derive(Args)]
pub struct FlushColsArgs
{
	/// Which matrix to target
	#[arg(short, long, value_enum)]
	pub matrix: MatrixNoPair,
}
