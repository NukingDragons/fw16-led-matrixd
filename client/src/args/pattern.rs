use crate::args::MatrixNoPair;
use clap::Args;

#[derive(Args)]
pub struct PatternArgs
{
	/// Which matrix to target
	#[arg(short, long, value_enum, requires = "pattern")]
	pub matrix: MatrixNoPair,

	/// Displays a percentage
	#[arg(short, long, group = "pattern", conflicts_with_all = ["gradient", "double_gradient", "lotus_horizontal", "lotus_vertical", "zigzag", "fullbright", "panic"])]
	pub percentage: Option<u8>,

	/// Displays a gradient
	#[arg(short, long, group = "pattern", conflicts_with_all = ["percentage", "double_gradient", "lotus_horizontal", "lotus_vertical", "zigzag", "fullbright", "panic"])]
	pub gradient: bool,

	/// Displays a double gradient
	#[arg(short, long, group = "pattern", conflicts_with_all = ["percentage", "gradient", "lotus_horizontal", "lotus_vertical", "zigzag", "fullbright", "panic"])]
	pub double_gradient: bool,

	/// Displays the text "LOTUS" horizontally
	#[arg(short, long, group = "pattern", conflicts_with_all = ["percentage", "gradient", "double_gradient", "lotus_vertical", "zigzag", "fullbright", "panic"])]
	pub lotus_horizontal: bool,

	/// Displays the text "LOTUS" vertically
	#[arg(short('L'), long, group = "pattern", conflicts_with_all = ["percentage", "gradient", "double_gradient", "lotus_horizontal", "zigzag", "fullbright", "panic"])]
	pub lotus_vertical: bool,

	/// Displays a zigzag
	#[arg(short, long, group = "pattern", conflicts_with_all = ["percentage", "gradient", "double_gradient", "lotus_horizontal", "lotus_vertical", "fullbright", "panic"])]
	pub zigzag: bool,

	/// Turns all of the LEDs on to the max brightness
	#[arg(short, long, group = "pattern", conflicts_with_all = ["percentage", "gradient", "double_gradient", "lotus_horizontal", "lotus_vertical", "zigzag", "panic"])]
	pub fullbright: bool,

	/// Displays the text "PANIC" vertically
	#[arg(short('P'), long, group = "pattern", conflicts_with_all = ["percentage", "gradient", "double_gradient", "lotus_horizontal", "lotus_vertical", "zigzag", "fullbright"])]
	pub panic: bool,
}
