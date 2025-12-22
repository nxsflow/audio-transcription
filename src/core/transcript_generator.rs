use std::path::{Path, PathBuf};
use crate::core::audio_processor::{SpeechSegment, TranscriptResult};
use crate::error::{Result, AudioTranscriptionError};

pub struct TranscriptGenerator {
    output_dir: Option<PathBuf>,
}

impl TranscriptGenerator {
    pub fn new(output_dir: Option<PathBuf>) -> Self {
        Self { output_dir }
    }

    pub fn generate_transcript(&self, input_path: &Path, result: &TranscriptResult) -> Result<PathBuf> {
        let output_path = self.determine_output_path(input_path)?;
        let formatted_transcript = self.format_transcript(&result.segments)?;
        
        // TODO: Write transcript to file
        // This will be implemented in task 11
        log::info!("Generated transcript: {}", output_path.display());
        
        // Placeholder - write to file
        std::fs::write(&output_path, formatted_transcript)?;
        
        Ok(output_path)
    }

    fn determine_output_path(&self, input_path: &Path) -> Result<PathBuf> {
        let base_name = input_path
            .file_stem()
            .ok_or_else(|| AudioTranscriptionError::Configuration("Invalid input file path".to_string()))?
            .to_string_lossy();

        let output_dir = self.output_dir
            .as_ref()
            .map(|p| p.as_path())
            .unwrap_or_else(|| input_path.parent().unwrap_or_else(|| Path::new(".")));

        let output_path = output_dir.join(format!("{}.txt", base_name));
        Ok(output_path)
    }

    fn format_transcript(&self, segments: &[SpeechSegment]) -> Result<String> {
        // TODO: Implement proper transcript formatting with speaker labels
        // This will be implemented in task 11
        let mut output = String::new();
        let mut current_speaker: Option<u8> = None;

        for segment in segments {
            // Check if speaker changed
            if segment.speaker != current_speaker {
                if current_speaker.is_some() {
                    output.push('\n'); // Empty line between speakers
                }
                
                if let Some(speaker_id) = segment.speaker {
                    output.push_str(&format!("[SPEAKER_{:02}]\n", speaker_id));
                } else {
                    output.push_str("[SPEAKER_00]\n");
                }
                
                current_speaker = segment.speaker;
            }

            // Add the transcribed text
            output.push_str(&segment.text);
            output.push('\n');
        }

        Ok(output)
    }

    pub fn set_output_dir(&mut self, output_dir: Option<PathBuf>) {
        self.output_dir = output_dir;
    }

    pub fn output_dir(&self) -> Option<&PathBuf> {
        self.output_dir.as_ref()
    }
}