/**
 * Shared types for Audio Level Monitor
 * 
 * This package provides type definitions that are shared between
 * the Rust backend and TypeScript frontend to ensure type safety
 * and consistency across the application.
 */

// Audio-related types and schemas
export type {
  AudioSource,
  AudioLevelUpdate,
  AudioLevels,
  AudioChunk,
  AudioErrorType,
  AudioError,
  MonitoringState
} from './audio.js';

export {
  AudioSourceSchema,
  AudioLevelUpdateSchema,
  AudioLevelsSchema,
  AudioErrorSchema,
  MonitoringStateSchema
} from './audio.js';

// Re-export rust types for ts-rs generation
export type {
  AudioSource as RustAudioSource,
  AudioLevelUpdate as RustAudioLevelUpdate,
  AudioLevels as RustAudioLevels,
  AudioError as RustAudioError,
  MonitoringState as RustMonitoringState
} from './rust-types.js';