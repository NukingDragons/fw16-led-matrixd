use crate::args::Matrix;
use clap::Args;
use clap_num::maybe_hex;

#[derive(Args)]
pub struct RenderArgs
{
	/// Which matrix to target
	#[arg(short, long, value_enum)]
	pub matrix: Matrix,

	/// Image file to render. Many image types are supported, including animated ones like APNG and GIF
	/// The image will be rendered as an 8-bit grayscale image sized at 9x34, or 18x34 when matrix is set to "pair"
	/// If the image does not meet those criteria, they will be scaled down, cropped, and converted to grayscale
	#[arg(short, long, conflicts_with = "raw", required_unless_present = "raw", verbatim_doc_comment)]
	pub file: Option<String>,

	/// Comma separated grayscale values to set the LEDs as [permits hex and decimal values: "-r 0 0xFF 40"]
	/// This must be 306 bytes, or 612 bytes when matrix is set to "pair"
	#[arg(short, long, value_names = ["VAL1", "VAL2"], required = false, num_args = 306..=612, value_parser = maybe_hex::<u8>, verbatim_doc_comment, conflicts_with = "file")]
	pub raw: Option<Vec<u8>>,
}
