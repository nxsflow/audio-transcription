// Basic setup test to verify the testing environment is working
import { describe, it, expect } from 'vitest';

describe('Setup', () => {
  it('should have a working test environment', () => {
    expect(true).toBe(true);
  });
  
  it('should be able to import fast-check for property-based testing', async () => {
    const fc = await import('fast-check');
    expect(fc).toBeDefined();
    expect(typeof fc.integer).toBe('function');
  });
});