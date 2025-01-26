mod commands;
mod error;

use crate::Matrix;
use error::MatrixError;
use serial::{open, BaudRate, SerialPort};
use shared::check_port;
use std::{error::Error, time::Duration};

pub use commands::*;

fn open_port(port_name: &str, baudrate: usize) -> Result<impl SerialPort, Box<dyn Error>>
{
	let (vid, pid) = check_port(port_name)?;

	if vid != 0x32AC && (pid != 0x20 && pid != 0x1F)
	{
		Err(Box::new(MatrixError::InvalidPort(port_name.to_string())))
	}
	else
	{
		let mut port = open(&port_name)?;

		// Serial config for the FW16 LED Serial
		port.reconfigure(&|settings| {
			    settings.set_baud_rate(BaudRate::from_speed(baudrate))?;
			    settings.set_char_size(serial::Bits8);
			    settings.set_parity(serial::ParityNone);
			    settings.set_stop_bits(serial::Stop1);
			    settings.set_flow_control(serial::FlowNone);
			    Ok(())
		    })?;

		port.set_timeout(Duration::from_millis(1000))?;

		Ok(port)
	}
}

pub fn render_percentage(matrix: &Matrix, percentage: u8) -> Result<(), Box<dyn Error>>
{
	pattern(matrix, 0x00, Some(percentage))
}

pub fn render_gradient(matrix: &Matrix) -> Result<(), Box<dyn Error>>
{
	pattern(matrix, 0x01, None)
}

pub fn render_double_gradient(matrix: &Matrix) -> Result<(), Box<dyn Error>>
{
	pattern(matrix, 0x02, None)
}

pub fn render_lotus_horizontal(matrix: &Matrix) -> Result<(), Box<dyn Error>>
{
	pattern(matrix, 0x03, None)
}

pub fn render_zigzag(matrix: &Matrix) -> Result<(), Box<dyn Error>>
{
	pattern(matrix, 0x04, None)
}

pub fn render_fullbright(matrix: &Matrix) -> Result<(), Box<dyn Error>>
{
	pattern(matrix, 0x05, None)
}

pub fn render_panic(matrix: &Matrix) -> Result<(), Box<dyn Error>>
{
	pattern(matrix, 0x06, None)
}

pub fn render_lotus_vertical(matrix: &Matrix) -> Result<(), Box<dyn Error>>
{
	pattern(matrix, 0x07, None)
}

pub fn render_single(matrix: &Matrix, frame: &[u8; 306]) -> Result<(), Box<dyn Error>>
{
	stage_column(matrix, 0, <&[u8; 34]>::try_from(&frame[0..=33])?)?;
	stage_column(matrix, 1, <&[u8; 34]>::try_from(&frame[34..=67])?)?;
	stage_column(matrix, 2, <&[u8; 34]>::try_from(&frame[68..=101])?)?;
	stage_column(matrix, 3, <&[u8; 34]>::try_from(&frame[102..=135])?)?;
	stage_column(matrix, 4, <&[u8; 34]>::try_from(&frame[136..=169])?)?;
	stage_column(matrix, 5, <&[u8; 34]>::try_from(&frame[170..=203])?)?;
	stage_column(matrix, 6, <&[u8; 34]>::try_from(&frame[204..=237])?)?;
	stage_column(matrix, 7, <&[u8; 34]>::try_from(&frame[238..=271])?)?;
	stage_column(matrix, 8, <&[u8; 34]>::try_from(&frame[272..=305])?)?;

	flush_columns(matrix)?;
	Ok(())
}

pub fn render_pair(left_matrix: &Matrix, right_matrix: &Matrix, frame: &[u8; 612]) -> Result<(), Box<dyn Error>>
{
	stage_column(left_matrix, 0, <&[u8; 34]>::try_from(&frame[0..=33])?)?;
	stage_column(left_matrix, 1, <&[u8; 34]>::try_from(&frame[34..=67])?)?;
	stage_column(left_matrix, 2, <&[u8; 34]>::try_from(&frame[68..=101])?)?;
	stage_column(left_matrix, 3, <&[u8; 34]>::try_from(&frame[102..=135])?)?;
	stage_column(left_matrix, 4, <&[u8; 34]>::try_from(&frame[136..=169])?)?;
	stage_column(left_matrix, 5, <&[u8; 34]>::try_from(&frame[170..=203])?)?;
	stage_column(left_matrix, 6, <&[u8; 34]>::try_from(&frame[204..=237])?)?;
	stage_column(left_matrix, 7, <&[u8; 34]>::try_from(&frame[238..=271])?)?;
	stage_column(left_matrix, 8, <&[u8; 34]>::try_from(&frame[272..=305])?)?;

	stage_column(right_matrix, 0, <&[u8; 34]>::try_from(&frame[306..=339])?)?;
	stage_column(right_matrix, 1, <&[u8; 34]>::try_from(&frame[340..=373])?)?;
	stage_column(right_matrix, 2, <&[u8; 34]>::try_from(&frame[374..=407])?)?;
	stage_column(right_matrix, 3, <&[u8; 34]>::try_from(&frame[408..=441])?)?;
	stage_column(right_matrix, 4, <&[u8; 34]>::try_from(&frame[442..=475])?)?;
	stage_column(right_matrix, 5, <&[u8; 34]>::try_from(&frame[476..=509])?)?;
	stage_column(right_matrix, 6, <&[u8; 34]>::try_from(&frame[510..=543])?)?;
	stage_column(right_matrix, 7, <&[u8; 34]>::try_from(&frame[544..=577])?)?;
	stage_column(right_matrix, 8, <&[u8; 34]>::try_from(&frame[578..=611])?)?;

	flush_columns(left_matrix)?;
	flush_columns(right_matrix)?;
	Ok(())
}
