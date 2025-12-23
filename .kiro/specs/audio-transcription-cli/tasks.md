# Implementation Plan: Audio Transcription CLI

## Overview

This implementation plan breaks down the audio transcription CLI into discrete, manageable coding tasks with **incremental validation**. Each major task produces a visible, testable result when running `cargo run`, allowing early detection of issues and continuous validation of the implementation approach.

## Tasks

- [x] 1. Set up project structure and dependencies
  - Create Rust project with Cargo.toml including all required dependencies
  - Set up basic CLI argument parsing with clap
  - Create module structure for core components
  - _Requirements: 9.1, 9.2, 9.3, 9.4, 9.5, 9.6_
  - **Validation**: `cargo run --help` shows command-line options

  - [x] 1.1 Write unit tests for CLI argument parsing
    - Test all command-line flags and their validation
    - _Requirements: 9.1, 9.2, 9.3, 9.4, 9.5, 9.6_

- [x] 2. Implement Model Manager with integration
  - [x] 2.1 Create ModelManager struct with cache directory management
    - Implement platform-specific cache directory detection
    - Create directory structure for model storage
    - _Requirements: 5.4_

  - [x] 2.2 Write property test for model storage location
    - **Property 12: Model Storage Location**
    - **Validates: Requirements 5.4**

  - [x] 2.3 Implement model download functionality
    - Add HTTP client for model downloads with progress tracking
    - Implement model validation and integrity checking
    - _Requirements: 5.2, 5.3_

  - [x] 2.4 Write unit tests for model download error handling
    - Test download failures and recovery scenarios
    - _Requirements: 10.1_

  - [x] 2.5 Integrate model checking into file browser flow
    - When audio file is selected, check if required models exist
    - Prompt user to download missing models with progress display
    - Return to file browser after successful download or cancellation
    - _Requirements: 5.1, 5.2, 5.3_
    - **Validation**: `cargo run` → select audio file → see model download prompt

- [ ] 3. Complete File Browser component with model integration
  - [x] 3.1 Create FileBrowser struct with directory navigation
    - Implement directory listing with file type detection
    - Add support for parent directory navigation
    - _Requirements: 1.1, 1.4, 1.5_

  - [ ] 3.2 Write property test for directory display formatting
    - **Property 1: Directory Display Formatting**
    - **Validates: Requirements 1.2**

  - [x] 3.3 Implement audio format filtering
    - Filter directory contents to show only supported audio formats
    - _Requirements: 1.6_

  - [ ] 3.4 Write property test for audio format filtering
    - **Property 3: Audio Format Filtering**
    - **Validates: Requirements 1.6**

  - [x] 3.5 Add interactive terminal UI with crossterm
    - Implement arrow key navigation and selection highlighting
    - Add Enter key handling for directory navigation and file selection
    - _Requirements: 1.3_

  - [ ] 3.6 Write property test for file browser navigation consistency
    - **Property 2: File Browser Navigation Consistency**
    - **Validates: Requirements 1.4, 1.5**

- [ ] 4. Implement basic audio processing with immediate feedback
  - [ ] 4.1 Create AudioProcessor struct and basic audio loading
    - Implement audio file loading with symphonia
    - Add audio resampling to 16kHz mono using sample rate conversion
    - Display basic audio info (duration, format, sample rate)
    - _Requirements: 2.1_
    - **Validation**: `cargo run` → select audio file → see "Processing audio..." → see audio info

  - [ ]* 4.2 Write property test for audio resampling consistency
    - **Property 4: Audio Resampling Consistency**
    - **Validates: Requirements 2.1**

  - [ ] 4.3 Implement VAD (Voice Activity Detection) with progress display
    - Integrate whisper-rs VAD capabilities
    - Return speech segments with timing information
    - Show progress: "Detecting speech segments... X segments found"
    - _Requirements: 2.2_
    - **Validation**: `cargo run` → select audio file → see VAD progress and results

  - [ ]* 4.4 Write unit tests for VAD error handling
    - Test VAD with various audio conditions and corrupted files
    - _Requirements: 10.5_

- [ ] 5. Implement chunking with visual feedback
  - [ ] 5.1 Create chunking logic with natural pause detection
    - Implement algorithm to split audio at silence gaps
    - Target 2-minute chunks with 0.5-second minimum silence
    - Display: "Creating chunks... X chunks created (avg Y seconds each)"
    - _Requirements: 2.3_
    - **Validation**: `cargo run` → select audio file → see chunking progress and statistics

  - [ ]* 5.2 Write property test for chunking duration bounds
    - **Property 5: Chunking Duration Bounds**
    - **Validates: Requirements 2.3**

- [ ] 6. Implement basic transcription with mock output
  - [ ] 6.1 Create transcription engine foundation
    - Set up whisper model loading and basic inference
    - For now, generate mock transcription output to validate pipeline
    - Display: "Transcribing chunk X/Y..." with progress
    - _Requirements: 2.4_
    - **Validation**: `cargo run` → select audio file → see transcription progress → get mock transcript file

  - [ ] 6.2 Add parallel chunk processing with rayon
    - Process multiple audio chunks concurrently
    - Show parallel progress: "Processing 4 chunks in parallel..."
    - Collect and merge transcription results
    - _Requirements: 2.4, 6.1_
    - **Validation**: See multiple chunks processing simultaneously

  - [ ]* 6.3 Write property test for parallel processing utilization
    - **Property 6: Parallel Processing Utilization**
    - **Validates: Requirements 2.4, 6.1**

