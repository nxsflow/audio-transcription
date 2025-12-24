# Implementation Plan: Audio Level Monitor

## Overview

This implementation plan breaks down the audio level monitor feature into discrete coding tasks that build incrementally within a monorepo workspace structure. The approach begins with workspace setup, then follows a backend-first strategy, implementing the Rust audio processing components before the React frontend, ensuring core functionality is solid before adding the user interface.

The monorepo structure enables shared type definitions, unified tooling, and future scalability for additional applications and packages.

## Tasks

- [x] 1. Set up project structure and dependencies
  - Initialize Tauri project with audio processing capabilities
  - Add required Rust dependencies (cpal, tokio, serde)
  - Add required frontend dependencies (using Tanstack Start)
  - Configure Tauri permissions for audio access
  - _Requirements: 5.1, 5.2, 5.5_

- [x] 2. Reorganize as monorepo workspace
  - [x] 2.1 Create root workspace configuration
    - Create root package.json with workspace scripts
    - Create root pnpm-workspace.yaml configuration
    - Set up unified tooling and scripts
    - _Requirements: 7.1, 7.4, 7.5_

  - [x] 2.2 Restructure existing code into apps/desktop
    - Move src-react to apps/tauri-react/src
    - Move src-tauri to apps/desktop/
    - Update Tauri configuration paths
    - Update package.json references
    - _Requirements: 7.2_

  - [x] 2.3 Create shared-types package
    - Create packages/shared-types directory structure
    - Set up TypeScript configuration for shared types
    - Add ts-rs for Rust type generation
    - Create initial audio type definitions
    - _Requirements: 7.3_

  - [x] 2.4 Verify workspace functionality
    - Test workspace dependency resolution
    - Verify unified scripts work correctly
    - Ensure Tauri build process works with new structure
    - _Requirements: 7.4, 7.6_

- [ ] 3. Implement core audio data structures and error handling
  - [ ] 3.1 Create AudioSource enum and AudioChunk struct in shared types
    - Define AudioSource enum in packages/shared-types
    - Implement AudioChunk struct with buffer, source, sample_rate, timestamp
    - Generate corresponding Rust types using ts-rs
    - _Requirements: 1.1, 2.1, 7.3_

  - [ ] 3.2 Implement AudioError enum and error handling
    - Create comprehensive AudioError enum for all error conditions
    - Implement error conversion traits and display formatting
    - _Requirements: 5.3_

  - [ ] 3.3 Write unit tests for data structures
    - Test AudioChunk creation and serialization
    - Test AudioError formatting and conversion
    - Test shared type consistency between TypeScript and Rust
    - _Requirements: 2.1, 5.3, 7.3_

- [ ] 4. Implement LevelMeterBackend
  - [ ] 4.1 Create LevelMeterBackend struct with level calculation methods
  - [ ] 4.1 Create LevelMeterBackend struct with level calculation methods
    - Implement RMS and peak level calculation functions
    - Add level smoothing functionality
    - Create event emission system for level updates using shared types
    - _Requirements: 1.2, 2.2, 3.3, 7.3_

  - [ ] 4.2 Write property test for level calculations
    - **Property 2: Audio processing pipeline**
    - **Validates: Requirements 1.2, 1.3, 2.2, 2.3**

  - [ ] 4.3 Write property test for silent audio handling
    - **Property 3: Silent audio handling**
    - **Validates: Requirements 1.4, 2.4**

- [ ] 5. Implement MicrophoneProcessor
  - [ ] 5.1 Create MicrophoneProcessor with device initialization
    - Implement microphone device discovery and setup
    - Add audio stream configuration for microphone input
    - Implement fire-and-forget chunk sending to LevelMeterBackend using shared types
    - _Requirements: 1.1, 5.1, 5.4, 7.3_

  - [ ] 5.2 Write property test for microphone capture initiation
    - **Property 1: Audio capture initiation**
    - **Validates: Requirements 1.1, 2.1**

  - [ ] 5.3 Write unit tests for microphone error handling
    - Test device not found scenarios
    - Test permission denied scenarios
    - _Requirements: 5.3, 5.4_

- [ ] 6. Implement SystemAudioProcessor
  - [ ] 6.1 Create SystemAudioProcessor with loopback capture
    - Implement system audio loopback setup for each platform
    - Add audio stream configuration for system audio capture
    - Implement fire-and-forget chunk sending to LevelMeterBackend using shared types
    - _Requirements: 2.1, 5.2, 7.3_

  - [ ] 6.2 Write property test for error handling
    - **Property 8: Error handling gracefully**
    - **Validates: Requirements 5.3**

  - [ ] 6.3 Write unit tests for system audio capture
    - Test loopback initialization on different platforms
    - Test audio format compatibility
    - _Requirements: 5.2_

