# Implementation Plan: Audio Transcription CLI

## Overview

This implementation plan breaks down the audio transcription CLI into discrete, manageable coding tasks. Each task builds incrementally on previous work, with early validation through testing. The plan prioritizes core functionality first, with optional testing tasks marked for flexibility in development approach.

## Tasks

- [x] 1. Set up project structure and dependencies
  - Create Rust project with Cargo.toml including all required dependencies
  - Set up basic CLI argument parsing with clap
  - Create module structure for core components
  - _Requirements: 9.1, 9.2, 9.3, 9.4, 9.5, 9.6_

- [x] 1.1 Write unit tests for CLI argument parsing
  - Test all command-line flags and their validation
  - _Requirements: 9.1, 9.2, 9.3, 9.4, 9.5, 9.6_

- [ ] 2. Implement Model Manager component
  - [ ] 2.1 Create ModelManager struct with cache directory management
    - Implement platform-specific cache directory detection
    - Create directory structure for model storage
    - _Requirements: 5.4_

  - [ ]* 2.2 Write property test for model storage location
    - **Property 12: Model Storage Location**
    - **Validates: Requirements 5.4**

  - [ ] 2.3 Implement model download functionality
    - Add HTTP client for model downloads with progress tracking
    - Implement model validation and integrity checking
    - _Requirements: 5.2, 5.3_

  - [ ]* 2.4 Write unit tests for model download error handling
    - Test download failures and recovery scenarios
    - _Requirements: 10.1_

- [ ] 3. Implement File Browser component
  - [ ] 3.1 Create FileBrowser struct with directory navigation
    - Implement directory listing with file type detection
    - Add support for parent directory navigation
    - _Requirements: 1.1, 1.4, 1.5_

  - [ ]* 3.2 Write property test for directory display formatting
    - **Property 1: Directory Display Formatting**
    - **Validates: Requirements 1.2**

  - [ ] 3.3 Implement audio format filtering
    - Filter directory contents to show only supported audio formats
    - _Requirements: 1.6_

  - [ ]* 3.4 Write property test for audio format filtering
    - **Property 3: Audio Format Filtering**
    - **Validates: Requirements 1.6**

  - [ ] 3.5 Add interactive terminal UI with crossterm
    - Implement arrow key navigation and selection highlighting
    - Add Enter key handling for directory navigation and file selection
    - _Requirements: 1.3_

  - [ ]* 3.6 Write property test for file browser navigation consistency
    - **Property 2: File Browser Navigation Consistency**
    - **Validates: Requirements 1.4, 1.5**

- [ ] 4. Checkpoint - Ensure file browser works independently
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 5. Implement audio processing foundation
  - [ ] 5.1 Create AudioProcessor struct and basic audio loading
    - Implement audio file loading with symphonia
    - Add audio resampling to 16kHz mono using sample rate conversion
    - _Requirements: 2.1_

  - [ ]* 5.2 Write property test for audio resampling consistency
    - **Property 4: Audio Resampling Consistency**
    - **Validates: Requirements 2.1**

  - [ ] 5.3 Implement VAD (Voice Activity Detection) integration
    - Integrate whisper-rs VAD capabilities
    - Return speech segments with timing information
    - _Requirements: 2.2_

  - [ ]* 5.4 Write unit tests for VAD error handling
    - Test VAD with various audio conditions and corrupted files
    - _Requirements: 10.5_

- [ ] 6. Implement chunking algorithm
  - [ ] 6.1 Create chunking logic with natural pause detection
    - Implement algorithm to split audio at silence gaps
    - Target 2-minute chunks with 0.5-second minimum silence
    - _Requirements: 2.3_

  - [ ]* 6.2 Write property test for chunking duration bounds
    - **Property 5: Chunking Duration Bounds**
    - **Validates: Requirements 2.3**

- [ ] 7. Implement parallel transcription engine
  - [ ] 7.1 Create transcription engine with whisper-rs integration
    - Set up whisper model loading and inference
    - Implement word-level timestamp extraction using DTW
    - _Requirements: 2.4_

  - [ ] 7.2 Add parallel chunk processing with rayon
    - Process multiple audio chunks concurrently
    - Collect and merge transcription results
    - _Requirements: 2.4, 6.1_

  - [ ]* 7.3 Write property test for parallel processing utilization
    - **Property 6: Parallel Processing Utilization**
    - **Validates: Requirements 2.4, 6.1**

