use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "svcs",
    version,
    about = "SystemVerilog Compiler and Simulator",
    long_about = None
)]
pub struct Args {
    /// Input files to process
    #[arg(
        short = 'i',
        long = "input",
        value_name = "FILE",
        help = "Input SystemVerilog files",
        num_args = 1..,
        value_delimiter = ' '
    )]
    pub input_files: Vec<PathBuf>,

    /// Input directory to process
    #[arg(
        long = "dir",
        value_name = "DIR",
        help = "Input directory containing SystemVerilog files",
        conflicts_with = "input_files"
    )]
    pub input_dir: Option<PathBuf>,

    /// Output directory for logs
    #[arg(
        long = "log-dir", 
        value_name = "DIR",
        default_value = "out",
        help = "Directory to store log files"
    )]
    pub log_dir: PathBuf,

    /// Enable verbose logging
    #[arg(short, long, help = "Enable verbose output")]
    pub verbose: bool,

    /// Log level
    #[arg(
        long = "log-level",
        value_name = "LEVEL",
        default_value = "info",
        help = "Set log level (trace, debug, info, warn, error)"
    )]
    pub log_level: String,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.input_files.is_empty() && self.input_dir.is_none() {
            return Err("Either input files (-i) or input directory (--dir) must be specified".to_string());
        }

        // Validate input files exist
        for file in &self.input_files {
            if !file.exists() {
                return Err(format!("Input file does not exist: {}", file.display()));
            }
        }

        // Validate input directory exists
        if let Some(ref dir) = self.input_dir {
            if !dir.exists() || !dir.is_dir() {
                return Err(format!("Input directory does not exist or is not a directory: {}", dir.display()));
            }
        }

        Ok(())
    }

    pub fn get_input_files(&self) -> Result<Vec<PathBuf>, String> {
        let mut files = self.input_files.clone();

        if let Some(ref dir) = self.input_dir {
            // Collect SystemVerilog files from directory
            let sv_files = std::fs::read_dir(dir)
                .map_err(|e| format!("Failed to read directory {}: {}", dir.display(), e))?
                .filter_map(|entry| {
                    let entry = entry.ok()?;
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) = path.extension() {
                            if ext == "sv" || ext == "v" || ext == "vh" || ext == "svh" {
                                return Some(path);
                            }
                        }
                    }
                    None
                })
                .collect::<Vec<_>>();
            
            files.extend(sv_files);
        }

        if files.is_empty() {
            return Err("No SystemVerilog files found".to_string());
        }

        Ok(files)
    }
}
