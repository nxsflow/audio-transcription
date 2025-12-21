use std::path::PathBuf;
use crate::error::{Result, AudioTranscriptionError};
use crate::ModelSize;

pub enum ModelType {
    Whisper,
    Pyannote,
}

pub struct WhisperModel {
    // TODO: Add whisper-rs model wrapper
    // This will be implemented in task 2
    pub path: PathBuf,
}

pub struct PyannoteModel {
    // TODO: Add pyannote-rs model wrapper
    // This will be implemented in task 2
    pub path: PathBuf,
}

pub struct ModelManager {
    cache_dir: PathBuf,
    whisper_model: Option<WhisperModel>,
    pyannote_model: Option<PyannoteModel>,
}

impl ModelManager {
    pub fn new() -> Result<Self> {
        let cache_dir = Self::get_cache_directory()?;
        
        // Ensure cache directory exists
        std::fs::create_dir_all(&cache_dir)?;
        
        Ok(Self {
            cache_dir,
            whisper_model: None,
            pyannote_model: None,
        })
    }

    pub async fn ensure_models(&mut self, model_size: ModelSize) -> Result<()> {
        // TODO: Implement model checking and downloading
        // This will be implemented in task 2
        log::info!("Ensuring models are available for size: {}", model_size);
        Ok(())
    }

    pub async fn download_model(&self, model_type: ModelType, size: ModelSize) -> Result<()> {
        // TODO: Implement model downloading with progress tracking
        // This will be implemented in task 2
        log::info!("Downloading {:?} model ({})", model_type, size);
        Ok(())
    }

    pub fn get_whisper_model(&self) -> Result<&WhisperModel> {
        self.whisper_model
            .as_ref()
            .ok_or_else(|| AudioTranscriptionError::Model("Whisper model not loaded".to_string()))
    }

    pub fn get_pyannote_model(&self) -> Result<&PyannoteModel> {
        self.pyannote_model
            .as_ref()
            .ok_or_else(|| AudioTranscriptionError::Model("Pyannote model not loaded".to_string()))
    }

    pub fn cache_dir(&self) -> &PathBuf {
        &self.cache_dir
    }

    fn get_cache_directory() -> Result<PathBuf> {
        let cache_dir = dirs::cache_dir()
            .ok_or_else(|| AudioTranscriptionError::Configuration("Could not determine cache directory".to_string()))?
            .join("audio-transcribe")
            .join("models");
        
        Ok(cache_dir)
    }
}

impl std::fmt::Debug for ModelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelType::Whisper => write!(f, "Whisper"),
            ModelType::Pyannote => write!(f, "Pyannote"),
        }
    }
}