- [ ] 7. Checkpoint - Ensure backend audio processing tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 8. Implement AudioManager coordination layer
  - [ ] 8.1 Create AudioManager with start/stop functionality
    - Implement centralized audio session management
    - Coordinate microphone and system audio processors
    - Handle resource cleanup and state transitions
    - _Requirements: 4.1, 4.2, 4.3, 4.4_

  - [ ] 8.2 Write property test for state transitions
    - **Property 6: State transition consistency**
    - **Validates: Requirements 4.3, 4.4**

  - [ ] 8.3 Write property test for resource management
    - **Property 11: Resource management**
    - **Validates: Requirements 6.3**

- [ ] 9. Implement Tauri commands and events
  - [ ] 9.1 Create Tauri command handlers using shared types
    - Implement start_audio_monitoring command
    - Implement stop_audio_monitoring command
    - Add error handling and response formatting using shared types
    - _Requirements: 4.1, 4.2, 7.3_

  - [ ] 9.2 Implement audio level event emission using shared types
    - Create AudioLevelUpdate event structure in shared types
    - Implement event emission from LevelMeterBackend
    - Configure event routing to frontend
    - _Requirements: 1.3, 2.3, 3.3, 7.3_

  - [ ] 9.3 Write integration tests for Tauri interface
    - Test command execution and response handling
    - Test event emission and delivery
    - Test shared type consistency across Tauri bridge
    - _Requirements: 4.3, 4.4, 7.3_

- [ ] 10. Implement React frontend components
  - [ ] 10.1 Create LevelMeter component using shared types
    - Implement visual level meter with animated bars
    - Add peak level indication and color coding
    - Handle level updates from Tauri events using shared types
    - _Requirements: 3.1, 3.2, 3.4, 7.3_

  - [ ] 10.2 Create ControlPanel component
    - Implement start/stop buttons with state management
    - Add loading states and error display
    - Connect to Tauri commands using shared types
    - _Requirements: 4.1, 4.2, 7.3_

  - [ ] 10.3 Create main AudioLevelMonitor component
    - Integrate LevelMeter and ControlPanel components
    - Implement event subscription for level updates using shared types
    - Add application state management
    - _Requirements: 3.1, 4.5, 7.3_

  - [ ] 10.4 Write property test for UI responsiveness
    - **Property 12: Non-blocking audio processing**
    - **Validates: Requirements 6.4**

  - [ ] 10.5 Write property test for workspace type consistency
    - **Property 13: Workspace type consistency**
    - **Validates: Requirements 7.3**

- [ ] 11. Implement performance optimizations
  - [ ] 11.1 Add level update throttling and smoothing
    - Implement 30 FPS update rate limiting
    - Add level smoothing to prevent jittery displays
    - Optimize event emission frequency
    - _Requirements: 6.1, 3.5_

  - [ ] 11.2 Write property test for update frequency
    - **Property 10: Update frequency performance**
    - **Validates: Requirements 6.1**

  - [ ] 11.3 Write unit tests for performance features
    - Test update rate limiting functionality
    - Test level smoothing algorithms
    - _Requirements: 6.1_

- [ ] 12. Implement no-storage guarantee
  - [ ] 12.1 Add explicit no-storage validation
    - Ensure no audio data is written to disk
    - Implement memory-only processing pipeline
    - Add validation that no persistent audio storage occurs
    - _Requirements: 1.5, 2.5_

  - [ ] 12.2 Write property test for no persistent storage
    - **Property 4: No persistent storage**
    - **Validates: Requirements 1.5, 2.5**

- [ ] 13. Final integration and testing
  - [ ] 13.1 Wire all components together
    - Connect frontend to backend through Tauri bridge using shared types
    - Ensure proper event flow from processors to UI
    - Implement complete start-to-stop workflow
    - _Requirements: All requirements_

  - [ ] 13.2 Write property test for stopped state display
    - **Property 7: Stopped state display**
    - **Validates: Requirements 4.5**

  - [ ] 13.3 Write integration tests for complete workflow
    - Test end-to-end monitoring session
    - Test error recovery scenarios
    - Test cross-platform compatibility
    - Test monorepo workspace functionality
    - _Requirements: 5.1, 5.2, 6.2, 7.4, 7.6_

- [ ] 14. Final checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- Task 2 focuses on monorepo workspace setup and restructuring
- All subsequent tasks reference shared types for consistency
- Backend implementation precedes frontend to ensure solid foundation
- Property tests validate universal correctness properties including workspace type consistency
- Unit tests validate specific examples and edge cases
- Integration tests ensure components work together correctly across the monorepo structure
- Shared types package ensures type safety between frontend and backend