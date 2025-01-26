pub mod error;
pub mod ipc;
pub mod version;

#[cfg(unix)]
mod posix;

#[cfg(unix)]
pub use posix::{check_port, find_ports};

#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use windows::{check_port, find_ports};
