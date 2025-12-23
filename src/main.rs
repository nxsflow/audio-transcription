use clap::Parser;
use std::path::PathBuf;
use std::io::{self, Write};

mod cli;
mod core;
mod ui;
mod error;

use crate::error::Result;
use crate::cli::FileBrowser;
use crate::core::{ModelManager, ModelSize};

#[derive(Parser)]
#[command(name = "audio-transcribe")]
#[command(about = "High-performance audio transcription CLI with speaker diarization")]
#[command(version = "0.1.0")]
#[derive(Debug)]
pub struct Cli {
    /// Input audio file path (optional - if not provided, opens file browser)
    pub input: Option<PathBuf>,

    /// Model size to use for transcription
    #[arg(long, value_enum, default_value_t = ModelSize::Medium)]
    pub model: ModelSize,

    /// Output directory for transcript files
    #[arg(long)]
    pub output: Option<PathBuf>,

    /// Target chunk duration in seconds
    #[arg(long, default_value_t = 120.0)]
    pub chunk_size: f32,

    /// Number of parallel transcription jobs
    #[arg(long)]
    pub jobs: Option<usize>,

    /// Disable GPU acceleration (force CPU-only processing)
    #[arg(long)]
    pub no_gpu: bool,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    if cli.verbose {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
    }

    log::info!("Audio Transcription CLI v{}", env!("CARGO_PKG_VERSION"));
    log::debug!("CLI arguments: {:?}", cli);

    // Check and ensure models are available before proceeding
    log::info!("Checking required models...");
    let model_manager = ModelManager::new()?;
    match model_manager.ensure_models_available(&cli.model).await {
        Ok(true) => {
            log::info!("All required models are available");
        }
        Ok(false) => {
            println!("Model download cancelled. Cannot proceed without required models.");
            return Ok(());
        }
        Err(e) => {
            println!("Error with model setup: {}", e);
            println!("Cannot proceed without required models.");
            return Err(e);
        }
    }

    // Determine input file path
    let input_file = if let Some(input) = cli.input {
        // Direct file input provided
        log::info!("Processing file: {}", input.display());
        input
    } else {
        // No input file provided, launch file browser
        log::info!("No input file provided, launching interactive file browser...");
        
        // Wait a moment for any pending output to finish, then clear screen completely
        std::thread::sleep(std::time::Duration::from_millis(100));
        print!("\x1b[2J\x1b[H\x1b[0m"); // Clear screen, home cursor, reset attributes
        io::stdout().flush().unwrap();
        
        println!("Audio Transcription CLI - File Browser");
        println!("Navigate to select an audio file for transcription");
        println!("Press 'q' to quit, 'f' to toggle filter, up/down to navigate, Enter to select");
        println!(); // Add blank line
        
        let current_dir = std::env::current_dir()
            .map_err(|e| crate::error::AudioTranscriptionError::FileBrowser(
                format!("Failed to get current directory: {}", e)
            ))?;
        
        let mut browser = FileBrowser::new(current_dir)?;

        match browser.run_interactive()? {
            Some(selected_file) => {
                // Clear screen after selection
                print!("\x1b[2J\x1b[H");
                io::stdout().flush().unwrap();
                log::info!("Selected file: {}", selected_file.display());
                selected_file
            }
            None => {
                // Clear screen on exit
                print!("\x1b[2J\x1b[H");
                io::stdout().flush().unwrap();
                println!("No file selected. Exiting...");
                return Ok(());
            }
        }
    };

    // Validate that the selected file exists and is supported
    if !input_file.exists() {
        return Err(crate::error::AudioTranscriptionError::FileBrowser(
            format!("File does not exist: {}", input_file.display())
        ));
    }

    if !input_file.is_file() {
        return Err(crate::error::AudioTranscriptionError::FileBrowser(
            format!("Path is not a file: {}", input_file.display())
        ));
    }

