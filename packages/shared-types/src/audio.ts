/**
 * Audio-related type definitions shared between frontend and backend
 */

import { z } from 'zod';

/**
 * Audio source identifier
 */
export type AudioSource = 'mic' | 'system_audio';

/**
 * Audio source validation schema
 */
export const AudioSourceSchema = z.enum(['mic', 'system_audio']);

/**
 * Audio level update event structure
 */
export interface AudioLevelUpdate {
  /** Audio source identifier */
  source: AudioSource;
  /** Audio level value (0-100 range) */
  level: number;
  /** Unix timestamp when the level was calculated */
  timestamp: number;
}

/**
 * Audio level update validation schema
 */
export const AudioLevelUpdateSchema = z.object({
  source: AudioSourceSchema,
  level: z.number().min(0).max(100),
  timestamp: z.number().positive()
});

/**
 * Current audio levels for both sources
 */
export interface AudioLevels {
  /** Current microphone level (0-100) */
  microphone: number;
  /** Current system audio level (0-100) */
  systemAudio: number;
}

/**
 * Audio levels validation schema
 */
export const AudioLevelsSchema = z.object({
  microphone: z.number().min(0).max(100),
  systemAudio: z.number().min(0).max(100)
});

/**
 * Audio chunk data structure for internal processing
 */
export interface AudioChunk {
  /** Audio buffer data */
  buffer: Float32Array;
  /** Source of the audio data */
  source: AudioSource;
  /** Sample rate of the audio data */
  sampleRate: number;
  /** Timestamp when the chunk was captured */
  timestamp: number;
}

/**
 * Audio error types for error handling
 */
export type AudioErrorType = 
  | 'device_not_found'
  | 'permission_denied'
  | 'unsupported_format'
  | 'stream_error'
  | 'initialization_failed';

/**
 * Audio error structure
 */
export interface AudioError {
  /** Type of the error */
  type: AudioErrorType;
  /** Human-readable error message */
  message: string;
  /** Optional additional details */
  details?: string;
}

/**
 * Audio error validation schema
 */
export const AudioErrorSchema = z.object({
  type: z.enum(['device_not_found', 'permission_denied', 'unsupported_format', 'stream_error', 'initialization_failed']),
  message: z.string(),
  details: z.string().optional()
});

/**
 * Monitoring state information
 */
export interface MonitoringState {
  /** Whether monitoring is currently active */
  isActive: boolean;
  /** Name of the microphone device being used */
  microphoneDevice?: string;
  /** Name of the system audio device being used */
  systemAudioDevice?: string;
  /** Current sample rate */
  sampleRate: number;
  /** Current buffer size */
  bufferSize: number;
}

/**
 * Monitoring state validation schema
 */
export const MonitoringStateSchema = z.object({
  isActive: z.boolean(),
  microphoneDevice: z.string().optional(),
  systemAudioDevice: z.string().optional(),
  sampleRate: z.number().positive(),
  bufferSize: z.number().positive()
});