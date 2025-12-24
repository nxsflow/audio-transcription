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
export declare const AudioSourceSchema: z.ZodEnum<{
    mic: "mic";
    system_audio: "system_audio";
}>;
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
export declare const AudioLevelUpdateSchema: z.ZodObject<{
    source: z.ZodEnum<{
        mic: "mic";
        system_audio: "system_audio";
    }>;
    level: z.ZodNumber;
    timestamp: z.ZodNumber;
}, z.core.$strip>;
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
export declare const AudioLevelsSchema: z.ZodObject<{
    microphone: z.ZodNumber;
    systemAudio: z.ZodNumber;
}, z.core.$strip>;
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
export type AudioErrorType = 'device_not_found' | 'permission_denied' | 'unsupported_format' | 'stream_error' | 'initialization_failed';
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
export declare const AudioErrorSchema: z.ZodObject<{
    type: z.ZodEnum<{
        device_not_found: "device_not_found";
        permission_denied: "permission_denied";
        unsupported_format: "unsupported_format";
        stream_error: "stream_error";
        initialization_failed: "initialization_failed";
    }>;
    message: z.ZodString;
    details: z.ZodOptional<z.ZodString>;
}, z.core.$strip>;
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
export declare const MonitoringStateSchema: z.ZodObject<{
    isActive: z.ZodBoolean;
    microphoneDevice: z.ZodOptional<z.ZodString>;
    systemAudioDevice: z.ZodOptional<z.ZodString>;
    sampleRate: z.ZodNumber;
    bufferSize: z.ZodNumber;
}, z.core.$strip>;
//# sourceMappingURL=audio.d.ts.map