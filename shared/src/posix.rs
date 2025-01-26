use crate::error::Error as CrateError;
use glob::glob;
use std::{error::Error, fs::read_to_string};

/// Get the VID and PID of the specified port
pub fn check_port(path: &str) -> Result<(u16, u16), Box<dyn Error>>
{
	let contents = if path.starts_with("/sys/class/tty/")
	{
		read_to_string(format!("/sys/class/tty/{}/device/uevent", path.split("/").collect::<Vec<_>>()[4]))?
	}
	else if path.starts_with("/dev/")
	{
		read_to_string(format!("/sys/class/tty/{}/device/uevent", path.split("/").collect::<Vec<_>>()[2]))?
	}
	else
	{
		return Err(Box::new(CrateError::UnknownPort(path.to_string())));
	};

	for line in contents.lines()
	{
		if line.starts_with("PRODUCT=")
		{
			let parts: Vec<_> = line.split("=").collect::<Vec<_>>()[1].split("/").collect();

			if parts.len() == 3
			{
				let vid = u16::from_str_radix(parts[0], 16)?;
				let pid = u16::from_str_radix(parts[1], 16)?;

				return Ok((vid, pid));
			}
		}
	}

	Err(Box::new(CrateError::NotUSB(path.to_string())))
}

/// Find serial ports with a given set of VIDs and PIDs
pub fn find_ports(vids: Vec<u16>, pids: Vec<u16>) -> Result<Vec<String>, Box<dyn Error>>
{
	let mut ports: Vec<String> = vec![];

	for path in glob("/sys/class/tty/*/device/uevent")?
	{
		if let Some(path) = path?.to_str()
		{
			match check_port(path)
			{
				Ok((vid, pid)) =>
				{
					if vids.contains(&vid) && pids.contains(&pid)
					{
						ports.push(format!("/dev/{}", path.split("/").collect::<Vec<_>>()[4]));
					}
				},
				Err(e) => match e.downcast_ref::<CrateError>()
				{
					Some(e) => match e
					{
						// Keep going if it's not a USB port
						CrateError::NotUSB(_) => (),
						e => return Err(Box::new(e.clone())),
					},
					None =>
					{
						// Any other error should be propogated up
						return Err(e);
					},
				},
			}
		}
	}

	Ok(ports)
}
