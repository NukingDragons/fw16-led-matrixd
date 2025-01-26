use crate::{matrix::*, Matrix};
use log::{error, info, warn};
use shared::ipc::*;
use std::{
	error::Error,
	sync::{Arc, Mutex},
	thread::{sleep, spawn},
};

pub struct HandlerData
{
	pub left_port: Option<Matrix>,
	pub right_port: Option<Matrix>,
	left_thread: Option<Arc<()>>,
	right_thread: Option<Arc<()>>,
	pair_thread: Option<Arc<()>>,
}

impl HandlerData
{
	pub fn new(left_port: Option<Matrix>, right_port: Option<Matrix>) -> Self
	{
		HandlerData { left_port, right_port, left_thread: None, right_thread: None, pair_thread: None }
	}

	pub fn get_ports(&mut self) -> (&mut Option<Matrix>, &mut Option<Matrix>)
	{
		(&mut self.left_port, &mut self.right_port)
	}

	pub fn kill_threads(&mut self, keep_left_alive: bool, keep_right_alive: bool)
	{
		self.pair_thread = None;

		if !keep_left_alive
		{
			self.left_thread = None;
		}

		if !keep_right_alive
		{
			self.right_thread = None;
		}
	}

	pub fn start_pair_thread<F: FnMut() + Send + 'static>(&mut self, mut worker: F)
	{
		let arc = Arc::new(());
		self.pair_thread = Some(Arc::clone(&arc));

		let clone = Arc::clone(&arc);
		spawn(move || {
			while Arc::strong_count(&clone) > 1
			{
				worker();
			}
		});
	}

	pub fn start_left_thread<F: FnMut() + Send + 'static>(&mut self, mut worker: F)
	{
		let arc = Arc::new(());
		self.left_thread = Some(Arc::clone(&arc));

		let clone = Arc::clone(&arc);
		spawn(move || {
			while Arc::strong_count(&clone) > 1
			{
				worker();
			}
		});
	}

	pub fn start_right_thread<F: FnMut() + Send + 'static>(&mut self, mut worker: F)
	{
		let arc = Arc::new(());
		self.right_thread = Some(Arc::clone(&arc));

		let clone = Arc::clone(&arc);
		spawn(move || {
			while Arc::strong_count(&clone) > 1
			{
				worker();
			}
		});
	}

	pub fn is_left_animated(&self) -> bool
	{
		self.left_thread.is_some()
	}

	pub fn is_right_animated(&self) -> bool
	{
		self.right_thread.is_some()
	}

	pub fn is_pair_animated(&self) -> bool
	{
		self.pair_thread.is_some()
	}
}

macro_rules! send_left_cmd_r {
	($data:expr, $command:expr) => {
		if let Some(ref mut left_port) = &mut $data.left_port
		{
			left_port.sleeping = false;
			$command(left_port)?
		}
		else
		{
			warn!("Can't send a command on the left matrix since the left matrix hasn't been defined");
			None
		}
	};
}

macro_rules! send_right_cmd_r {
	($data:expr, $command:expr) => {
		if let Some(ref mut right_port) = &mut $data.right_port
		{
			right_port.sleeping = false;
			$command(right_port)?
		}
		else
		{
			warn!("Can't send a command on the right matrix since the right matrix hasn't been defined");
			None
		}
	};
}

macro_rules! send_left_cmd {
	($data:expr, $command:expr) => {
		if let Some(ref mut left_port) = &mut $data.left_port
		{
			left_port.sleeping = false;
			$command(left_port)?;
		}
		else
		{
			warn!("Can't send a command on the left matrix since the left matrix hasn't been defined");
		}
	};
}

macro_rules! send_right_cmd {
	($data:expr, $command:expr) => {
		if let Some(ref mut right_port) = &mut $data.right_port
		{
			right_port.sleeping = false;
			$command(right_port)?;
		}
		else
		{
			warn!("Can't send a command on the right matrix since the right matrix hasn't been defined");
		}
	};
}

