/**
 * Type definitions specifically for Rust code generation using ts-rs
 * 
 * This file contains type definitions that will be used to generate
 * corresponding Rust types using the ts-rs library.
 */

/**
 * Audio source for Rust generation
 * @ts-rs-export AudioSource
 */
export type AudioSource = 'mic' | 'system_audio';

/**
 * Audio level update for Rust generation
 * @ts-rs-export AudioLevelUpdate
 */
export interface AudioLevelUpdate {
  source: AudioSource;
  level: number;
  timestamp: number;
}

/**
 * Audio levels for Rust generation
 * @ts-rs-export AudioLevels
 */
export interface AudioLevels {
  microphone: number;
  systemAudio: number;
}

/**
 * Audio error for Rust generation
 * @ts-rs-export AudioError
 */
export interface AudioError {
  type: 'device_not_found' | 'permission_denied' | 'unsupported_format' | 'stream_error' | 'initialization_failed';
  message: string;
  details?: string;
}

/**
 * Monitoring state for Rust generation
 * @ts-rs-export MonitoringState
 */
export interface MonitoringState {
  isActive: boolean;
  microphoneDevice?: string;
  systemAudioDevice?: string;
  sampleRate: number;
  bufferSize: number;
}