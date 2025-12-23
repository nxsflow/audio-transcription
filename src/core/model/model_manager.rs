use std::path::PathBuf;
use std::io::Write;
use crate::error::{Result, AudioTranscriptionError};
use crate::core::model::ModelSize;
use crate::core::model::download;

pub struct ModelManager {
    cache_dir: PathBuf,
}

impl ModelManager {
    pub fn new() -> Result<Self> {
        let cache_dir = Self::get_cache_directory()?;
        
        // Create the complete directory structure for model storage
        Self::create_directory_structure(&cache_dir)?;
        
        Ok(Self {
            cache_dir,
        })
    }

    pub fn cache_dir(&self) -> &PathBuf {
        &self.cache_dir
    }

    /// Get platform-specific cache directory for model storage
    fn get_cache_directory() -> Result<PathBuf> {
        let cache_dir = dirs::cache_dir()
            .ok_or_else(|| AudioTranscriptionError::Configuration(
                "Unable to determine cache directory".to_string()
            ))?
            .join("audio-transcribe")
            .join("models");
        
        Ok(cache_dir)
    }

    /// Check if required models exist and prompt for download if needed
    /// Returns Ok(true) if models are available, Ok(false) if user cancelled, Err on error
    pub async fn ensure_models_available(&self, model_size: &ModelSize) -> Result<bool> {
        // Check if transcription model exists
        let transcription_available = download::is_transcription_model_available(&self.cache_dir, model_size);
        
        // Check if diarization model exists
        let diarization_available = download::is_diarization_model_available(&self.cache_dir);
        
        // If both models are available, we're good to go
        if transcription_available && diarization_available {
            log::info!("All required models are available");
            return Ok(true);
        }
        
        // Display which models are missing
        println!("\n⚠️  Required models are missing:");
        if !transcription_available {
            println!("   - Whisper {} model", model_size);
        }
        if !diarization_available {
            println!("   - Sherpa-ONNX speaker diarization models (segmentation + embedding)");
        }
        println!();
        
        // Prompt user for download confirmation
        println!("Would you like to download the missing models now?");
        println!("(This is a one-time download and models will be cached for future use)");
        print!("Download models? [Y/n]: ");
        std::io::stdout().flush().map_err(|e| AudioTranscriptionError::Io(e))?;
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).map_err(|e| AudioTranscriptionError::Io(e))?;
        
        let input = input.trim().to_lowercase();
        if input == "n" || input == "no" {
            return Ok(false);
        }
        
        // Download missing models
        println!("\n📥 Downloading models...");
        
        if !transcription_available {
            download::download_transcription_model(&self.cache_dir, model_size).await?;
        }
        
        if !diarization_available {
            download::download_diarization_model(&self.cache_dir, "").await?;
        }
        
        println!("\n✅ All models downloaded successfully!");
        println!("Models are cached at: {}", self.cache_dir().display());
        std::thread::sleep(std::time::Duration::from_millis(1500));
        
        Ok(true)
    }

    /// Create the complete directory structure for model storage
    fn create_directory_structure(cache_dir: &PathBuf) -> Result<()> {
        // Create main cache directory
        std::fs::create_dir_all(cache_dir)
            .map_err(|e| AudioTranscriptionError::Configuration(
                format!("Failed to create cache directory {}: {}", cache_dir.display(), e)
            ))?;

        // Create whisper model subdirectories for each size
        let whisper_dir = cache_dir.join("whisper");
        std::fs::create_dir_all(&whisper_dir)
            .map_err(|e| AudioTranscriptionError::Configuration(
                format!("Failed to create whisper directory: {}", e)
            ))?;

        // Create subdirectories for each whisper model size
        for size in [ModelSize::Tiny, ModelSize::Base, ModelSize::Small, ModelSize::Medium, ModelSize::Large] {
            let size_dir = whisper_dir.join(size.to_string());
            std::fs::create_dir_all(&size_dir)
                .map_err(|e| AudioTranscriptionError::Configuration(
                    format!("Failed to create whisper size directory {}: {}", size, e)
                ))?;
        }

        // Create pyannote model directory
        let pyannote_dir = cache_dir.join("pyannote");
        std::fs::create_dir_all(&pyannote_dir)
            .map_err(|e| AudioTranscriptionError::Configuration(
                format!("Failed to create pyannote directory: {}", e)
            ))?;

        Ok(())
    }
}