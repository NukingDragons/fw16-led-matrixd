use crate::error::Error as CrateError;
use std::{error::Error, ffi::CString, ptr::null_mut};
use winapi::{
	shared::minwindef::HKEY,
	um::winreg::{RegCloseKey, RegEnumKeyExA, RegEnumValueA, RegOpenKeyA, RegQueryValueExA, HKEY_LOCAL_MACHINE},
};

macro_rules! wintry {
	($command:expr) => {
		let status = $command;
		if status != 0
		{
			return Err(Box::new(CrateError::WindowsError(status)));
		}
	};
}

fn enum_subkeys(subkey: &str) -> Result<Vec<String>, Box<dyn Error>>
{
	let mut result: Vec<String> = vec![];

	unsafe {
		let mut enum_key: HKEY = null_mut();
		wintry!(RegOpenKeyA(HKEY_LOCAL_MACHINE, CString::new(subkey)?.as_ptr(), &mut enum_key));

		let mut index: u32 = 0;
		loop
		{
			// I doubt any name will be larger than 4096 bytes
			let mut raw_key_name: Vec<u8> = vec![0; 4096];
			let mut key_name_size: u32 = 4096;
			match RegEnumKeyExA(
			                    enum_key,
			                    index,
			                    raw_key_name.as_mut_ptr() as *mut i8,
			                    &mut key_name_size,
			                    null_mut(),
			                    null_mut(),
			                    null_mut(),
			                    null_mut(),
			)
			{
				// ERROR_NO_MORE_ITEMS
				259 => break,
				// raw_key_name is populated
				0 =>
				{
					// Remove the excess null bytes and convert into a string
					raw_key_name.resize(key_name_size as usize + 1, 0);
					result.push(CString::from_vec_with_nul_unchecked(raw_key_name).to_str()?.to_string());
				},
				// Something went wrong
				status =>
				{
					wintry!(RegCloseKey(enum_key));
					return Err(Box::new(CrateError::WindowsError(status)));
				},
			}

			index += 1;
		}

		wintry!(RegCloseKey(enum_key));
	}

	Ok(result)
}

fn enum_subvals(subkey: &str) -> Result<Vec<String>, Box<dyn Error>>
{
	let mut result: Vec<String> = vec![];

	unsafe {
		let mut enum_key: HKEY = null_mut();
		wintry!(RegOpenKeyA(HKEY_LOCAL_MACHINE, CString::new(subkey)?.as_ptr(), &mut enum_key));

		let mut index: u32 = 0;
		loop
		{
			// I doubt any name will be larger than 4096 bytes
			let mut raw_key_name: Vec<u8> = vec![0; 4096];
			let mut key_name_size: u32 = 4096;

			// I doubt any value name will be larger than 4096 bytes
			let mut raw_value: Vec<u8> = vec![0; 4096];
			let mut value_size: u32 = 4096;
			match RegEnumValueA(
			                    enum_key,
			                    index,
			                    raw_key_name.as_mut_ptr() as *mut i8,
			                    &mut key_name_size,
			                    null_mut(),
			                    null_mut(),
			                    raw_value.as_mut_ptr(),
			                    &mut value_size,
			)
			{
				// ERROR_NO_MORE_ITEMS
				259 => break,
				// raw_key_name is populated
				0 =>
				{
					// Remove the excess null bytes and convert into a string
					raw_value.resize(value_size as usize, 0);
					result.push(CString::from_vec_with_nul_unchecked(raw_value).to_str()?.to_string());
				},
				// Something went wrong
				status =>
				{
					wintry!(RegCloseKey(enum_key));
					return Err(Box::new(CrateError::WindowsError(status)));
				},
			}

			index += 1;
		}

		wintry!(RegCloseKey(enum_key));
	}

	Ok(result)
}

fn get_port_names(vid_pid: &str) -> Result<Vec<String>, Box<dyn Error>>
{
	let mut port_names: Vec<String> = vec![];

	for subkey in enum_subkeys(&format!("SYSTEM\\CurrentControlSet\\Enum\\USB\\{}", vid_pid))?
	{
		unsafe {
			let mut port_name_key: HKEY = null_mut();
			wintry!(RegOpenKeyA(HKEY_LOCAL_MACHINE, CString::new(format!("SYSTEM\\CurrentControlSet\\Enum\\USB\\{}\\{}\\Device Parameters", vid_pid, subkey))?.as_ptr(), &mut port_name_key));

			// I doubt any port name will be larger than 4096 bytes
			let mut raw_port_name: Vec<u8> = vec![0; 4096];
			let mut port_name_size: u32 = 4096;
			match RegQueryValueExA(
			                       port_name_key,
			                       CString::new("PortName")?.as_ptr(),
			                       null_mut(),
			                       null_mut(),
			                       raw_port_name.as_mut_ptr(),
			                       &mut port_name_size,
			)
			{
				// Not all USBs have port names
				// ERROR_FILE_NOT_FOUND
				2 => (),
				0 =>
				{
					raw_port_name.resize(port_name_size as usize, 0);
					let name = CString::from_vec_with_nul_unchecked(raw_port_name).to_str()?.to_string();

					// Make sure the name found is actually connected
					let connected_names = enum_subvals("HARDWARE\\DEVICEMAP\\SERIALCOMM")?;
					if connected_names.contains(&name)
					{
						port_names.push(name);
					}
				},
				status =>
				{
					wintry!(RegCloseKey(port_name_key));
					return Err(Box::new(CrateError::WindowsError(status)));
				},
			}

			wintry!(RegCloseKey(port_name_key));
		}
	}

	Ok(port_names)
}

/// Get the VID and PID of the specified port
pub fn check_port(port_name: &str) -> Result<(u16, u16), Box<dyn Error>>
{
	for subkey in enum_subkeys("SYSTEM\\CurrentControlSet\\Enum\\USB")?
	{
		if subkey.starts_with("VID_")
		{
			let vid = u16::from_str_radix(&subkey[4..=7], 16)?;
			let pid = u16::from_str_radix(&subkey[13..=16], 16)?;

			for name in get_port_names(&subkey)?
			{
				if name == port_name
				{
					return Ok((vid, pid));
				}
			}
		}
	}

	Err(Box::new(CrateError::NotUSB(port_name.to_string())))
}

/// Find serial ports with a given set of VIDs and PIDs
pub fn find_ports(vids: Vec<u16>, pids: Vec<u16>) -> Result<Vec<String>, Box<dyn Error>>
{
	let mut ports: Vec<String> = vec![];

	for subkey in enum_subkeys("SYSTEM\\CurrentControlSet\\Enum\\USB")?
	{
		if subkey.starts_with("VID_")
		{
			let vid = u16::from_str_radix(&subkey[4..=7], 16)?;
			let pid = u16::from_str_radix(&subkey[13..=16], 16)?;

			if vids.contains(&vid) && pids.contains(&pid)
			{
				for name in get_port_names(&subkey)?
				{
					ports.push(name);
				}
			}
		}
	}

	Ok(ports)
}