- [ ] 8. Implement speaker diarization engine
  - [ ] 8.1 Create diarization engine with pyannote-rs integration
    - Set up pyannote model loading and inference
    - Run diarization on full audio with global clustering
    - _Requirements: 2.5_

  - [ ] 8.2 Implement speaker assignment algorithm
    - Merge transcription and diarization results
    - Assign speaker IDs to transcribed segments based on timing overlap
    - _Requirements: 2.6_

  - [ ]* 8.3 Write property test for speaker assignment consistency
    - **Property 7: Speaker Assignment Consistency**
    - **Validates: Requirements 2.6**

- [ ] 9. Checkpoint - Ensure core processing pipeline works
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 10. Implement Progress Display component
  - [ ] 10.1 Create ProgressDisplay with stage tracking
    - Display current processing stage and progress bars
    - Show elapsed time and estimated completion time
    - _Requirements: 3.1, 3.2, 3.3, 3.4_

  - [ ]* 10.2 Write unit tests for progress tracking accuracy
    - Test progress updates and time calculations
    - _Requirements: 3.2, 3.3_

- [ ] 11. Implement Transcript Generator component
  - [ ] 11.1 Create transcript formatting logic
    - Generate output filename based on input audio file
    - Format speaker labels with zero-padded numbering
    - _Requirements: 4.1, 4.2_

  - [ ]* 11.2 Write property test for output file naming
    - **Property 8: Output File Naming**
    - **Validates: Requirements 4.1**

  - [ ]* 11.3 Write property test for speaker label formatting
    - **Property 9: Speaker Label Formatting**
    - **Validates: Requirements 4.2**

  - [ ] 11.4 Implement speaker transition formatting
    - Group consecutive same-speaker segments
    - Add empty lines between speaker changes
    - Preserve original transcription text formatting
    - _Requirements: 4.3, 4.4, 4.5, 4.6_

  - [ ]* 11.5 Write property test for speaker transition formatting
    - **Property 10: Speaker Transition Formatting**
    - **Validates: Requirements 4.3, 4.4, 4.5**

  - [ ]* 11.6 Write property test for text preservation
    - **Property 11: Text Preservation**
    - **Validates: Requirements 4.6**

- [ ] 12. Implement memory management and optimization
  - [ ] 12.1 Add memory-efficient chunk processing
    - Implement chunk release after processing
    - Avoid loading full audio into memory simultaneously
    - _Requirements: 6.4, 7.2, 7.3_

  - [ ]* 12.2 Write property test for memory management efficiency
    - **Property 14: Memory Management Efficiency**
    - **Validates: Requirements 6.4, 7.2, 7.3**

- [ ] 13. Implement GPU acceleration support
  - [ ] 13.1 Add GPU detection and Metal/CUDA support
    - Detect available GPU acceleration (Metal on macOS, CUDA on Linux/Windows)
    - Implement fallback to CPU when GPU unavailable
    - _Requirements: 6.2, 10.4_

  - [ ]* 13.2 Write unit tests for GPU fallback behavior
    - Test GPU unavailable scenarios and fallback handling
    - _Requirements: 10.4_

- [ ] 14. Implement comprehensive error handling
  - [ ] 14.1 Add error handling for unsupported audio formats
    - Detect unsupported formats and return to file browser
    - _Requirements: 10.2_

  - [ ] 14.2 Add memory constraint detection and suggestions
    - Detect insufficient memory conditions
    - Suggest smaller models or fewer parallel jobs
    - _Requirements: 10.3_

  - [ ]* 14.3 Write unit tests for error handling scenarios
    - Test various error conditions and recovery mechanisms
    - _Requirements: 10.1, 10.2, 10.5_

- [ ] 15. Integrate all components and implement main application flow
  - [ ] 15.1 Wire together file browser, audio processor, and output generation
    - Connect all components in main application loop
    - Implement command-line flag processing and configuration
    - _Requirements: 9.2, 9.3, 9.4, 9.5_

  - [ ]* 15.2 Write property test for command-line configuration
    - **Property 15: Command-Line Configuration**
    - **Validates: Requirements 9.2, 9.3, 9.4, 9.5**

  - [ ]* 15.3 Write property test for command-line model selection
    - **Property 13: Command-Line Model Selection**
    - **Validates: Requirements 5.5, 9.1**

- [ ] 16. Final integration testing and validation
  - [ ] 16.1 Create end-to-end integration tests
    - Test complete processing pipeline with sample audio files
    - Verify output format and content accuracy
    - _Requirements: All requirements_

  - [ ]* 16.2 Write performance benchmarks
    - Benchmark parallel vs sequential processing
    - Memory usage profiling during processing
    - _Requirements: 6.1, 6.4_

- [ ] 17. Final checkpoint - Complete system validation
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP development
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation and provide opportunities for user feedback
- Property tests validate universal correctness properties across many inputs
- Unit tests validate specific examples, edge cases, and error conditions
- The implementation prioritizes core functionality first, with comprehensive testing available as optional tasks