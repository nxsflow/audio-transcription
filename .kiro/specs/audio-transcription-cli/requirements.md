# Requirements Document

## Introduction

This document specifies the requirements for an audio transcription CLI application with speaker diarization capabilities. The system provides an interactive file browser for selecting audio files and processes them to generate human-readable transcripts with speaker labels using Rust for performance and native GPU acceleration.

## Glossary

- **CLI**: Command Line Interface application
- **VAD**: Voice Activity Detection system for identifying speech segments
- **Diarization**: Process of partitioning audio stream into homogeneous segments according to speaker identity
- **DTW**: Dynamic Time Warping algorithm for word-level timestamp alignment
- **Audio_Processor**: Core system component responsible for audio transcription and diarization
- **File_Browser**: Interactive terminal interface for file navigation and selection
- **Transcript_Generator**: Component responsible for formatting and outputting final transcripts
- **Model_Manager**: Component responsible for downloading and managing ML models
- **Progress_Display**: User interface component showing processing status and progress

## Requirements

### Requirement 1: Interactive File Navigation

**User Story:** As a user, I want to browse and select audio files interactively, so that I can easily choose files for transcription without typing full paths.

#### Acceptance Criteria

1. WHEN the application starts without file arguments, THE File_Browser SHALL display the current directory contents
2. WHEN displaying directory contents, THE File_Browser SHALL show folders with a "[DIR]" prefix and audio files with their file sizes
3. WHEN a user navigates with arrow keys, THE File_Browser SHALL highlight the selected item
4. WHEN a user presses Enter on a directory, THE File_Browser SHALL navigate into that directory
5. WHEN a user selects the ".." entry, THE File_Browser SHALL navigate to the parent directory
6. WHERE audio format filtering is enabled, THE File_Browser SHALL display only directories and supported audio formats (.wav, .mp3, .m4a, .flac, .ogg, .webm)

### Requirement 2: Audio File Processing

**User Story:** As a user, I want my audio files transcribed with speaker identification, so that I can read who said what in meetings and conversations.

#### Acceptance Criteria

1. WHEN an audio file is selected, THE Audio_Processor SHALL load and resample the audio to 16kHz mono format
2. WHEN processing begins, THE Audio_Processor SHALL run VAD to detect speech segments in the full audio
3. WHEN speech segments are detected, THE Audio_Processor SHALL create chunks of approximately 2 minutes duration, splitting at natural pauses of 0.5 seconds or greater
4. WHEN chunks are created, THE Audio_Processor SHALL transcribe chunks in parallel using available CPU cores or GPUs
5. WHEN transcription completes, THE Audio_Processor SHALL run speaker diarization on the full audio file
6. WHEN both transcription and diarization complete, THE Audio_Processor SHALL merge results by assigning speaker IDs to transcribed segments

### Requirement 3: Progress Indication

**User Story:** As a user, I want to see processing progress and time estimates, so that I know how long transcription will take and can track completion.

#### Acceptance Criteria

1. WHEN processing begins, THE Progress_Display SHALL show the current processing stage (VAD, Transcription, Diarization, Merging)
2. WHILE transcription is running, THE Progress_Display SHALL display a progress bar showing completed chunks out of total chunks
3. WHILE any processing stage is active, THE Progress_Display SHALL show elapsed time and estimated time remaining
4. WHEN each stage completes, THE Progress_Display SHALL mark that stage as completed with a checkmark

### Requirement 4: Transcript Output Generation

**User Story:** As a user, I want transcripts formatted with clear speaker labels, so that I can easily follow conversations and identify who spoke when.

#### Acceptance Criteria

1. WHEN processing completes, THE Transcript_Generator SHALL create a .txt file with the same base name as the input audio
2. WHEN formatting transcripts, THE Transcript_Generator SHALL use speaker labels in the format "[SPEAKER_XX]" where XX is a zero-padded two-digit number
3. WHEN the speaker changes, THE Transcript_Generator SHALL print a new speaker label
4. WHEN consecutive sentences are from the same speaker, THE Transcript_Generator SHALL group them together without repeating the speaker label
5. WHEN transitioning between speakers, THE Transcript_Generator SHALL insert an empty line for readability
6. THE Transcript_Generator SHALL preserve punctuation and capitalization from the original transcription