- [ ] 7. Implement real transcription with whisper-rs
  - [ ] 7.1 Replace mock transcription with real whisper inference
    - Implement word-level timestamp extraction using DTW
    - Generate actual transcription text from audio
    - _Requirements: 2.4_
    - **Validation**: `cargo run` → select audio file → get real transcription output

  - [ ] 7.2 Add Progress Display component integration
    - Display current processing stage and progress bars
    - Show elapsed time and estimated completion time
    - _Requirements: 3.1, 3.2, 3.3, 3.4_
    - **Validation**: See detailed progress with time estimates

  - [ ]* 7.3 Write unit tests for progress tracking accuracy
    - Test progress updates and time calculations
    - _Requirements: 3.2, 3.3_

- [ ] 8. Implement speaker diarization with basic output
  - [ ] 8.1 Create diarization engine with pyannote-rs integration
    - Set up pyannote model loading and inference
    - Run diarization on full audio with global clustering
    - Display: "Identifying speakers... X speakers detected"
    - _Requirements: 2.5_
    - **Validation**: `cargo run` → select audio file → see speaker detection progress

  - [ ] 8.2 Implement speaker assignment algorithm
    - Merge transcription and diarization results
    - Assign speaker IDs to transcribed segments based on timing overlap
    - _Requirements: 2.6_
    - **Validation**: Generated transcript shows speaker labels

  - [ ]* 8.3 Write property test for speaker assignment consistency
    - **Property 7: Speaker Assignment Consistency**
    - **Validates: Requirements 2.6**

- [ ] 9. Implement Transcript Generator with formatting
  - [ ] 9.1 Create transcript formatting logic
    - Generate output filename based on input audio file
    - Format speaker labels with zero-padded numbering
    - _Requirements: 4.1, 4.2_
    - **Validation**: Generated transcript file has proper naming and speaker labels

  - [ ]* 9.2 Write property test for output file naming
    - **Property 8: Output File Naming**
    - **Validates: Requirements 4.1**

  - [ ]* 9.3 Write property test for speaker label formatting
    - **Property 9: Speaker Label Formatting**
    - **Validates: Requirements 4.2**

  - [ ] 9.4 Implement speaker transition formatting
    - Group consecutive same-speaker segments
    - Add empty lines between speaker changes
    - Preserve original transcription text formatting
    - _Requirements: 4.3, 4.4, 4.5, 4.6_
    - **Validation**: Generated transcript has proper speaker transitions and formatting

  - [ ]* 9.5 Write property test for speaker transition formatting
    - **Property 10: Speaker Transition Formatting**
    - **Validates: Requirements 4.3, 4.4, 4.5**

  - [ ]* 9.6 Write property test for text preservation
    - **Property 11: Text Preservation**
    - **Validates: Requirements 4.6**

- [ ] 10. Implement performance optimizations
  - [ ] 10.1 Add memory-efficient chunk processing
    - Implement chunk release after processing
    - Avoid loading full audio into memory simultaneously
    - Display memory usage information
    - _Requirements: 6.4, 7.2, 7.3_
    - **Validation**: See memory usage stats during processing

  - [ ]* 10.2 Write property test for memory management efficiency
    - **Property 14: Memory Management Efficiency**
    - **Validates: Requirements 6.4, 7.2, 7.3**

  - [ ] 10.3 Add GPU acceleration support
    - Detect available GPU acceleration (Metal on macOS, CUDA on Linux/Windows)
    - Implement fallback to CPU when GPU unavailable
    - Display: "Using GPU acceleration" or "Using CPU (GPU unavailable)"
    - _Requirements: 6.2, 10.4_
    - **Validation**: See GPU/CPU usage indication

  - [ ]* 10.4 Write unit tests for GPU fallback behavior
    - Test GPU unavailable scenarios and fallback handling
    - _Requirements: 10.4_

- [ ] 11. Implement comprehensive error handling with user feedback
  - [ ] 11.1 Add error handling for unsupported audio formats
    - Detect unsupported formats and return to file browser
    - Display clear error messages
    - _Requirements: 10.2_
    - **Validation**: Select unsupported file → see error → return to browser

  - [ ] 11.2 Add memory constraint detection and suggestions
    - Detect insufficient memory conditions
    - Suggest smaller models or fewer parallel jobs
    - _Requirements: 10.3_
    - **Validation**: Trigger memory constraint → see helpful suggestion

  - [ ]* 11.3 Write unit tests for error handling scenarios
    - Test various error conditions and recovery mechanisms
    - _Requirements: 10.1, 10.2, 10.5_

- [ ] 12. Final integration and command-line enhancements
  - [ ] 12.1 Implement command-line flag processing
    - Add support for --model, --output, --chunk-size, --no-gpu flags
    - Integrate flags with processing pipeline
    - _Requirements: 9.2, 9.3, 9.4, 9.5_
    - **Validation**: `cargo run --model tiny` → see tiny model being used

  - [ ]* 12.2 Write property test for command-line configuration
    - **Property 15: Command-Line Configuration**
    - **Validates: Requirements 9.2, 9.3, 9.4, 9.5**

  - [ ]* 12.3 Write property test for command-line model selection
    - **Property 13: Command-Line Model Selection**
    - **Validates: Requirements 5.5, 9.1**

- [ ] 13. Final integration testing and validation
  - [ ] 13.1 Create end-to-end integration tests
    - Test complete processing pipeline with sample audio files
    - Verify output format and content accuracy
    - _Requirements: All requirements_

  - [ ]* 13.2 Write performance benchmarks
    - Benchmark parallel vs sequential processing
    - Memory usage profiling during processing
    - _Requirements: 6.1, 6.4_

## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP development
- Each task references specific requirements for traceability
- **Validation** sections ensure incremental validation through `cargo run`
- Property tests validate universal correctness properties across many inputs
- Unit tests validate specific examples, edge cases, and error conditions
- The implementation prioritizes core functionality first, with comprehensive testing available as optional tasks
- Each major task should produce visible results when running the application