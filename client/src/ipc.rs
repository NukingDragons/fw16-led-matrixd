use interprocess::local_socket::{prelude::*, GenericNamespaced, Stream};
use shared::ipc::*;
use std::{
	error::Error,
	io::{BufRead, BufReader, Write},
};

pub fn send_command(name: String,
                    command: IpcCommand,
                    collect_response: bool)
                    -> Result<Option<IpcResponse>, Box<dyn Error>>
{
	let socket_path = name.to_ns_name::<GenericNamespaced>()?;
	let conn = Stream::connect(socket_path)?;
	let mut conn = BufReader::new(conn);

	command.validate()?;
	conn.get_mut().write_all(command.to_json()?.as_bytes())?;

	let mut response = None;

	if collect_response
	{
		let mut buffer = String::new();
		conn.read_line(&mut buffer)?;

		response = Some(IpcResponse::from_json(buffer)?);
	}

	Ok(response)
}
