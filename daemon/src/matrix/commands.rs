use crate::{matrix::open_port, Matrix};
use std::{
	error::Error,
	io::{Read, Write},
};

fn send_command(matrix: &Matrix,
                command: u8,
                parameters: Option<Vec<u8>>,
                response_size: usize)
                -> Result<Vec<u8>, Box<dyn Error>>
{
	let mut port = open_port(&matrix.port, matrix.baudrate)?;

	let request: Vec<u8> = match parameters
	{
		Some(mut p) =>
		{
			let mut c: Vec<u8> = vec![0x32, 0xAC, command];
			c.append(&mut p);
			c
		},
		None => vec![0x32, 0xAC, command],
	};

	port.write_all(&request)?;

	let mut response: Vec<u8> = vec![];
	if response_size > 0
	{
		response.resize(response_size, 0);
		port.read_exact(&mut response)?;
	}

	Ok(response)
}

pub fn set_brightness(matrix: &Matrix, brightness: u8) -> Result<(), Box<dyn Error>>
{
	send_command(matrix, 0x00, Some(vec![brightness]), 0)?;

	Ok(())
}

pub fn get_brightness(matrix: &Matrix) -> Result<u8, Box<dyn Error>>
{
	Ok(send_command(matrix, 0x00, None, 1)?[0])
}

pub fn pattern(matrix: &Matrix, pattern: u8, parameters: Option<u8>) -> Result<(), Box<dyn Error>>
{
	let parameters = match parameters
	{
		Some(p) => vec![pattern, p],
		None => vec![pattern],
	};

	send_command(matrix, 0x01, Some(parameters), 0)?;

	Ok(())
}

pub fn bootloader(matrix: &Matrix) -> Result<(), Box<dyn Error>>
{
	send_command(matrix, 0x02, None, 0)?;

	Ok(())
}

pub fn set_sleep(matrix: &Matrix, sleep: bool) -> Result<(), Box<dyn Error>>
{
	let parameters = match sleep
	{
		true => Some(vec![1u8]),
		false => Some(vec![0u8]),
	};

	send_command(matrix, 0x03, parameters, 0)?;

	Ok(())
}

// Useless command since it wakes up the matrix upon request
//pub fn is_sleeping(matrix: &Matrix) -> Result<bool, Box<dyn Error>>
//{
//	match send_command(matrix, 0x03, None, 1)?[0]
//	{
//		0 => Ok(false),
//		_ => Ok(true),
//	}
//}

pub fn set_scroll(matrix: &Matrix, scroll: bool) -> Result<(), Box<dyn Error>>
{
	let parameters = match scroll
	{
		true => Some(vec![1u8]),
		false => Some(vec![0u8]),
	};

	send_command(matrix, 0x04, parameters, 0)?;

	Ok(())
}

pub fn is_scrolling(matrix: &Matrix) -> Result<bool, Box<dyn Error>>
{
	match send_command(matrix, 0x04, None, 1)?[0]
	{
		0 => Ok(false),
		_ => Ok(true),
	}
}

pub fn crash(matrix: &Matrix) -> Result<(), Box<dyn Error>>
{
	send_command(matrix, 0x05, None, 0)?;

	Ok(())
}

pub fn draw_bw(matrix: &Matrix, bitmap: &[u8; 39]) -> Result<(), Box<dyn Error>>
{
	send_command(matrix, 0x06, Some(bitmap.to_vec()), 0)?;

	Ok(())
}

pub fn stage_column(matrix: &Matrix, column_number: u8, column_vals: &[u8; 34]) -> Result<(), Box<dyn Error>>
{
	let mut parameters: Vec<u8> = vec![column_number];
	parameters.append(&mut column_vals.to_vec());

	send_command(matrix, 0x07, Some(parameters), 0)?;

	Ok(())
}

pub fn flush_columns(matrix: &Matrix) -> Result<(), Box<dyn Error>>
{
	send_command(matrix, 0x08, None, 0)?;

	Ok(())
}

pub fn version(matrix: &Matrix) -> Result<Vec<u8>, Box<dyn Error>>
{
	send_command(matrix, 0x20, None, 3)
}
