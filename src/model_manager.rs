use std::path::PathBuf;
use crate::error::{Result, AudioTranscriptionError};
use crate::ModelSize;

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

    /// Get the whisper model directory for a specific model size
    pub fn get_whisper_model_dir(&self, size: &ModelSize) -> PathBuf {
        self.cache_dir.join("whisper").join(size.to_string())
    }

    /// Get the pyannote model directory
    pub fn get_pyannote_model_dir(&self) -> PathBuf {
        self.cache_dir.join("pyannote")
    }

    /// Get the full path to a whisper model file
    pub fn get_whisper_model_path(&self, size: &ModelSize) -> PathBuf {
        self.get_whisper_model_dir(size).join(format!("ggml-{}.bin", size))
    }

    /// Get the full path to the pyannote model file
    pub fn get_pyannote_model_path(&self) -> PathBuf {
        self.get_pyannote_model_dir().join("pytorch_model.bin")
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

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_model_manager_creation() {
        let manager = ModelManager::new();
        assert!(manager.is_ok());
        
        let manager = manager.unwrap();
        assert!(manager.cache_dir().exists());
    }

    #[test]
    fn test_model_path_generation() {
        let manager = ModelManager::new().unwrap();
        
        let whisper_path = manager.get_whisper_model_path(&ModelSize::Base);
        assert!(whisper_path.to_string_lossy().contains("whisper"));
        assert!(whisper_path.to_string_lossy().contains("base"));
        assert!(whisper_path.to_string_lossy().ends_with("ggml-base.bin"));
        
        let pyannote_path = manager.get_pyannote_model_path();
        assert!(pyannote_path.to_string_lossy().contains("pyannote"));
        assert!(pyannote_path.to_string_lossy().ends_with("pytorch_model.bin"));
    }

    // Feature: audio-transcription-cli, Property 12: Model Storage Location
    // **Validates: Requirements 5.4**
    proptest! {
        #[test]
        fn test_model_storage_location_property(
            model_size in prop::sample::select(vec![
                ModelSize::Tiny, 
                ModelSize::Base, 
                ModelSize::Small, 
                ModelSize::Medium, 
                ModelSize::Large
            ])
        ) {
            tokio_test::block_on(async {
                // For any model size, the model storage location should be in the platform-specific cache directory
                let manager = ModelManager::new().unwrap();
                
                // Get the expected cache directory path
                let expected_cache_dir = dirs::cache_dir()
                    .expect("Should be able to get cache directory")
                    .join("audio-transcribe")
                    .join("models");
                
                // Test whisper model path
                let whisper_path = manager.get_whisper_model_path(&model_size);
                prop_assert!(
                    whisper_path.starts_with(&expected_cache_dir),
                    "Whisper model path {:?} should start with cache directory {:?}",
                    whisper_path,
                    expected_cache_dir
                );
                
                // Verify the path contains the whisper subdirectory
                prop_assert!(
                    whisper_path.to_string_lossy().contains("whisper"),
                    "Whisper model path should contain 'whisper' subdirectory"
                );
                
                // Verify the path contains the model size
                prop_assert!(
                    whisper_path.to_string_lossy().contains(&model_size.to_string()),
                    "Whisper model path should contain model size '{}'",
                    model_size.to_string()
                );
                
                // Test pyannote model path
                let pyannote_path = manager.get_pyannote_model_path();
                prop_assert!(
                    pyannote_path.starts_with(&expected_cache_dir),
                    "Pyannote model path {:?} should start with cache directory {:?}",
                    pyannote_path,
                    expected_cache_dir
                );
                
                // Verify the path contains the pyannote subdirectory
                prop_assert!(
                    pyannote_path.to_string_lossy().contains("pyannote"),
                    "Pyannote model path should contain 'pyannote' subdirectory"
                );
                
                // Test that the cache directory itself is correctly set
                let manager_cache_dir = manager.cache_dir();
                prop_assert!(
                    manager_cache_dir == &expected_cache_dir,
                    "Manager cache directory {:?} should equal expected cache directory {:?}",
                    manager_cache_dir,
                    expected_cache_dir
                );
                
                Ok(())
            }).unwrap()
        }
    }
}