use std::path::PathBuf;
use crate::error::{Result, AudioTranscriptionError};

#[derive(Debug, Clone)]
pub enum DirectoryEntry {
    Directory { name: String },
    AudioFile { name: String, size: u64 },
    Parent,
}

pub enum Direction {
    Up,
    Down,
}

pub struct FileBrowser {
    current_path: PathBuf,
    entries: Vec<DirectoryEntry>,
    selected_index: usize,
}

impl FileBrowser {
    pub fn new(path: PathBuf) -> Result<Self> {
        let mut browser = Self {
            current_path: path,
            entries: Vec::new(),
            selected_index: 0,
        };
        browser.refresh_entries()?;
        Ok(browser)
    }

    pub fn navigate_to(&mut self, path: PathBuf) -> Result<()> {
        self.current_path = path;
        self.selected_index = 0;
        self.refresh_entries()
    }

    pub fn get_selected(&self) -> Option<&DirectoryEntry> {
        self.entries.get(self.selected_index)
    }

    pub fn move_selection(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            Direction::Down => {
                if self.selected_index < self.entries.len().saturating_sub(1) {
                    self.selected_index += 1;
                }
            }
        }
    }

    pub fn render(&self) -> String {
        // TODO: Implement terminal rendering
        // This will be implemented in task 3
        format!("File browser at: {}", self.current_path.display())
    }

    pub fn current_path(&self) -> &PathBuf {
        &self.current_path
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn entries(&self) -> &[DirectoryEntry] {
        &self.entries
    }

    fn refresh_entries(&mut self) -> Result<()> {
        // TODO: Implement directory scanning and audio file filtering
        // This will be implemented in task 3
        self.entries.clear();
        
        // Add parent directory entry if not at root
        if self.current_path.parent().is_some() {
            self.entries.push(DirectoryEntry::Parent);
        }

        // Placeholder implementation - will be replaced in task 3
        self.entries.push(DirectoryEntry::Directory { 
            name: "example_dir".to_string() 
        });
        self.entries.push(DirectoryEntry::AudioFile { 
            name: "example.wav".to_string(), 
            size: 1024 
        });

        Ok(())
    }

    fn is_supported_audio_format(extension: &str) -> bool {
        matches!(
            extension.to_lowercase().as_str(),
            "wav" | "mp3" | "m4a" | "flac" | "ogg" | "webm"
        )
    }
}