use serde::Deserialize;
use shared::error::Error as SharedError;
use std::{
	error::Error,
	fmt::{self, Display, Formatter},
	fs::read_to_string,
};

#[derive(Deserialize)]
pub struct Config
{
	pub left_matrix: Option<Matrix>,
	pub right_matrix: Option<Matrix>,
}

#[derive(Deserialize)]
pub struct Matrix
{
	pub port: String,
	pub baudrate: usize,
	pub sleeping: bool,
}

#[derive(Debug)]
pub enum ConfigError
{
	MissingMatrix,
}

impl Error for ConfigError {}

impl Display for ConfigError
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error>
	{
		match self
		{
			Self::MissingMatrix => write!(f, "missing at least one matrix from the config file"),
		}
	}
}

pub fn read_config(config: String) -> Result<Config, Box<dyn Error>>
{
	let config_contents = match read_to_string(config)
	{
		Ok(c) => c,
		Err(e) => return Err(Box::new(SharedError::Config(e.to_string()))),
	};

	let config: Config = match toml::from_str(&config_contents)
	{
		Ok(c) => c,
		Err(e) => return Err(Box::new(SharedError::Config(e.to_string()))),
	};

	let missing_left = config.left_matrix.is_none();
	let missing_right = config.right_matrix.is_none();
	let missing_both = missing_left && missing_right;

	if missing_both
	{
		Err(Box::new(ConfigError::MissingMatrix))
	}
	else
	{
		Ok(config)
	}
}
