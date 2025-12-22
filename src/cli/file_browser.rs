use std::path::PathBuf;
use crate::error::{Result, AudioTranscriptionError};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{self},
};
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub enum DirectoryEntry {
    Directory { name: String },
    AudioFile { name: String, size: u64 },
    File { name: String, size: u64 },
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
    filter_audio_only: bool,
}

impl FileBrowser {
    pub fn new(path: PathBuf) -> Result<Self> {
        let mut browser = Self {
            current_path: path,
            entries: Vec::new(),
            selected_index: 0,
            filter_audio_only: true, // Default to filtering enabled
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

    pub fn set_audio_filter(&mut self, enabled: bool) -> Result<()> {
        if self.filter_audio_only != enabled {
            self.filter_audio_only = enabled;
            self.selected_index = 0;
            self.refresh_entries()?;
        }
        Ok(())
    }

    pub fn is_audio_filter_enabled(&self) -> bool {
        self.filter_audio_only
    }

    pub fn navigate_selected(&mut self) -> Result<Option<PathBuf>> {
        match self.get_selected() {
            Some(DirectoryEntry::Parent) => {
                if let Some(parent) = self.current_path.parent() {
                    self.navigate_to(parent.to_path_buf())?;
                    Ok(None)
                } else {
                    Ok(None)
                }
            }
            Some(DirectoryEntry::Directory { name }) => {
                let new_path = self.current_path.join(name);
                self.navigate_to(new_path)?;
                Ok(None)
            }
            Some(DirectoryEntry::AudioFile { name, .. }) => {
                let file_path = self.current_path.join(name);
                Ok(Some(file_path))
            }
            Some(DirectoryEntry::File { name, .. }) => {
                let file_path = self.current_path.join(name);
                Ok(Some(file_path))
            }
            None => Ok(None),
        }
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
        let mut output = String::new();
        
        output.push_str("Directory: ");
        output.push_str(&self.current_path.display().to_string());
        output.push_str("\r\n");
        
        if self.filter_audio_only {
            output.push_str("Filter: Audio only\r\n");
        } else {
            output.push_str("Filter: All files\r\n");
        }
        
        output.push_str("Controls: Up/Down=navigate, Enter=select, f=filter, q=quit\r\n");
        output.push_str("------------------------------------------------------------\r\n");

        for (index, entry) in self.entries.iter().enumerate() {
            let is_selected = index == self.selected_index;
            
            if is_selected {
                // Highlight selected item in lime/bright green
                output.push_str("\x1b[92m> ");
            } else {
                output.push_str("  ");
            }
            
            match entry {
                DirectoryEntry::Parent => {
                    output.push_str("../");
                    if is_selected {
                        output.push_str("\x1b[0m");
                    }
                }
                DirectoryEntry::Directory { name } => {
                    output.push_str(name);
                    output.push_str("/");
                    if is_selected {
                        output.push_str("\x1b[0m");
                    }
                }
                DirectoryEntry::AudioFile { name, size } => {
                    if !is_selected {
                        output.push_str("\x1b[94m");
                    }
                    output.push_str(name);
                    output.push_str(" (");
                    output.push_str(&format_file_size(*size));
                    output.push_str(")\x1b[0m");
                }
                DirectoryEntry::File { name, size } => {
                    output.push_str(name);
                    output.push_str(" (");
                    output.push_str(&format_file_size(*size));
                    output.push_str(")");
                    if is_selected {
                        output.push_str("\x1b[0m");
                    }
                }
            }
            
            output.push_str("\r\n");
        }

        if self.entries.is_empty() {
            output.push_str("  (No files to display)\r\n");
        }

        output.push_str("\r\n");
        output
    }

    pub fn render_to_terminal(&self) -> Result<()> {
        // Clear screen and move cursor to top
        print!("\x1b[2J\x1b[H\x1b[0m");

        // Print the rendered content
        print!("{}", self.render());
        io::stdout().flush().map_err(|e| AudioTranscriptionError::FileBrowser(format!("IO error: {}", e)))?;

        Ok(())
    }

    pub fn handle_input(&mut self) -> Result<Option<PathBuf>> {
        loop {
            if let Event::Key(KeyEvent { code, .. }) = event::read()
                .map_err(|e| AudioTranscriptionError::FileBrowser(format!("Input error: {}", e)))? 
            {
                match code {
                    KeyCode::Up => {
                        self.move_selection(Direction::Up);
                        self.render_to_terminal()?;
                    }
                    KeyCode::Down => {
                        self.move_selection(Direction::Down);
                        self.render_to_terminal()?;
                    }
                    KeyCode::Enter => {
                        if let Some(file_path) = self.navigate_selected()? {
                            return Ok(Some(file_path));
                        }
                        self.render_to_terminal()?;
                    }
                    KeyCode::Char('f') | KeyCode::Char('F') => {
                        self.set_audio_filter(!self.filter_audio_only)?;
                        self.render_to_terminal()?;
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                        return Ok(None);
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn run_interactive(&mut self) -> Result<Option<PathBuf>> {
        // Enable raw mode for terminal input
        terminal::enable_raw_mode()
            .map_err(|e| AudioTranscriptionError::FileBrowser(format!("Failed to enable raw mode: {}", e)))?;

        let result = {
            // Clear screen completely and reset terminal state
            print!("\x1b[2J\x1b[H\x1b[0m");
            io::stdout().flush().map_err(|e| AudioTranscriptionError::FileBrowser(format!("IO error: {}", e)))?;
            
            self.render_to_terminal()?;
            self.handle_input()
        };

        // Disable raw mode and clean up terminal before returning
        terminal::disable_raw_mode()
            .map_err(|e| AudioTranscriptionError::FileBrowser(format!("Failed to disable raw mode: {}", e)))?;
        
        // Clear screen on exit
        print!("\x1b[2J\x1b[H\x1b[0m");
        io::stdout().flush().map_err(|e| AudioTranscriptionError::FileBrowser(format!("IO error: {}", e)))?;

        result
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
        self.entries.clear();
        
        // Add parent directory entry if not at root
        if self.current_path.parent().is_some() {
            self.entries.push(DirectoryEntry::Parent);
        }

        // Read directory contents
        let dir_entries = std::fs::read_dir(&self.current_path)
            .map_err(|e| AudioTranscriptionError::FileBrowser(
                format!("Failed to read directory {}: {}", self.current_path.display(), e)
            ))?;

        let mut entries = Vec::new();
        
        for entry in dir_entries {
            let entry = entry.map_err(|e| AudioTranscriptionError::FileBrowser(
                format!("Failed to read directory entry: {}", e)
            ))?;
            
            let path = entry.path();
            let file_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("?")
                .to_string();

            if path.is_dir() {
                entries.push(DirectoryEntry::Directory { name: file_name });
            } else if path.is_file() {
                // Get file size
                let metadata = entry.metadata().map_err(|e| AudioTranscriptionError::FileBrowser(
                    format!("Failed to read file metadata for {}: {}", file_name, e)
                ))?;
                let size = metadata.len();

                if self.filter_audio_only {
                    // Only show supported audio formats when filtering is enabled
                    if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                        if Self::is_supported_audio_format(extension) {
                            entries.push(DirectoryEntry::AudioFile { name: file_name, size });
                        }
                    }
                } else {
                    // Show all files when filtering is disabled
                    if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                        if Self::is_supported_audio_format(extension) {
                            entries.push(DirectoryEntry::AudioFile { name: file_name, size });
                        } else {
                            entries.push(DirectoryEntry::File { name: file_name, size });
                        }
                    } else {
                        entries.push(DirectoryEntry::File { name: file_name, size });
                    }
                }
            }
        }

        // Sort entries: directories first, then audio files, then other files, all alphabetically
        entries.sort_by(|a, b| {
            match (a, b) {
                (DirectoryEntry::Directory { name: a }, DirectoryEntry::Directory { name: b }) => a.cmp(b),
                (DirectoryEntry::AudioFile { name: a, .. }, DirectoryEntry::AudioFile { name: b, .. }) => a.cmp(b),
                (DirectoryEntry::File { name: a, .. }, DirectoryEntry::File { name: b, .. }) => a.cmp(b),
                (DirectoryEntry::Directory { .. }, _) => std::cmp::Ordering::Less,
                (_, DirectoryEntry::Directory { .. }) => std::cmp::Ordering::Greater,
                (DirectoryEntry::AudioFile { .. }, DirectoryEntry::File { .. }) => std::cmp::Ordering::Less,
                (DirectoryEntry::File { .. }, DirectoryEntry::AudioFile { .. }) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            }
        });

        self.entries.extend(entries);
        Ok(())
    }

    fn is_supported_audio_format(extension: &str) -> bool {
        matches!(
            extension.to_lowercase().as_str(),
            "wav" | "mp3" | "m4a" | "flac" | "ogg" | "webm"
        )
    }
}

fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_supported_audio_formats() {
        assert!(FileBrowser::is_supported_audio_format("wav"));
        assert!(FileBrowser::is_supported_audio_format("WAV"));
        assert!(FileBrowser::is_supported_audio_format("mp3"));
        assert!(FileBrowser::is_supported_audio_format("m4a"));
        assert!(FileBrowser::is_supported_audio_format("flac"));
        assert!(FileBrowser::is_supported_audio_format("ogg"));
        assert!(FileBrowser::is_supported_audio_format("webm"));
        
        assert!(!FileBrowser::is_supported_audio_format("txt"));
        assert!(!FileBrowser::is_supported_audio_format("pdf"));
        assert!(!FileBrowser::is_supported_audio_format("mp4"));
    }

    #[test]
    fn test_file_size_formatting() {
        assert_eq!(format_file_size(0), "0 B");
        assert_eq!(format_file_size(512), "512 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1536), "1.5 KB");
        assert_eq!(format_file_size(1024 * 1024), "1.0 MB");
        assert_eq!(format_file_size(1024 * 1024 * 1024), "1.0 GB");
    }

    #[test]
    fn test_file_browser_creation() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let browser = FileBrowser::new(temp_dir.path().to_path_buf())?;
        
        assert_eq!(browser.current_path(), temp_dir.path());
        assert_eq!(browser.selected_index(), 0);
        assert!(browser.is_audio_filter_enabled());
        
        Ok(())
    }

    #[test]
    fn test_directory_navigation() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        let sub_dir = temp_dir.path().join("subdir");
        fs::create_dir(&sub_dir).unwrap();
        
        let mut browser = FileBrowser::new(temp_dir.path().to_path_buf())?;
        
        // Should have parent entry and subdirectory
        assert!(browser.entries().len() >= 1);
        
        // Navigate to subdirectory
        browser.navigate_to(sub_dir.clone())?;
        assert_eq!(browser.current_path(), &sub_dir);
        
        Ok(())
    }

    #[test]
    fn test_audio_filter_toggle() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        
        // Create test files
        fs::write(temp_dir.path().join("audio.wav"), b"fake audio")?;
        fs::write(temp_dir.path().join("document.txt"), b"text content")?;
        
        let mut browser = FileBrowser::new(temp_dir.path().to_path_buf())?;
        
        // With filter enabled, should only see audio files (plus parent)
        let filtered_count = browser.entries().len();
        
        // Disable filter
        browser.set_audio_filter(false)?;
        let unfiltered_count = browser.entries().len();
        
        // Should see more files when filter is disabled
        assert!(unfiltered_count >= filtered_count);
        assert!(!browser.is_audio_filter_enabled());
        
        Ok(())
    }

    #[test]
    fn test_selection_movement() -> Result<()> {
        let temp_dir = TempDir::new().unwrap();
        
        // Create multiple files
        fs::write(temp_dir.path().join("audio1.wav"), b"fake audio 1")?;
        fs::write(temp_dir.path().join("audio2.mp3"), b"fake audio 2")?;
        
        let mut browser = FileBrowser::new(temp_dir.path().to_path_buf())?;
        
        let initial_index = browser.selected_index();
        
        // Move down
        browser.move_selection(Direction::Down);
        assert!(browser.selected_index() >= initial_index);
        
        // Move up
        browser.move_selection(Direction::Up);
        
        Ok(())
    }
}