    // Check if it's a supported audio format
    if let Some(extension) = input_file.extension().and_then(|ext| ext.to_str()) {
        let supported_formats = ["wav", "mp3", "m4a", "flac", "ogg", "webm"];
        if !supported_formats.contains(&extension.to_lowercase().as_str()) {
            return Err(crate::error::AudioTranscriptionError::UnsupportedFormat(
                format!("Unsupported audio format: .{}", extension)
            ));
        }
    } else {
        return Err(crate::error::AudioTranscriptionError::UnsupportedFormat(
            "File has no extension or unsupported format".to_string()
        ));
    }

    println!("\n✅ Selected audio file: {}", input_file.display());
    println!("📊 Configuration:");
    println!("   Model: {}", cli.model);
    if let Some(output) = &cli.output {
        println!("   Output directory: {}", output.display());
    } else {
        println!("   Output directory: Same as input file");
    }
    println!("   Chunk size: {} seconds", cli.chunk_size);
    if let Some(jobs) = cli.jobs {
        println!("   Parallel jobs: {}", jobs);
    } else {
        println!("   Parallel jobs: auto-detect ({})", num_cpus::get());
    }
    println!("   GPU acceleration: {}", !cli.no_gpu);

    // TODO: Implement actual audio processing
    // This will be implemented in subsequent tasks
    println!("\n🚧 Audio processing pipeline not yet implemented.");
    println!("This will be added in upcoming tasks (Task 5-15).");
    println!("For now, the file browser integration is complete!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use std::path::PathBuf;

    #[test]
    fn test_default_cli_arguments() {
        // Test default values when no arguments are provided
        let cli = Cli::try_parse_from(&["audio-transcribe"]).unwrap();
        
        assert!(cli.input.is_none());
        assert!(matches!(cli.model, ModelSize::Medium));
        assert!(cli.output.is_none());
        assert_eq!(cli.chunk_size, 120.0);
        assert!(cli.jobs.is_none());
        assert!(!cli.no_gpu);
        assert!(!cli.verbose);
    }

    #[test]
    fn test_model_size_flag_tiny() {
        let cli = Cli::try_parse_from(&["audio-transcribe", "--model", "tiny"]).unwrap();
        assert!(matches!(cli.model, ModelSize::Tiny));
    }

    #[test]
    fn test_model_size_flag_base() {
        let cli = Cli::try_parse_from(&["audio-transcribe", "--model", "base"]).unwrap();
        assert!(matches!(cli.model, ModelSize::Base));
    }

    #[test]
    fn test_model_size_flag_small() {
        let cli = Cli::try_parse_from(&["audio-transcribe", "--model", "small"]).unwrap();
        assert!(matches!(cli.model, ModelSize::Small));
    }

    #[test]
    fn test_model_size_flag_medium() {
        let cli = Cli::try_parse_from(&["audio-transcribe", "--model", "medium"]).unwrap();
        assert!(matches!(cli.model, ModelSize::Medium));
    }

    #[test]
    fn test_model_size_flag_large() {
        let cli = Cli::try_parse_from(&["audio-transcribe", "--model", "large"]).unwrap();
        assert!(matches!(cli.model, ModelSize::Large));
    }

    #[test]
    fn test_invalid_model_size() {
        let result = Cli::try_parse_from(&["audio-transcribe", "--model", "invalid"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_output_directory_flag() {
        let cli = Cli::try_parse_from(&["audio-transcribe", "--output", "/tmp/transcripts"]).unwrap();
        assert_eq!(cli.output, Some(PathBuf::from("/tmp/transcripts")));
    }

    #[test]
    fn test_chunk_size_flag() {
        let cli = Cli::try_parse_from(&["audio-transcribe", "--chunk-size", "60.5"]).unwrap();
        assert_eq!(cli.chunk_size, 60.5);
    }

    #[test]
    fn test_invalid_chunk_size() {
        let result = Cli::try_parse_from(&["audio-transcribe", "--chunk-size", "invalid"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_jobs_flag() {
        let cli = Cli::try_parse_from(&["audio-transcribe", "--jobs", "4"]).unwrap();
        assert_eq!(cli.jobs, Some(4));
    }

    #[test]
    fn test_invalid_jobs_value() {
        let result = Cli::try_parse_from(&["audio-transcribe", "--jobs", "invalid"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_no_gpu_flag() {
        let cli = Cli::try_parse_from(&["audio-transcribe", "--no-gpu"]).unwrap();
        assert!(cli.no_gpu);
    }

    #[test]
    fn test_verbose_flag_short() {
        let cli = Cli::try_parse_from(&["audio-transcribe", "-v"]).unwrap();
        assert!(cli.verbose);
    }

    #[test]
    fn test_verbose_flag_long() {
        let cli = Cli::try_parse_from(&["audio-transcribe", "--verbose"]).unwrap();
        assert!(cli.verbose);
    }

    #[test]
    fn test_input_file_positional() {
        let cli = Cli::try_parse_from(&["audio-transcribe", "test.wav"]).unwrap();
        assert_eq!(cli.input, Some(PathBuf::from("test.wav")));
    }

    #[test]
    fn test_combined_flags() {
        let cli = Cli::try_parse_from(&[
            "audio-transcribe",
            "input.mp3",
            "--model", "large",
            "--output", "/tmp/output",
            "--chunk-size", "90.0",
            "--jobs", "8",
            "--no-gpu",
            "--verbose"
        ]).unwrap();

        assert_eq!(cli.input, Some(PathBuf::from("input.mp3")));
        assert!(matches!(cli.model, ModelSize::Large));
        assert_eq!(cli.output, Some(PathBuf::from("/tmp/output")));
        assert_eq!(cli.chunk_size, 90.0);
        assert_eq!(cli.jobs, Some(8));
        assert!(cli.no_gpu);
        assert!(cli.verbose);
    }

    #[test]
    fn test_help_flag() {
        let result = Cli::try_parse_from(&["audio-transcribe", "--help"]);
        // Help flag causes clap to exit with an error (but it's expected behavior)
        assert!(result.is_err());
        
        // Verify the error is specifically a help request
        if let Err(err) = result {
            assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
        }
    }

    #[test]
    fn test_version_flag() {
        let result = Cli::try_parse_from(&["audio-transcribe", "--version"]);
        // Version flag causes clap to exit with an error (but it's expected behavior)
        assert!(result.is_err());
        
        // Verify the error is specifically a version request
        if let Err(err) = result {
            assert_eq!(err.kind(), clap::error::ErrorKind::DisplayVersion);
        }
    }

    #[test]
    fn test_model_size_display() {
        assert_eq!(ModelSize::Tiny.to_string(), "tiny");
        assert_eq!(ModelSize::Base.to_string(), "base");
        assert_eq!(ModelSize::Small.to_string(), "small");
        assert_eq!(ModelSize::Medium.to_string(), "medium");
        assert_eq!(ModelSize::Large.to_string(), "large");
    }

    #[test]
    fn test_negative_chunk_size() {
        // Test that negative chunk size can be passed using -- separator
        let result = Cli::try_parse_from(&["audio-transcribe", "--chunk-size", "--", "-10.0"]);
        // This should fail because -- separates positional args, not flag values
        assert!(result.is_err());
        
        // Test with proper escaping - clap should accept this
        let result = Cli::try_parse_from(&["audio-transcribe", "--chunk-size=-10.0"]);
        match result {
            Ok(cli) => {
                assert_eq!(cli.chunk_size, -10.0);
            }
            Err(err) => {
                // If clap rejects negative values, that's also valid behavior
                assert!(err.kind() == clap::error::ErrorKind::ValueValidation 
                    || err.kind() == clap::error::ErrorKind::UnknownArgument);
            }
        }
    }

    #[test]
    fn test_zero_jobs() {
        let cli = Cli::try_parse_from(&["audio-transcribe", "--jobs", "0"]).unwrap();
        assert_eq!(cli.jobs, Some(0));
    }
}