macro_rules! send_pair_cmd {
	($data:expr, $command:expr) => {
		if let (Some(ref mut left_port), Some(ref mut right_port)) = $data.get_ports()
		{
			left_port.sleeping = false;
			right_port.sleeping = false;
			$command(left_port, right_port)?;
		}
		else
		{
			warn!("Can't send a pair command due to missing at least one matrix from the config");
		}
	};
}

pub fn handler(command: IpcCommand,
               response: &mut Option<IpcResponse>,
               handler_data: Arc<Mutex<HandlerData>>)
               -> Result<bool, Box<dyn Error>>
{
	let mut data = handler_data.lock().unwrap();

	match command
	{
		// Commands the require responses
		// GetBrightness doesn't need to kill render threads
		IpcCommand::GetBrightness(left, right) =>
		{
			let mut left_res: Option<u8> = None;
			let mut right_res: Option<u8> = None;

			if left
			{
				left_res = send_left_cmd_r!(data, |left_port| -> Result<Option<u8>, Box<dyn Error>> {
					Ok(Some(get_brightness(left_port)?))
				});
			}

			if right
			{
				right_res = send_right_cmd_r!(data, |right_port| -> Result<Option<u8>, Box<dyn Error>> {
					Ok(Some(get_brightness(right_port)?))
				});
			}

			*response = Some(IpcResponse::Brightness(left_res, right_res));
		},
		// GetSleep doesn't need to kill render threads
		// Asking a sleeping matrix will wake it up. So just use the Matrix sleeping bool instead
		IpcCommand::GetSleep(left, right) =>
		{
			let mut left_res: Option<bool> = None;
			let mut right_res: Option<bool> = None;

			if left
			{
				left_res = send_left_cmd_r!(data, |left_port: &Matrix| -> Result<Option<bool>, Box<dyn Error>> {
					//Ok(Some(is_sleeping(left_port)?))
					Ok(Some(left_port.sleeping))
				});
			}

			if right
			{
				right_res = send_right_cmd_r!(data, |right_port: &Matrix| -> Result<Option<bool>, Box<dyn Error>> {
					//Ok(Some(is_sleeping(right_port)?))
					Ok(Some(right_port.sleeping))
				});
			}

			*response = Some(IpcResponse::Sleeping(left_res, right_res));
		},
		// GetAnimate doesn't need to kill render threads
		IpcCommand::GetAnimate(left, right) =>
		{
			let mut left_res: Option<bool> = None;
			let mut right_res: Option<bool> = None;

			if left
			{
				left_res = send_left_cmd_r!(data, |left_port| -> Result<Option<bool>, Box<dyn Error>> {
					Ok(Some(is_scrolling(left_port)?))
				});
			}

			if right
			{
				right_res = send_right_cmd_r!(data, |right_port| -> Result<Option<bool>, Box<dyn Error>> {
					Ok(Some(is_scrolling(right_port)?))
				});
			}

			*response = Some(IpcResponse::Animated(left_res, right_res));
		},
		// Version doesn't need to kill render threads
		IpcCommand::Version(left, right) =>
		{
			let mut left_res: Option<Vec<u8>> = None;
			let mut right_res: Option<Vec<u8>> = None;

			if left
			{
				left_res = send_left_cmd_r!(data, |left_port| -> Result<Option<Vec<u8>>, Box<dyn Error>> {
					Ok(Some(version(left_port)?))
				});
			}

			if right
			{
				right_res = send_right_cmd_r!(data, |right_port| -> Result<Option<Vec<u8>>, Box<dyn Error>> {
					Ok(Some(version(right_port)?))
				});
			}

			*response = Some(IpcResponse::Version(left_res, right_res));
		},
		// Commands that don't need responses
		// Brightness doesn't need to kill render threads
		IpcCommand::SetBrightness(left, right) =>
		{
			if let Some(b) = left
			{
				send_left_cmd!(data, |left_port| -> Result<(), Box<dyn Error>> { set_brightness(left_port, b) });
			}

			if let Some(b) = right
			{
				send_right_cmd!(data, |right_port| -> Result<(), Box<dyn Error>> { set_brightness(right_port, b) });
			}
		},
		// Also update the Matrix sleeping bool so that sleeping works as intended
		IpcCommand::SetSleep(left, right) =>
		{
			if let Some(s) = left
			{
				data.kill_threads(false, true);
				send_left_cmd!(data, |left_port: &mut Matrix| -> Result<(), Box<dyn Error>> {
					left_port.sleeping = s;
					set_sleep(left_port, s)
				});
			}

			if let Some(s) = right
			{
				data.kill_threads(true, false);
				send_right_cmd!(data, |right_port: &mut Matrix| -> Result<(), Box<dyn Error>> {
					right_port.sleeping = s;
					set_sleep(right_port, s)
				});
			}
		},
		IpcCommand::SetAnimate(left, right) =>
		{
			if let Some(s) = left
			{
				data.kill_threads(false, true);
				send_left_cmd!(data, |left_port| -> Result<(), Box<dyn Error>> { set_scroll(left_port, s) });
			}

			if let Some(s) = right
			{
				data.kill_threads(true, false);
				send_right_cmd!(data, |right_port| -> Result<(), Box<dyn Error>> { set_scroll(right_port, s) });
			}
		},
		IpcCommand::PatternPercentage(left, right) =>
		{
			if let Some(p) = left
			{
				data.kill_threads(false, true);
				send_left_cmd!(data, |left_port| -> Result<(), Box<dyn Error>> { render_percentage(left_port, p) });
			}

			if let Some(p) = right
			{
				data.kill_threads(true, false);
				send_right_cmd!(data, |right_port| -> Result<(), Box<dyn Error>> { render_percentage(right_port, p) });
			}
		},
		IpcCommand::PatternGradient(left, right) =>
		{
			data.kill_threads(!left, !right);

			if left
			{
				send_left_cmd!(data, |left_port| -> Result<(), Box<dyn Error>> { render_gradient(left_port) });
			}

			if right
			{
				send_right_cmd!(data, |right_port| -> Result<(), Box<dyn Error>> { render_gradient(right_port) });
			}
		},
		IpcCommand::PatternDoubleGradient(left, right) =>
		{
			data.kill_threads(!left, !right);

			if left
			{
				send_left_cmd!(data, |left_port| -> Result<(), Box<dyn Error>> { render_double_gradient(left_port) });
			}

			if right
			{
				send_right_cmd!(data, |right_port| -> Result<(), Box<dyn Error>> {
					render_double_gradient(right_port)
				});
			}
		},
		IpcCommand::PatternLotusHorizontal(left, right) =>
		{
			data.kill_threads(!left, !right);

			if left
			{
				send_left_cmd!(data, |left_port| -> Result<(), Box<dyn Error>> { render_lotus_horizontal(left_port) });
			}

			if right
			{
				send_right_cmd!(data, |right_port| -> Result<(), Box<dyn Error>> {
					render_lotus_horizontal(right_port)
				});
			}
		},
		IpcCommand::PatternLotusVertical(left, right) =>
		{
			data.kill_threads(!left, !right);

			if left
			{
				send_left_cmd!(data, |left_port| -> Result<(), Box<dyn Error>> { render_lotus_vertical(left_port) });
			}

			if right
			{
				send_right_cmd!(data, |right_port| -> Result<(), Box<dyn Error>> { render_lotus_vertical(right_port) });
			}
		},
		IpcCommand::PatternZigzag(left, right) =>
		{
			data.kill_threads(!left, !right);

			if left
			{
				send_left_cmd!(data, |left_port| -> Result<(), Box<dyn Error>> { render_zigzag(left_port) });
			}

			if right
			{
				send_right_cmd!(data, |right_port| -> Result<(), Box<dyn Error>> { render_zigzag(right_port) });
			}
		},
		IpcCommand::PatternFullbright(left, right) =>
		{
			data.kill_threads(!left, !right);

			if left
			{
				send_left_cmd!(data, |left_port| -> Result<(), Box<dyn Error>> { render_fullbright(left_port) });
			}

			if right
			{
				send_right_cmd!(data, |right_port| -> Result<(), Box<dyn Error>> { render_fullbright(right_port) });
			}
		},
		IpcCommand::PatternPanic(left, right) =>
		{
			data.kill_threads(!left, !right);

			if left
			{
				send_left_cmd!(data, |left_port| -> Result<(), Box<dyn Error>> { render_panic(left_port) });
			}

			if right
			{
				send_right_cmd!(data, |right_port| -> Result<(), Box<dyn Error>> { render_panic(right_port) });
			}
		},
		IpcCommand::Bootloader(left, right) =>
		{
			data.kill_threads(!left, !right);

			if left
			{
				send_left_cmd!(data, |left_port| -> Result<(), Box<dyn Error>> { bootloader(left_port) });
			}

			if right
			{
				send_right_cmd!(data, |right_port| -> Result<(), Box<dyn Error>> { bootloader(right_port) });
			}
		},
		IpcCommand::Crash(left, right) =>
		{
			data.kill_threads(!left, !right);

			if left
			{
				send_left_cmd!(data, |left_port| -> Result<(), Box<dyn Error>> { crash(left_port) });
			}

			if right
			{
				send_right_cmd!(data, |right_port| -> Result<(), Box<dyn Error>> { crash(right_port) });
			}
		},
		IpcCommand::DrawBW(left, right) =>
		{
			if let Some(bw) = left
			{
				data.kill_threads(false, true);

				// The validate function ensures this won't fail here
				let bw: [u8; 39] = bw.try_into().unwrap();
				send_left_cmd!(data, |left_port| -> Result<(), Box<dyn Error>> { draw_bw(left_port, &bw) });
			}

			if let Some(bw) = right
			{
				data.kill_threads(true, false);

				// The validate function ensures this won't fail here
				let bw: [u8; 39] = bw.try_into().unwrap();
				send_right_cmd!(data, |right_port| -> Result<(), Box<dyn Error>> { draw_bw(right_port, &bw) });
			}
		},
		IpcCommand::StageCol(left, right) =>
		{
			if let Some((col, vals)) = left
			{
				// The validate function ensures this won't fail here
				let vals: [u8; 34] = vals.try_into().unwrap();
				send_left_cmd!(data, |left_port| -> Result<(), Box<dyn Error>> { stage_column(left_port, col, &vals) });
			}

			if let Some((col, vals)) = right
			{
				// The validate function ensures this won't fail here
				let vals: [u8; 34] = vals.try_into().unwrap();
				send_right_cmd!(data, |right_port| -> Result<(), Box<dyn Error>> {
					stage_column(right_port, col, &vals)
				});
			}
		},
		IpcCommand::FlushCols(left, right) =>
		{
			data.kill_threads(!left, !right);

			if left
			{
				send_left_cmd!(data, |left_port| -> Result<(), Box<dyn Error>> { flush_columns(left_port) });
			}

			if right
			{
				send_right_cmd!(data, |right_port| -> Result<(), Box<dyn Error>> { flush_columns(right_port) });
			}
		},
		IpcCommand::RenderSingle(left, right) =>
		{
			if let Some(s) = left
			{
				data.kill_threads(false, true);
				if s.len() == 1
				{
					// The validate function ensures this won't fail here
					let s: [u8; 306] = s[0].0.clone().try_into().unwrap();
					send_left_cmd!(data, |left_port| -> Result<(), Box<dyn Error>> { render_single(left_port, &s) });
				}
				else
				{
					info!("Starting up left thread");
					let data_two = Arc::clone(&handler_data);
					data.start_left_thread(move || {
						    // Render left
						    for (frame, duration) in &s
						    {
							    let mut data = data_two.lock().unwrap();

							    // We got the lock, but don't render if we shouldn't be animated anymore
							    if data.is_left_animated()
							    {
								    // The validate function ensures this won't fail here
								    let s: [u8; 306] = frame.clone().try_into().unwrap();
								    if let Some(ref mut left_port) = &mut data.left_port
								    {
									    if let Err(e) = render_single(left_port, &s)
									    {
										    error!("Left render thread failed with error: {}", e);
										    data.kill_threads(true, false);
										    break;
									    }
								    }
								    else
								    {
									    warn!("Can't send a command on the left matrix since the left matrix hasn't been defined");
									    data.kill_threads(true, false);
									    break;
								    }
							    }
							    else
							    {
								    warn!("Terminating left render thread early");
								    data.kill_threads(true, false);
								    break;
							    }

							    drop(data);
							    sleep(*duration);
						    }
					    });
				}
			}

			if let Some(s) = right
			{
				data.kill_threads(true, false);
				if s.len() == 1
				{
					// The validate function ensures this won't fail here
					let s: [u8; 306] = s[0].0.clone().try_into().unwrap();
					send_right_cmd!(data, |right_port| -> Result<(), Box<dyn Error>> { render_single(right_port, &s) });
				}
				else
				{
					info!("Starting up right thread");
					let data_two = Arc::clone(&handler_data);
					data.start_right_thread(move || {
						    // Render right
						    for (frame, duration) in &s
						    {
							    let mut data = data_two.lock().unwrap();

							    // We got the lock, but don't render if we shouldn't be animated anymore
							    if data.is_right_animated()
							    {
								    // The validate function ensures this won't fail here
								    let s: [u8; 306] = frame.clone().try_into().unwrap();
								    if let Some(ref mut right_port) = &mut data.right_port
								    {
									    if let Err(e) = render_single(right_port, &s)
									    {
										    error!("Right render thread failed with error: {}", e);
										    data.kill_threads(true, false);
										    break;
									    }
								    }
								    else
								    {
									    warn!("Can't send a command on the right matrix since the right matrix hasn't been defined");
									    data.kill_threads(true, false);
									    break;
								    }
							    }
							    else
							    {
								    warn!("Terminating right render thread early");
								    data.kill_threads(true, false);
								    break;
							    }

							    drop(data);
							    sleep(*duration);
						    }
					    });
				}
			}
		},
		IpcCommand::RenderPair(pair) =>
		{
			data.kill_threads(false, false);

			if pair.len() == 1
			{
				// The validate function ensures this won't fail here
				let pair: [u8; 612] = pair[0].0.clone().try_into().unwrap();
				send_pair_cmd!(data, |left_port, right_port| -> Result<(), Box<dyn Error>> {
					render_pair(left_port, right_port, &pair)
				});
			}
			else
			{
				info!("Starting up pair thread");
				let data_two = Arc::clone(&handler_data);
				data.start_pair_thread(move || {
					    // Render pair
					    for (frame, duration) in &pair
					    {
						    let mut data = data_two.lock().unwrap();

						    // We got the lock, but don't render if we shouldn't be animated anymore
						    if data.is_pair_animated()
						    {
							    // The validate function ensures this won't fail here
							    let pair: [u8; 612] = frame.clone().try_into().unwrap();
							    if let (Some(ref mut left_port), Some(ref mut right_port)) = data.get_ports()
							    {
								    if let Err(e) = render_pair(left_port, right_port, &pair)
								    {
									    error!("Pair render thread failed with error: {}", e);
									    data.kill_threads(false, false);
									    break;
								    }
							    }
							    else
							    {
								    warn!("Can't send a pair command due to missing at least one matrix from the config");
								    data.kill_threads(false, false);
								    break;
							    }
						    }
						    else
						    {
							    warn!("Terminating pair render thread early");
							    data.kill_threads(false, false);
							    break;
						    }

						    drop(data);
						    sleep(*duration);
					    }
				    });
			}
		},
	}

	Ok(false)
}
