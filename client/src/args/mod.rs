mod animate;
mod bootloader;
mod brightness;
mod crash;
mod drawbw;
mod flushcols;
mod pattern;
mod render;
mod sleep;
mod stagecol;
mod version;

#[cfg(windows)]
mod explain;

use clap::{crate_authors, Parser, Subcommand, ValueEnum};

pub use animate::AnimateArgs;
pub use bootloader::BootloaderArgs;
pub use brightness::BrightnessArgs;
pub use crash::CrashArgs;
pub use drawbw::DrawBWArgs;
pub use flushcols::FlushColsArgs;
pub use pattern::PatternArgs;
pub use render::RenderArgs;
pub use sleep::SleepArgs;
pub use stagecol::StageColArgs;
pub use version::VersionArgs;

#[cfg(windows)]
pub use explain::ExplainArgs;

#[derive(Parser)]
#[command(author = crate_authors!("\n"), version, long_about = None)]
#[command(about = "Framework 16 LED matrix command line interface")]
#[command(help_template = "{author-with-newline} {about-section}Version: {version}\n{usage-heading} {usage}\n\n{all-args} {tab}")]
pub struct Args
{
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Matrix
{
	Left,
	Right,
	Both,
	Pair,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum MatrixNoPair
{
	Left,
	Right,
	Both,
}

#[derive(Subcommand)]
pub enum Commands
{
	/// Lists all available LED matrixes on the system
	List,

	#[cfg(windows)]
	/// Explain what the "Service Exit Code" means if the daemon dies on Windows
	Explain(ExplainArgs),

	/// Render a JPG, PNG, APNG, GIF, WEBP, and various other image types [can also specify a raw byte vector for a single frame]
	Render(RenderArgs),

	/// Gets and sets the brightness
	Brightness(BrightnessArgs),

	/// Sets a predefined pattern
	Pattern(PatternArgs),

	/// Gets and sets the sleeping state
	Sleep(SleepArgs),

	/// Gets and sets the vertical scroll
	Animate(AnimateArgs),

	/// Flush all columns and render any staged ones on the specified matrix
	FlushCols(FlushColsArgs),

	/// Fetch the version of the specified matrix
	Version(VersionArgs),

	/// Have the specified matrix enter the bootloader [provided for completeness]
	Bootloader(BootloaderArgs),

	/// Cause the specified matrix to panic [provided for completeness]
	Crash(CrashArgs),

	/// Stage a column on the specified matrix [provided for completeness, please use render instead]
	StageCol(StageColArgs),

	/// Draw a black and white image from a vector [provided for completeness, please use render instead]
	DrawBW(DrawBWArgs),
}

impl From<MatrixNoPair> for Matrix
{
	fn from(m: MatrixNoPair) -> Self
	{
		match m
		{
			MatrixNoPair::Left => Matrix::Left,
			MatrixNoPair::Right => Matrix::Right,
			MatrixNoPair::Both => Matrix::Both,
		}
	}
}
