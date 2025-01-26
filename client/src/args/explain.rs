use clap::Args;
use clap_num::maybe_hex;

#[derive(Args)]
pub struct ExplainArgs
{
	/// The service exit code to explain [permits hex and decimal values: "-c 0xDEAD0000"]
	#[arg(short, long, value_parser = maybe_hex::<u32>)]
	pub code: u32,
}
