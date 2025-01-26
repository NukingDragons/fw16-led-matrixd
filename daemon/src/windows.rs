use crate::{daemon_main, Args};
use clap::Parser;
use shared::error::Error as SharedError;
use std::{
	ffi::OsString,
	sync::mpsc::channel,
	thread::{sleep, spawn},
	time::Duration,
};
use windows_service::{
	define_windows_service,
	service::{ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus, ServiceType},
	service_control_handler::{self, ServiceControlHandlerResult},
	service_dispatcher,
};

define_windows_service!(ffi_service_main, daemon_entry);

pub fn start_service() -> windows_service::Result<()>
{
	service_dispatcher::start("fw16-led-matrixd", ffi_service_main)?;
	Ok(())
}

fn run_service() -> windows_service::Result<()>
{
	let (tx, rx) = channel();

	let event_tx = tx.clone();
	let event_handler = move |event| -> ServiceControlHandlerResult {
		match event
		{
			ServiceControl::Stop =>
			{
				event_tx.send(ServiceExitCode::Win32(0)).unwrap();
				ServiceControlHandlerResult::NoError
			},
			ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
			_ => ServiceControlHandlerResult::NotImplemented,
		}
	};

	let status_handle = service_control_handler::register("fw16-led-matrixd", event_handler)?;

	let mut next_status = ServiceStatus { service_type: ServiceType::OWN_PROCESS,
	                                      current_state: ServiceState::Running,
	                                      controls_accepted: ServiceControlAccept::STOP,

	                                      exit_code: ServiceExitCode::Win32(0),
	                                      checkpoint: 0,
	                                      wait_hint: Duration::default(),
	                                      process_id: None };

	status_handle.set_service_status(next_status)?;

	let args = Args::parse();

	let error_tx = tx.clone();
	spawn(move || match daemon_main(args)
	{
		Ok(_) => error_tx.send(ServiceExitCode::Win32(0)).unwrap(),
		Err(e) if e.is::<SharedError>() =>
		{
			error_tx.send(Into::<ServiceExitCode>::into(*e.downcast::<SharedError>().unwrap())).unwrap()
		},
		Err(_) => error_tx.send(ServiceExitCode::ServiceSpecific(0xDEAD0000)).unwrap(),
	});

	loop
	{
		if let Ok(e) = rx.try_recv()
		{
			next_status = ServiceStatus { service_type: ServiceType::OWN_PROCESS,
			                              current_state: ServiceState::Stopped,
			                              controls_accepted: ServiceControlAccept::empty(),
			                              exit_code: e,
			                              checkpoint: 0,
			                              wait_hint: Duration::default(),
			                              process_id: None };
			status_handle.set_service_status(next_status)?;
			break;
		}

		sleep(Duration::from_secs(1));
	}

	Ok(())
}

fn daemon_entry(_: Vec<OsString>)
{
	if let Err(e) = run_service()
	{
		panic!("{}", e);
	}
}
