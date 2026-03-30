import { describe, it, expect, beforeEach } from 'vitest';
import { useErrorStore } from './errorStore';

describe('errorStore', () => {
  beforeEach(() => {
    useErrorStore.setState({ currentError: null });
  });

  it('setError updates currentError', () => {
    useErrorStore.getState().setError('Test error');
    expect(useErrorStore.getState().currentError).toBe('Test error');
  });

  it('clearError resets to null', () => {
    useErrorStore.getState().setError('Something went wrong');
    useErrorStore.getState().clearError();
    expect(useErrorStore.getState().currentError).toBeNull();
  });
});
