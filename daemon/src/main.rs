mod config;
mod handler;
mod ipc;
mod logger;
mod matrix;

pub use config::Matrix;

use crate::{config::read_config, handler::handler, handler::HandlerData, ipc::listen, matrix::version};
use clap::{crate_authors, Parser};
use log::{error, info};
use logger::setup_logging;
use std::{
	error::Error,
	sync::{Arc, Mutex},
	thread::{sleep, spawn},
	time::Duration,
};

#[derive(Parser)]
#[command(author = crate_authors!("\n"), version, long_about = None)]
#[command(about = "Framework 16 LED matrix control daemon")]
#[command(help_template = "{author-with-newline} {about-section}Version: {version}\n{usage-heading} {usage}\n\n{all-args} {tab}")]
struct Args
{
	/// Log file for the daemon
	#[cfg(unix)]
	#[arg(short, long, default_value = "/var/log/fw16-led-matrixd.log")]
	log_file: String,

	/// Config file for the daemon
	#[cfg(unix)]
	#[arg(short, long, default_value = "/etc/fw16-led-matrixd/config.toml")]
	config: String,

	/// PID file for the daemon
	#[cfg(unix)]
	#[arg(short, long, default_value = "/run/fw16-led-matrixd.pid")]
	pid_file: String,

	/// Log file for the daemon
	#[cfg(windows)]
	#[arg(short, long, default_value = "C:\\Program Files\\fw16-led-matrixd\\fw16-led-matrixd.log")]
	log_file: String,

	/// Config file for the daemon
	#[cfg(windows)]
	#[arg(short, long, default_value = "C:\\Program Files\\fw16-led-matrixd\\config.toml")]
	config: String,
}

#[cfg(windows)]
mod windows;

#[cfg(windows)]
fn main() -> windows_service::Result<()>
{
	// Let clap parse args and render help menus before starting the windows service
	let _ = Args::parse();

	windows::start_service()
}

#[cfg(unix)]
mod daemon;

#[cfg(unix)]
fn main() -> Result<(), Box<dyn Error>>
{
	let args = Args::parse();
	let pid_file = args.pid_file.clone();

	daemon::daemonize(&pid_file.clone(), move || {
		if let Err(e) = daemon_main(args)
		{
			error!("Error: {}", e);
		}

		if let Err(e) = daemon::cleanup(&pid_file)
		{
			error!("Failed to cleanup daemon with error: {}", e);
		};
	})?;

	Ok(())
}

fn daemon_main(args: Args) -> Result<(), Box<dyn Error>>
{
	if let Err(e) = setup_logging(Some(&args.log_file), None)
	{
		setup_logging(None, None)?;
		error!("Failed to open log file \"{}\"", args.log_file);
		return Err(e);
	};

	info!("Reading config file \"{}\"", args.config);
	match read_config(args.config)
	{
		Ok(config) =>
		{
			info!("Starting keep alive thread");
			let data = Arc::new(Mutex::new(HandlerData::new(config.left_matrix, config.right_matrix)));

			// Ask for the version every 45 seconds so that the matrixes don't timeout
			// This can be any command, but it shouldn't refresh/reset the LEDs as that defeats the purpose of this thread
			let keep_alive_data = Arc::clone(&data);
			spawn(move || {
				loop
				{
					let mut data = keep_alive_data.lock().unwrap();

					if !data.is_pair_animated()
					{
						if !data.is_left_animated()
						{
							if let Some(ref mut left_port) = &mut data.left_port
							{
								if !left_port.sleeping
								{
									let _ = version(left_port);
								}
							}
						}

						if !data.is_right_animated()
						{
							if let Some(ref mut right_port) = &mut data.right_port
							{
								if !right_port.sleeping
								{
									let _ = version(right_port);
								}
							}
						}
					}

					// Let the handler thread gain access to the mutex while we sleep
					drop(data);
					sleep(Duration::from_secs(45));
				}
			});

			// Listen on the IPC socket and handle requests
			listen("fw16-led-matrixd.socket".to_string(), handler, Arc::clone(&data))?;
		},
		Err(e) => return Err(e),
	};

	Ok(())
}
