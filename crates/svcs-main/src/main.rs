use anyhow::Result;
use svcs_cli::Args;
use svcs_logger::{Logger, log_stage, log_file_processing};
use std::fs;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    // Parse command line arguments
    let args = Args::parse_args();

    // Validate arguments
    if let Err(e) = args.validate() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    // Initialize logger
    let mut logger = Logger::new(args.log_dir.clone(), args.log_level.clone());
    logger.init()?;


    // Log startup information
    tracing::info!("SVCS SystemVerilog Compiler starting up");
    tracing::info!("Arguments: {:?}", args);

    // Get input files
    let input_files = match args.get_input_files() {
        Ok(files) => files,
        Err(e) => {
            tracing::error!("Failed to get input files: {}", e);
            std::process::exit(1);
        }
    };

    // Process each stage
    log_stage!("Preprocessing");
    for file in &input_files {
        log_file_processing!(file);
        process_file_preprocessing(file)?;
    }

    log_stage!("Lexical Analysis");
    for file in &input_files {
        log_file_processing!(file);
        process_file_lexing(file)?;
    }

    log_stage!("Parsing");
    for file in &input_files {
        log_file_processing!(file);
        process_file_parsing(file)?;
    }

    log_stage!("Semantic Analysis");
    for file in &input_files {
        log_file_processing!(file);
        process_file_analysis(file)?;
    }

    tracing::info!("SVCS compilation completed successfully");
    
    Ok(())
}

fn process_file_preprocessing(file: &std::path::Path) -> Result<()> {
    tracing::debug!("Preprocessing: {}", file.display());
    // TODO: Call svcs-preprocessor crate
    Ok(())
}

fn process_file_lexing(file: &std::path::Path) -> Result<()> {
    tracing::debug!("Lexing: {}", file.display());

    // Read the file content
    let content = std::fs::read_to_string(file)?;

    // Create lexer with all default plugins
    let mut lexer = svcs_lexer::create_default_lexer(&content, file.display().to_string());

    // Tokenize with statistics
    match lexer.tokenize_with_stats() {
        Ok((tokens, _stats)) => {
            tracing::info!("Generated {} tokens from {}", tokens.len(), file.display());

            // Prepare output directory for tokens
            let mut out_dir = PathBuf::from("out/lexer/tokens");
            if !out_dir.exists() {
                fs::create_dir_all(&out_dir)?;
            }

            // Use full file name with extension plus ".tokens" suffix
            let file_name = file.file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");
            out_dir.push(format!("{}.tokens", file_name));

            // Write tokens to file
            let mut token_lines = Vec::new();
            for (token, span) in &tokens {
                let text = &content[span.clone()];
                token_lines.push(format!("Token: {:?}, Text: '{}'", token, text));
            }
            fs::write(&out_dir, token_lines.join("\n"))?;

            tracing::info!("Tokens written to file: {}", out_dir.display());

            Ok(())
        }
        Err(e) => {
            tracing::error!("Lexical analysis failed for {}: {}", file.display(), e);
            Err(anyhow::anyhow!("Lexical analysis failed: {}", e))
        }
    }
}


fn process_file_parsing(file: &std::path::Path) -> Result<()> {
    tracing::debug!("Parsing: {}", file.display());
    // TODO: Call svcs-parser crate
    Ok(())
}

fn process_file_analysis(file: &std::path::Path) -> Result<()> {
    tracing::debug!("Analyzing: {}", file.display());
    // TODO: Call svcs-analyzer crate
    Ok(())
}
