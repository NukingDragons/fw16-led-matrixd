use crate::error::Error as CrateError;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::{error::Error, time::Duration};

#[derive(Debug, Serialize, Deserialize)]
pub enum IpcCommand
{
	SetBrightness(Option<u8>, Option<u8>),
	GetBrightness(bool, bool),
	PatternPercentage(Option<u8>, Option<u8>),
	PatternGradient(bool, bool),
	PatternDoubleGradient(bool, bool),
	PatternLotusHorizontal(bool, bool),
	PatternLotusVertical(bool, bool),
	PatternZigzag(bool, bool),
	PatternFullbright(bool, bool),
	PatternPanic(bool, bool),
	Bootloader(bool, bool),
	SetSleep(Option<bool>, Option<bool>),
	GetSleep(bool, bool),
	SetAnimate(Option<bool>, Option<bool>),
	GetAnimate(bool, bool),
	Crash(bool, bool),
	DrawBW(Option<Vec<u8>>, Option<Vec<u8>>),
	StageCol(Option<(u8, Vec<u8>)>, Option<(u8, Vec<u8>)>),
	FlushCols(bool, bool),
	Version(bool, bool),
	RenderSingle(Option<Vec<(Vec<u8>, Duration)>>, Option<Vec<(Vec<u8>, Duration)>>),
	RenderPair(Vec<(Vec<u8>, Duration)>),
}

impl IpcCommand
{
	pub fn validate(&self) -> Result<(), CrateError>
	{
		let mut matrix = "";
		match self
		{
			IpcCommand::DrawBW(left, right) =>
			{
				if let Some(l) = left
				{
					if l.len() != 39
					{
						matrix = "left matrix has ";
					}
				}

				if let Some(r) = right
				{
					if r.len() != 39
					{
						matrix = if !matrix.is_empty() { "both matrixes have " } else { "right matrix has " };
					}
				}

				if matrix.is_empty()
				{
					Ok(())
				}
				else
				{
					Err(CrateError::InvalidVecSize(matrix.to_string(), 39))
				}
			},
			IpcCommand::StageCol(left, right) =>
			{
				if let Some((col, vals)) = left
				{
					if *col >= 9
					{
						return Err(CrateError::InvalidColNumber(*col));
					}

					if vals.len() != 34
					{
						matrix = "left matrix has ";
					}
				}

				if let Some((col, vals)) = right
				{
					if *col >= 9
					{
						return Err(CrateError::InvalidColNumber(*col));
					}

					if vals.len() != 34
					{
						matrix = if !matrix.is_empty() { "both matrixes have " } else { "right matrix has " };
					}
				}

				if matrix.is_empty()
				{
					Ok(())
				}
				else
				{
					Err(CrateError::InvalidVecSize(matrix.to_string(), 34))
				}
			},
			IpcCommand::RenderSingle(left, right) =>
			{
				if let Some(l) = left
				{
					for (frame_num, (frame, _)) in l.iter().enumerate()
					{
						if frame.len() != 306
						{
							return Err(CrateError::InvalidVecSizeFrame(
							                                           "left matrix has ".to_string(),
							                                           frame.len(),
							                                           frame_num,
							));
						}
					}
				}

				if let Some(r) = right
				{
					for (frame_num, (frame, _)) in r.iter().enumerate()
					{
						if frame.len() != 306
						{
							return Err(CrateError::InvalidVecSizeFrame(
							                                           "right matrix has ".to_string(),
							                                           frame.len(),
							                                           frame_num,
							));
						}
					}
				}

				Ok(())
			},
			IpcCommand::RenderPair(pair) =>
			{
				for (frame_num, (frame, _)) in pair.iter().enumerate()
				{
					if frame.len() != 612
					{
						return Err(CrateError::InvalidVecSizeFrame("".to_string(), frame.len(), frame_num));
					}
				}

				Ok(())
			},
			_ => Ok(()),
		}
	}

	pub fn needs_response(&self) -> bool
	{
		matches!(self, Self::GetBrightness(_, _) | Self::GetSleep(_, _) | Self::GetAnimate(_, _) | Self::Version(_, _))
	}

	pub fn to_json(&self) -> Result<String, Box<dyn Error>>
	{
		Ok(format!("{}\n", to_string(self)?))
	}

	pub fn from_json(json: String) -> Result<Self, Box<dyn Error>>
	{
		Ok(from_str(&json)?)
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IpcResponse
{
	Brightness(Option<u8>, Option<u8>),
	Sleeping(Option<bool>, Option<bool>),
	Animated(Option<bool>, Option<bool>),
	Version(Option<Vec<u8>>, Option<Vec<u8>>),
	InvalidCommand(CrateError),
}

impl IpcResponse
{
	pub fn to_json(&self) -> Result<String, Box<dyn Error>>
	{
		Ok(format!("{}\n", to_string(self)?))
	}

	pub fn from_json(json: String) -> Result<Self, Box<dyn Error>>
	{
		Ok(from_str(&json)?)
	}
}
