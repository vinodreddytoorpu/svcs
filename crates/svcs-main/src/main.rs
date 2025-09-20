use anyhow::{anyhow, Result};
use std::fs;
use std::path::{Path, PathBuf};
use svcs_cli::Args;
use svcs_lexer::{create_default_lexer, Token};
use svcs_lexer::utils::TokenStats;
use svcs_parser::{parse_tokens_with_spans, Cst};
use svcs_preprocessor::preprocess;
use svcs_analyzer::analyze;
use svcs_logger::{log_stage, Logger};
use tracing::{debug, error, info};

fn main() -> Result<()> {
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

    info!("SVCS SystemVerilog Compiler starting up");
    debug!("Arguments: {:?}", args);

    // Get input files to process
    let input_files = match args.get_input_files() {
        Ok(files) => files,
        Err(e) => {
            error!("Failed to get input files: {}", e);
            std::process::exit(1);
        }
    };

    log_stage!("Preprocessing");
    for file in &input_files {
        debug!("Preprocessing file: {}", file.display());
        process_file_preprocessing(file)?;
    }

    log_stage!("Lexical Analysis");
    let mut all_tokens = Vec::new();
    for file in &input_files {
        debug!("Lexing file: {}", file.display());
        let tokens = process_file_lexing(file)?;
        all_tokens.push((file.clone(), tokens));
    }

    log_stage!("Parsing");
    let mut all_csts = Vec::new();
    for (file, tokens) in all_tokens {
        debug!("Parsing file: {}", file.display());
        let cst = process_file_parsing(&file, tokens)?;
        all_csts.push((file, cst));
    }

    log_stage!("Semantic Analysis");
    for (file, cst) in all_csts {
        debug!("Analyzing file: {}", file.display());
        process_file_analysis(cst, &file)?;
    }

    info!("SVCS compilation completed successfully");
    Ok(())
}

fn process_file_preprocessing(file: &Path) -> Result<()> {
    debug!("Preprocessing: {}", file.display());
    let content = fs::read_to_string(file)?;
    let _preprocessed = preprocess(&content)
        .map_err(|e| anyhow!("Preprocessing failed for {}: {}", file.display(), e))?;
    info!("Preprocessing completed for {}", file.display());
    Ok(())
}

fn process_file_lexing(file: &Path) -> Result<Vec<(Token, std::ops::Range<usize>)>> {
    debug!("Lexing: {}", file.display());
    let content = fs::read_to_string(file)?;
    let mut lexer = create_default_lexer(&content, file.display().to_string());
    let (tokens, stats): (Vec<(Token, std::ops::Range<usize>)>, TokenStats) =
        lexer.tokenize_with_stats()
            .map_err(|e| anyhow!("Lexical analysis failed for {}: {}", file.display(), e))?;

    info!("Generated {} tokens from {}", stats.total_tokens, file.display());

    // Write tokens to out/lexer/tokens/<file>.tokens
    let mut out_dir = PathBuf::from("out/lexer/tokens");
    fs::create_dir_all(&out_dir)?;
    let file_stem = file.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown");
    out_dir.push(format!("{}.tokens", file_stem));

    let mut lines = Vec::with_capacity(tokens.len());
    for (token, span) in &tokens {
        let text = &content[span.clone()];
        lines.push(format!("{:?}: '{}'", token, text.replace('\n', "\\n")));
    }
    fs::write(&out_dir, lines.join("\n"))?;
    info!("Tokens written to {}", out_dir.display());

    Ok(tokens)
}

fn process_file_parsing(
    file: &Path,
    tokens: Vec<(Token, std::ops::Range<usize>)>,
) -> Result<Cst> {
    debug!("Parsing: {}", file.display());
    let content = fs::read_to_string(file)?;
    let cst = parse_tokens_with_spans(&content, &tokens)
        .map_err(|e| anyhow!("Parsing failed for {}: {}", file.display(), e))?;
    info!("Parsed CST for {}", file.display());
    // === Write CST to disk ===
    let cst_out_dir = Path::new("out/parser/cst");
    fs::create_dir_all(cst_out_dir)?;
    let cst_file_path = cst_out_dir.join(format!(
        "{}.cst",
        file.file_stem().unwrap().to_string_lossy()
    ));
    fs::write(&cst_file_path, cst.to_string())?;
    info!("CST written to {}", cst_file_path.display());
    Ok(cst)
}

fn process_file_analysis(cst: Cst, file: &Path) -> Result<()> {
    debug!("Analyzing: {}", file.display());
    analyze(&cst)
        .map_err(|e| anyhow!("Semantic analysis failed for {}: {}", file.display(), e))?;
    info!("Semantic analysis completed for {}", file.display());
    Ok(())
}
