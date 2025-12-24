/**
 * Audio-related type definitions shared between frontend and backend
 */
import { z } from 'zod';
/**
 * Audio source validation schema
 */
export const AudioSourceSchema = z.enum(['mic', 'system_audio']);
/**
 * Audio level update validation schema
 */
export const AudioLevelUpdateSchema = z.object({
    source: AudioSourceSchema,
    level: z.number().min(0).max(100),
    timestamp: z.number().positive()
});
/**
 * Audio levels validation schema
 */
export const AudioLevelsSchema = z.object({
    microphone: z.number().min(0).max(100),
    systemAudio: z.number().min(0).max(100)
});
/**
 * Audio error validation schema
 */
export const AudioErrorSchema = z.object({
    type: z.enum(['device_not_found', 'permission_denied', 'unsupported_format', 'stream_error', 'initialization_failed']),
    message: z.string(),
    details: z.string().optional()
});
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
//# sourceMappingURL=audio.js.map