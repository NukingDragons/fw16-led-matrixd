mod args;
mod image;
mod ipc;

use crate::{
	args::{Args, Commands, Matrix, MatrixNoPair},
	image::read_image,
	ipc::send_command,
};
use clap::Parser;
use shared::{
	find_ports,
	ipc::{IpcCommand, IpcResponse},
	version::Version,
};
use std::{error::Error, time::Duration};

fn main() -> Result<(), Box<dyn Error>>
{
	let args = Args::parse();

	let command = match args.command
	{
		Commands::List =>
		{
			println!("Enumerating ports");
			for port in find_ports(vec![0x32AC], vec![0x20, 0x1F])?
			{
				println!("Found port: {}", port);
			}

			None
		},
		Commands::Render(args) =>
		{
			if let Some(file) = args.file
			{
				let raw = read_image(file, args.matrix == Matrix::Pair)?;

				match args.matrix
				{
					Matrix::Left => Some(IpcCommand::RenderSingle(Some(raw), None)),
					Matrix::Right => Some(IpcCommand::RenderSingle(None, Some(raw))),
					Matrix::Both => Some(IpcCommand::RenderSingle(Some(raw.clone()), Some(raw))),
					Matrix::Pair => Some(IpcCommand::RenderPair(raw)),
				}
			}
			else if let Some(raw) = args.raw
			{
				match args.matrix
				{
					Matrix::Left => Some(IpcCommand::RenderSingle(Some(vec![(raw, Duration::default())]), None)),
					Matrix::Right => Some(IpcCommand::RenderSingle(None, Some(vec![(raw, Duration::default())]))),
					Matrix::Both => Some(IpcCommand::RenderSingle(
						Some(vec![(raw.clone(), Duration::default())]),
						Some(vec![(raw, Duration::default())]),
					)),
					Matrix::Pair => Some(IpcCommand::RenderPair(vec![(raw, Duration::default())])),
				}
			}
			else
			{
				None
			}
		},
		Commands::Brightness(args) =>
		{
			if let Some(set) = args.set
			{
				match args.matrix
				{
					MatrixNoPair::Left => Some(IpcCommand::SetBrightness(Some(set), None)),
					MatrixNoPair::Right => Some(IpcCommand::SetBrightness(None, Some(set))),
					MatrixNoPair::Both => Some(IpcCommand::SetBrightness(Some(set), Some(set))),
				}
			}
			else if args.get
			{
				match args.matrix
				{
					MatrixNoPair::Left => Some(IpcCommand::GetBrightness(true, false)),
					MatrixNoPair::Right => Some(IpcCommand::GetBrightness(false, true)),
					MatrixNoPair::Both => Some(IpcCommand::GetBrightness(true, true)),
				}
			}
			else
			{
				None
			}
		},
		Commands::Pattern(args) =>
		{
			if let Some(percentage) = args.percentage
			{
				match args.matrix
				{
					MatrixNoPair::Left => Some(IpcCommand::PatternPercentage(Some(percentage), None)),
					MatrixNoPair::Right => Some(IpcCommand::PatternPercentage(None, Some(percentage))),
					MatrixNoPair::Both => Some(IpcCommand::PatternPercentage(Some(percentage), Some(percentage))),
				}
			}
			else if args.gradient
			{
				match args.matrix
				{
					MatrixNoPair::Left => Some(IpcCommand::PatternGradient(true, false)),
					MatrixNoPair::Right => Some(IpcCommand::PatternGradient(false, true)),
					MatrixNoPair::Both => Some(IpcCommand::PatternGradient(true, true)),
				}
			}
			else if args.double_gradient
			{
				match args.matrix
				{
					MatrixNoPair::Left => Some(IpcCommand::PatternDoubleGradient(true, false)),
					MatrixNoPair::Right => Some(IpcCommand::PatternDoubleGradient(false, true)),
					MatrixNoPair::Both => Some(IpcCommand::PatternDoubleGradient(true, true)),
				}
			}
			else if args.lotus_horizontal
			{
				match args.matrix
				{
					MatrixNoPair::Left => Some(IpcCommand::PatternLotusHorizontal(true, false)),
					MatrixNoPair::Right => Some(IpcCommand::PatternLotusHorizontal(false, true)),
					MatrixNoPair::Both => Some(IpcCommand::PatternLotusHorizontal(true, true)),
				}
			}
			else if args.lotus_vertical
			{
				match args.matrix
				{
					MatrixNoPair::Left => Some(IpcCommand::PatternLotusVertical(true, false)),
					MatrixNoPair::Right => Some(IpcCommand::PatternLotusVertical(false, true)),
					MatrixNoPair::Both => Some(IpcCommand::PatternLotusVertical(true, true)),
				}
			}
			else if args.zigzag
			{
				match args.matrix
				{
					MatrixNoPair::Left => Some(IpcCommand::PatternZigzag(true, false)),
					MatrixNoPair::Right => Some(IpcCommand::PatternZigzag(false, true)),
					MatrixNoPair::Both => Some(IpcCommand::PatternZigzag(true, true)),
				}
			}
			else if args.fullbright
			{
				match args.matrix
				{
					MatrixNoPair::Left => Some(IpcCommand::PatternFullbright(true, false)),
					MatrixNoPair::Right => Some(IpcCommand::PatternFullbright(false, true)),
					MatrixNoPair::Both => Some(IpcCommand::PatternFullbright(true, true)),
				}
			}
			else if args.panic
			{
				match args.matrix
				{
					MatrixNoPair::Left => Some(IpcCommand::PatternPanic(true, false)),
					MatrixNoPair::Right => Some(IpcCommand::PatternPanic(false, true)),
					MatrixNoPair::Both => Some(IpcCommand::PatternPanic(true, true)),
				}
			}
			else
			{
				None
			}
		},
		Commands::Sleep(args) =>
		{
			if let Some(set) = args.set
			{
				match args.matrix
				{
					MatrixNoPair::Left => Some(IpcCommand::SetSleep(Some(set), None)),
					MatrixNoPair::Right => Some(IpcCommand::SetSleep(None, Some(set))),
					MatrixNoPair::Both => Some(IpcCommand::SetSleep(Some(set), Some(set))),
				}
			}
			else if args.get
			{
				match args.matrix
				{
					MatrixNoPair::Left => Some(IpcCommand::GetSleep(true, false)),
					MatrixNoPair::Right => Some(IpcCommand::GetSleep(false, true)),
					MatrixNoPair::Both => Some(IpcCommand::GetSleep(true, true)),
				}
			}
			else
			{
				None
			}
		},
		Commands::Animate(args) =>
		{
			if let Some(set) = args.set
			{
				match args.matrix
				{
					MatrixNoPair::Left => Some(IpcCommand::SetAnimate(Some(set), None)),
					MatrixNoPair::Right => Some(IpcCommand::SetAnimate(None, Some(set))),
					MatrixNoPair::Both => Some(IpcCommand::SetAnimate(Some(set), Some(set))),
				}
			}
			else if args.get
			{
				match args.matrix
				{
					MatrixNoPair::Left => Some(IpcCommand::GetAnimate(true, false)),
					MatrixNoPair::Right => Some(IpcCommand::GetAnimate(false, true)),
					MatrixNoPair::Both => Some(IpcCommand::GetAnimate(true, true)),
				}
			}
			else
			{
				None
			}
		},
		Commands::FlushCols(args) => match args.matrix
		{
			MatrixNoPair::Left => Some(IpcCommand::FlushCols(true, false)),
			MatrixNoPair::Right => Some(IpcCommand::FlushCols(false, true)),
			MatrixNoPair::Both => Some(IpcCommand::FlushCols(true, true)),
		},
		Commands::Version(args) => match args.matrix
		{
			MatrixNoPair::Left => Some(IpcCommand::Version(true, false)),
			MatrixNoPair::Right => Some(IpcCommand::Version(false, true)),
			MatrixNoPair::Both => Some(IpcCommand::Version(true, true)),
		},
		Commands::Bootloader(args) => match args.matrix
		{
			MatrixNoPair::Left => Some(IpcCommand::Bootloader(true, false)),
			MatrixNoPair::Right => Some(IpcCommand::Bootloader(false, true)),
			MatrixNoPair::Both => Some(IpcCommand::Bootloader(true, true)),
		},
		Commands::Crash(args) => match args.matrix
		{
			MatrixNoPair::Left => Some(IpcCommand::Crash(true, false)),
			MatrixNoPair::Right => Some(IpcCommand::Crash(false, true)),
			MatrixNoPair::Both => Some(IpcCommand::Crash(true, true)),
		},
		Commands::StageCol(args) => match args.matrix
		{
			Matrix::Left => Some(IpcCommand::StageCol(Some((args.column, args.values)), None)),
			Matrix::Right => Some(IpcCommand::StageCol(None, Some((args.column, args.values)))),
			Matrix::Both =>
			{
				Some(IpcCommand::StageCol(Some((args.column, args.values.clone())), Some((args.column, args.values))))
			},
			// When treating the matrixes as a pair, ensure the column seamlessly selects the correct matrix
			Matrix::Pair =>
			{
				if args.column <= 8
				{
					Some(IpcCommand::StageCol(Some((args.column, args.values)), None))
				}
				else
				{
					Some(IpcCommand::StageCol(None, Some((args.column - 9, args.values))))
				}
			},
		},
		Commands::DrawBW(args) => match args.matrix
		{
			MatrixNoPair::Left => Some(IpcCommand::DrawBW(Some(args.bitmap), None)),
			MatrixNoPair::Right => Some(IpcCommand::DrawBW(None, Some(args.bitmap))),
			MatrixNoPair::Both => Some(IpcCommand::DrawBW(Some(args.bitmap.clone()), Some(args.bitmap))),
		},
		#[cfg(windows)]
		Commands::Explain(args) =>
		{
			println!("{:#08x}: {}", args.code, match args.code
			{
				0xDEAD0000 => "Crate error, please check the daemon's log file for details",
				0xDEAD0001 => "Error reading config",
				0xDEAD0002 => "One of the ports specific in the config is not a USB device",
				0xDEAD0003 => "Can't determine one of the port type",
				0xDEAD0004 => "Invalid vector size was provided, this shouldn't have crashed the daemon",
				0xDEAD0005 => "Invalid vector size frame was provided, this shouldn't have crashed the daemon",
				0xDEAD0006 => "Invalid column number was provided, this shouldn't have crashed the daemon",
				0xDEAD0007 => "An internal handler error occurred, this shouldn't have crashed the daemon",
				_ => "Unknown error",
			});

			None
		},
	};

	if let Some(command) = command
	{
		let collect_response = command.needs_response();
		match send_command("fw16-led-matrixd.socket".to_string(), command, collect_response)
		{
			Ok(Some(response)) => match response
			{
				IpcResponse::Brightness(left, right) =>
				{
					if let Some(left) = left
					{
						println!("Left LED matrix brightness: {}", left);
					}

					if let Some(right) = right
					{
						println!("Right LED matrix brightness: {}", right);
					}
				},
				IpcResponse::Sleeping(left, right) =>
				{
					if let Some(left) = left
					{
						println!("Left LED matrix sleeping: {}", left);
					}

					if let Some(right) = right
					{
						println!("Right LED matrix sleeping: {}", right);
					}
				},
				IpcResponse::Animated(left, right) =>
				{
					if let Some(left) = left
					{
						println!("Left LED matrix animated: {}", left);
					}

					if let Some(right) = right
					{
						println!("Right LED matrix animated: {}", right);
					}
				},
				IpcResponse::Version(left, right) =>
				{
					if let Some(left) = left
					{
						match Version::try_from(left)
						{
							Ok(version) => println!("Left LED matrix version: {}", version),
							Err(e) => println!("Failed to get left LED matrix version with error: {}", e),
						}
					}

					if let Some(right) = right
					{
						match Version::try_from(right)
						{
							Ok(version) => println!("Right LED matrix version: {}", version),
							Err(e) => println!("Failed to get right LED matrix version with error: {}", e),
						}
					}
				},
				IpcResponse::InvalidCommand(e) =>
				{
					println!("Daemon encountered an issue executing the command with error: {}", e)
				},
			},
			Ok(None) => (),
			Err(e) => println!("Failed to send command with error: {}", e),
		}
	}

	Ok(())
}
