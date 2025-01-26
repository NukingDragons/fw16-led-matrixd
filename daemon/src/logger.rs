use log::LevelFilter;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::rolling_file::policy::compound::{
	roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
};
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use std::error::Error;

pub struct LoggingOptions<'a>
{
	pub pattern: &'a str,
	pub max_filesize: u64,
	pub max_log_count: u32,
	pub stderr_log_level: LevelFilter,
}

impl Default for LoggingOptions<'_>
{
	fn default() -> Self
	{
		LoggingOptions { pattern: "{D([DEBUG: {f}:{L}] )}[{d(%Y-%m-%d %H:%M:%S)}] {h({l})} {t} - {m}{n}",
		                 max_filesize: 10 * 1024 * 1024,
		                 max_log_count: 10,
		                 stderr_log_level: LevelFilter::Info }
	}
}

pub fn setup_logging(log_file: Option<&str>, log_options: Option<LoggingOptions>) -> Result<(), Box<dyn Error>>
{
	let options = log_options.unwrap_or_default();

	// Console appender to append to stderr
	let stderr = ConsoleAppender::builder().target(Target::Stderr)
	                                       .encoder(Box::new(PatternEncoder::new(options.pattern)))
	                                       .build();

	match log_file
	{
		Some(file) =>
		{
			// Configure the log rolling mechanism
			let roll_pattern = format!("{}.{{}}", file);
			let policy = CompoundPolicy::new(
			                                 Box::new(SizeTrigger::new(options.max_filesize)),
			                                 Box::new(FixedWindowRoller::builder().build(
				&roll_pattern,
				options.max_log_count,
			)?),
			);

			// Rolling appender, so that logs don't end up huge
			let logfile = RollingFileAppender::builder().encoder(Box::new(PatternEncoder::new(options.pattern)))
			                                            .build(file, Box::new(policy))?;

			// Build the actual config
			let config =
            Config::builder().appender(Appender::builder().filter(Box::new(ThresholdFilter::new(options.stderr_log_level)))
                                                          .build("stderr", Box::new(stderr)))
                             .appender(Appender::builder().build("logfile", Box::new(logfile)))
                             .build(Root::builder().appender("stderr").appender("logfile").build(LevelFilter::Trace))?;

			log4rs::init_config(config)?;
		},
		None =>
		{
			// Build the actual config
			let config =
            Config::builder().appender(Appender::builder().filter(Box::new(ThresholdFilter::new(options.stderr_log_level)))
                                                          .build("stderr", Box::new(stderr)))
                             .build(Root::builder().appender("stderr").build(LevelFilter::Trace))?;

			log4rs::init_config(config)?;
		},
	};

	Ok(())
}
