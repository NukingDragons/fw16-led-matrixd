use crate::handler::HandlerData;
use interprocess::local_socket::{prelude::*, GenericNamespaced, ListenerOptions};
use log::warn;
use shared::{error::Error as SharedError, ipc::*};
use std::{
	error::Error,
	io::{BufRead, BufReader, Write},
	sync::{Arc, Mutex},
};

pub fn listen<F>(name: String, mut handler: F, data: Arc<Mutex<HandlerData>>) -> Result<(), Box<dyn Error>>
	where F: FnMut(IpcCommand, &mut Option<IpcResponse>, Arc<Mutex<HandlerData>>) -> Result<bool, Box<dyn Error>>
{
	let socket_path = name.to_ns_name::<GenericNamespaced>()?;
	let opts = ListenerOptions::new().name(socket_path);
	let listener = opts.create_sync()?;

	for conn in listener.incoming().filter_map(|conn| match conn
	                               {
		                               Ok(c) => Some(c),
	                                   Err(e) =>
	                                   {
		                                   warn!("Incoming connection on the IPC socket failed with error: {}", e);
		                                   None
	                                   },
	                               })
	{
		let mut buffer = String::new();
		let mut conn = BufReader::new(conn);
		conn.read_line(&mut buffer)?;

		let command = IpcCommand::from_json(buffer)?;

		let mut response: Option<IpcResponse> = None;
		match command.validate()
		{
			Ok(_) => match handler(command, &mut response, Arc::clone(&data))
			{
				Ok(close) =>
				{
					if close
					{
						break;
					}
				},
				Err(e) =>
				{
					response = Some(IpcResponse::InvalidCommand(SharedError::Handler(e.to_string())));
				},
			},
			Err(e) =>
			{
				response = Some(IpcResponse::InvalidCommand(e));
			},
		}

		if let Some(r) = response
		{
			conn.get_mut().write_all(r.to_json()?.as_bytes())?;
		}
	}

	Ok(())
}
