use std::path::{PathBuf};
use tracing_subscriber::{Layer};
use tracing_subscriber::prelude::*;
use std::fs::OpenOptions;
use std::io;

pub struct Logger {
    log_dir: PathBuf,
    log_level: String,
    _file_guard: Option<tracing_appender::non_blocking::WorkerGuard>,
}


impl Logger {
    pub fn new(log_dir: PathBuf, log_level: String) -> Self {
        Self {
            log_dir,
            log_level,
            _file_guard: None,
        }
    }

    pub fn init(&mut self) -> anyhow::Result<()> {
        std::fs::create_dir_all(&self.log_dir)?;

        let log_path = self.log_dir.join("svcs_compilation.log");

        // Open file in write mode to truncate existing log file
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&log_path)?;
        
        let (non_blocking_file, guard) = tracing_appender::non_blocking(file);
        self._file_guard = Some(guard);

        let (non_blocking_console, _console_guard) = tracing_appender::non_blocking(std::io::stdout());

        let timer = tracing_subscriber::fmt::time::ChronoLocal::new("%Y-%m-%d %H:%M:%S%.3f".to_string());
        let file_layer = tracing_subscriber::fmt::layer()
            .with_writer(non_blocking_file)
            .with_timer(timer.clone())
            .with_ansi(false)
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true);
        let console_layer = tracing_subscriber::fmt::layer()
            .with_writer(non_blocking_console)
            .with_timer(timer)
            .with_ansi(true)
            .with_target(false);

        let filter = tracing_subscriber::EnvFilter::try_new(&self.log_level)
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

        tracing_subscriber::registry()
            .with(file_layer.with_filter(filter.clone()))
            .with(console_layer.with_filter(filter))
            .init();

        tracing::info!("Logger initialized - writing to {}", log_path.display());

        Ok(())
    }
}

// Optional macros for consistent logging stages
#[macro_export]
macro_rules! log_stage {
    ($stage:expr) => {
        tracing::info!("=== {} Stage ===", $stage);
    };
}

#[macro_export]
macro_rules! log_file_processing {
    ($file:expr) => {
        tracing::info!("Processing file: {}", $file.display());
    };
}

#[macro_export]
macro_rules! log_error {
    ($msg:expr) => {
        tracing::error!("ERROR: {}", $msg);
    };
    ($msg:expr, $($arg:tt)*) => {
        tracing::error!("ERROR: {}", format!($msg, $($arg)*));
    };
}
