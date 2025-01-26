use std::fmt::{self, Display, Formatter};

pub struct Version
{
	pub major: u8,
	pub minor: u8,
	pub patch: u8,
	pub pre_release: bool,
}

impl From<[u8; 3]> for Version
{
	fn from(a: [u8; 3]) -> Version
	{
		Version { major: a[0], minor: (a[1] & 0xF0) >> 4, patch: a[1] & 0x0F, pre_release: !matches!(a[2], 0) }
	}
}

impl TryFrom<Vec<u8>> for Version
{
	type Error = Box<dyn std::error::Error>;

	fn try_from(v: Vec<u8>) -> Result<Version, Self::Error>
	{
		let a: [u8; 3] = v[..].try_into()?;

		Ok(a.into())
	}
}

impl TryFrom<&[u8]> for Version
{
	type Error = Box<dyn std::error::Error>;

	fn try_from(a: &[u8]) -> Result<Version, Self::Error>
	{
		a.to_vec().try_into()
	}
}

impl Display for Version
{
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error>
	{
		if self.pre_release
		{
			write!(f, "v{}.{}.{}-pre", self.major, self.minor, self.patch)
		}
		else
		{
			write!(f, "v{}.{}.{}", self.major, self.minor, self.patch)
		}
	}
}