### Requirement 5: Model Management

**User Story:** As a user, I want the application to automatically manage required ML models, so that I don't need to manually download or configure model files.

#### Acceptance Criteria

1. WHEN the application starts, THE Model_Manager SHALL check for required models in the cache directory
2. IF required models are missing, THEN THE Model_Manager SHALL prompt the user for download confirmation
3. WHEN downloading models, THE Model_Manager SHALL display download progress to the user
4. THE Model_Manager SHALL store models in the platform-specific cache directory (~/.cache/audio-transcribe/models/)
5. WHERE model size is specified via command-line flag, THE Model_Manager SHALL use the specified model (tiny, base, small, medium, large). The default should be the medium models.

### Requirement 6: Performance Optimization

**User Story:** As a user, I want fast transcription processing, so that I can process long audio files efficiently without excessive waiting.

#### Acceptance Criteria

1. WHEN transcribing audio, THE Audio_Processor SHALL utilize all available CPU cores for parallel chunk processing
2. WHERE GPU acceleration is available (Metal on macOS, CUDA on Linux/Windows), THE Audio_Processor SHALL use GPU acceleration
3. WHEN processing 1 hour of audio with GPU acceleration, THE Audio_Processor SHALL complete processing in under 10 minutes on modern hardware
4. WHEN processing audio chunks, THE Audio_Processor SHALL process and release chunks to avoid loading the full audio into memory simultaneously

### Requirement 7: Memory Management

**User Story:** As a system administrator, I want the application to use memory efficiently, so that it can run on systems with limited RAM without causing performance issues.

#### Acceptance Criteria

1. WHEN using the base model, THE Audio_Processor SHALL not exceed 4GB peak memory usage
2. WHEN processing audio chunks, THE Audio_Processor SHALL release processed chunks from memory to maintain efficient memory usage
3. THE Audio_Processor SHALL avoid loading the complete audio file into memory when possible

### Requirement 8: Cross-Platform Compatibility

**User Story:** As a user on different operating systems, I want the application to work consistently, so that I can use the same tool regardless of my platform.

#### Acceptance Criteria

1. THE CLI SHALL compile and run on macOS (both ARM and Intel architectures)
2. THE CLI SHALL compile and run on Linux systems
3. THE CLI SHALL compile and run on Windows systems
4. THE CLI SHALL operate without requiring a Python runtime dependency

### Requirement 9: Command Line Interface

**User Story:** As a power user, I want configurable command-line options, so that I can customize the transcription process for my specific needs.

#### Acceptance Criteria

1. WHERE model size is specified, THE CLI SHALL accept --model flag with values (tiny, base, small, medium, large)
2. WHERE output directory is specified, THE CLI SHALL accept --output flag to set the output directory
3. WHERE chunk size is specified, THE CLI SHALL accept --chunk-size flag to set target chunk duration in seconds
4. WHERE GPU acceleration is disabled, THE CLI SHALL accept --no-gpu flag to force CPU-only processing
5. THE CLI SHALL provide --help and --version flags for user assistance

### Requirement 10: Error Handling

**User Story:** As a user, I want clear error messages and graceful failure handling, so that I can understand and resolve issues when they occur.

#### Acceptance Criteria

1. IF required models are missing and download fails, THEN THE CLI SHALL display a clear error message and exit gracefully
2. IF an unsupported audio format is selected, THEN THE CLI SHALL display an error message and return to the file browser
3. IF insufficient memory is available, THEN THE CLI SHALL suggest using a smaller model or fewer parallel jobs
4. IF GPU acceleration is unavailable when requested, THEN THE CLI SHALL fall back to CPU processing with a warning message
5. IF audio file corruption is detected, THEN THE CLI SHALL skip the file with an error message without crashing the application