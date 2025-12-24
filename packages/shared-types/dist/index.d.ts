/**
 * Shared types for Audio Level Monitor
 *
 * This package provides type definitions that are shared between
 * the Rust backend and TypeScript frontend to ensure type safety
 * and consistency across the application.
 */
export type { AudioSource, AudioLevelUpdate, AudioLevels, AudioChunk, AudioErrorType, AudioError, MonitoringState } from './audio.js';
export { AudioSourceSchema, AudioLevelUpdateSchema, AudioLevelsSchema, AudioErrorSchema, MonitoringStateSchema } from './audio.js';
export type { AudioSource as RustAudioSource, AudioLevelUpdate as RustAudioLevelUpdate, AudioLevels as RustAudioLevels, AudioError as RustAudioError, MonitoringState as RustMonitoringState } from './rust-types.js';
//# sourceMappingURL=index.d.ts.map