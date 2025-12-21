use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub enum ProcessingStage {
    VAD,
    Chunking,
    Transcription,
    Diarization,
    Merging,
    Complete,
}

impl std::fmt::Display for ProcessingStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessingStage::VAD => write!(f, "Voice Activity Detection"),
            ProcessingStage::Chunking => write!(f, "Audio Chunking"),
            ProcessingStage::Transcription => write!(f, "Transcription"),
            ProcessingStage::Diarization => write!(f, "Speaker Diarization"),
            ProcessingStage::Merging => write!(f, "Merging Results"),
            ProcessingStage::Complete => write!(f, "Complete"),
        }
    }
}

pub struct ProgressDisplay {
    current_stage: ProcessingStage,
    total_chunks: usize,
    completed_chunks: usize,
    start_time: Instant,
}

impl ProgressDisplay {
    pub fn new() -> Self {
        Self {
            current_stage: ProcessingStage::VAD,
            total_chunks: 0,
            completed_chunks: 0,
            start_time: Instant::now(),
        }
    }

    pub fn set_stage(&mut self, stage: ProcessingStage) {
        log::info!("Processing stage: {}", stage);
        self.current_stage = stage;
    }

    pub fn update_progress(&mut self, completed: usize, total: usize) {
        self.completed_chunks = completed;
        self.total_chunks = total;
        
        if total > 0 {
            let percentage = (completed as f32 / total as f32) * 100.0;
            log::debug!("Progress: {}/{} ({:.1}%)", completed, total, percentage);
        }
    }

    pub fn render(&self) -> String {
        // TODO: Implement rich terminal progress display
        // This will be implemented in task 10
        let elapsed = self.start_time.elapsed();
        let elapsed_secs = elapsed.as_secs();
        
        let progress_info = if self.total_chunks > 0 {
            let percentage = (self.completed_chunks as f32 / self.total_chunks as f32) * 100.0;
            format!(" ({}/{} - {:.1}%)", self.completed_chunks, self.total_chunks, percentage)
        } else {
            String::new()
        };

        format!(
            "Stage: {}{} | Elapsed: {}:{:02}",
            self.current_stage,
            progress_info,
            elapsed_secs / 60,
            elapsed_secs % 60
        )
    }

    pub fn current_stage(&self) -> &ProcessingStage {
        &self.current_stage
    }

    pub fn elapsed_time(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }

    pub fn estimate_remaining(&self) -> Option<std::time::Duration> {
        if self.completed_chunks > 0 && self.total_chunks > self.completed_chunks {
            let elapsed = self.start_time.elapsed();
            let avg_time_per_chunk = elapsed.as_secs_f32() / self.completed_chunks as f32;
            let remaining_chunks = self.total_chunks - self.completed_chunks;
            let estimated_remaining_secs = avg_time_per_chunk * remaining_chunks as f32;
            
            Some(std::time::Duration::from_secs_f32(estimated_remaining_secs))
        } else {
            None
        }
    }
}

impl Default for ProgressDisplay {
    fn default() -> Self {
        Self::new()
    }
}