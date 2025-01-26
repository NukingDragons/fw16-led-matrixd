use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[cfg(windows)]
use windows_service::service::ServiceExitCode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error
{
	Config(String),
	NotUSB(String),
	UnknownPort(String),
	InvalidVecSize(String, usize),
	InvalidVecSizeFrame(String, usize, usize),
	InvalidColNumber(u8),
	Handler(String),
	#[cfg(windows)]
	WindowsError(i32),
}

impl std::error::Error for Error {}

impl Display for Error
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error>
	{
		match self
		{
			Self::Config(e) => write!(f, "failed to open config with error: {}", e),
			Self::NotUSB(port) => write!(f, "port \"{}\" is not a USB device", port),
			Self::UnknownPort(port) =>
			{
				write!(
				       f,
				       "cant determine port type for \"{}\", please use /dev/portname or /sys/class/tty/portname",
				       port
				)
			},
			Self::InvalidVecSize(matrix, vecsize) =>
			{
				write!(f, "{}an invalid vector size of {}", matrix, vecsize)
			},
			Self::InvalidVecSizeFrame(matrix, vecsize, index) =>
			{
				write!(f, "{}an invalid vector size of {} at index {}", matrix, vecsize, index)
			},
			Self::InvalidColNumber(col) => write!(f, "invalid column number {} (must be between 0 and 8)", col),
			Self::Handler(msg) => write!(f, "{}", msg),
			#[cfg(windows)]
			Self::WindowsError(status) => write!(f, "Win32 API error: {:x}", status),
		}
	}
}

#[cfg(windows)]
impl Into<ServiceExitCode> for Error
{
	fn into(self) -> ServiceExitCode
	{
		match self
		{
			Self::Config(_) => ServiceExitCode::ServiceSpecific(0xDEAD0001),
			Self::NotUSB(_) => ServiceExitCode::ServiceSpecific(0xDEAD0002),
			Self::UnknownPort(_) => ServiceExitCode::ServiceSpecific(0xDEAD0003),
			Self::InvalidVecSize(_, _) => ServiceExitCode::ServiceSpecific(0xDEAD0004),
			Self::InvalidVecSizeFrame(_, _, _) => ServiceExitCode::ServiceSpecific(0xDEAD0005),
			Self::InvalidColNumber(_) => ServiceExitCode::ServiceSpecific(0xDEAD0006),
			Self::Handler(_) => ServiceExitCode::ServiceSpecific(0xDEAD0007),
			Self::WindowsError(e) => ServiceExitCode::Win32(e as u32),
		}
	}
}
