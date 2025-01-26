use nix::{
	libc::clearenv,
	sys::{
		resource::{getrlimit, Resource},
		signal::{signal, sigprocmask, SigHandler, SigSet, SigmaskHow, Signal},
		stat::{umask, Mode},
	},
	unistd::{close, dup2, fork, getpid, pipe, setsid, ForkResult},
};
use std::{
	env::set_current_dir,
	error::Error,
	fs::{remove_file, File},
	io::{Read, Write},
	os::fd::{AsRawFd, FromRawFd},
	process::exit,
};

// SysV compatible daemonize function
pub fn daemonize<F>(pid_file: &str, daemon_main: F) -> Result<(), Box<dyn Error>>
	where F: FnOnce()
{
	// Perform the main sequence for creating a sane daemon environment
	unsafe {
		// Step 1. Close all file descriptors that are not STDIN, STDOUT, or STDERR
		let (num_fds, _) = getrlimit(Resource::RLIMIT_NOFILE)?;
		if num_fds >= 3
		{
			for fd in 3..=num_fds
			{
				// fd might not exist
				let _ = close(fd as i32);
			}
		}

		// Step 2. Reset all signals to default
		for sig in Signal::iterator()
		{
			// Ignore SIGSTOP and SIGKILL since they can't be set
			if sig != Signal::SIGKILL && sig != Signal::SIGSTOP
			{
				signal(sig, SigHandler::SigDfl)?;
			}
		}

		// Step 3. Reset the signal mask
		sigprocmask(SigmaskHow::SIG_SETMASK, Some(&SigSet::empty()), None)?;

		// Step 4. Sanitize environment block
		if clearenv() != 0
		{
			println!("Failed to sanitize the environment variables");
			exit(1);
		}

		// Step 5. First fork
		let (read_fd, write_fd) = pipe()?;
		match fork()?
		{
			ForkResult::Parent { .. } =>
			{
				drop(write_fd);
				let mut read = File::from_raw_fd(read_fd.as_raw_fd());

				// Step 15. Kill the parent with the status sent over from step 14
				let mut response: String = Default::default();
				if read.read_to_string(&mut response)? != 0
				{
					println!("Failed to daemonize with error: {}", response);
					println!("Does \"{}\" still exist, or is \"{}\" in a writable location?", pid_file, pid_file);
					exit(1);
				}
				else
				{
					exit(0);
				}
			},
			ForkResult::Child =>
			{
				drop(read_fd);
				let mut write = File::from_raw_fd(write_fd.as_raw_fd());

				// Set the exit code for the parent based on the exit status of part2
				let response = match daemonize_part2(pid_file)
				{
					Ok(_) => Default::default(),
					Err(e) => e.to_string(),
				};

				// Step 14. Notify the parent that initialization is complete
				write.write_all(response.as_bytes())?;
				write.flush()?;
			},
		}

		daemon_main();

		Ok(())
	}
}

pub fn cleanup(pid_file: &str) -> Result<(), Box<dyn Error>>
{
	remove_file(pid_file)?;

	Ok(())
}

unsafe fn daemonize_part2(pid_file: &str) -> Result<(), Box<dyn Error>>
{
	// Step 6. Create a new session
	setsid()?;

	// Step 7. Second fork so that this child can get reparented by PID 1
	match fork()?
	{
		// Step 8. Kill the first child
		ForkResult::Parent { .. } => exit(0),
		ForkResult::Child => (),
	}

	// Step 9. Tie STDIN, STDOUT, and STDERR, to /dev/null
	let devnull = File::open("/dev/null")?;
	let devnull_fd = devnull.as_raw_fd();
	dup2(devnull_fd, 0)?;
	dup2(devnull_fd, 1)?;
	dup2(devnull_fd, 2)?;
	drop(devnull);

	// Step 10. Clear the umask
	umask(Mode::empty());

	// Step 11. Switch to the root directory
	set_current_dir("/")?;

	// Step 12. Race-free PID file
	let mut pid_file = File::create_new(pid_file)?;
	pid_file.write_all(format!("{}\n", getpid().as_raw()).as_bytes())?;
	drop(pid_file);

	// Step 13. Adjust privileges
	// This isn't done because root is needed to interact with the serial ports on some systems

	Ok(())
}
