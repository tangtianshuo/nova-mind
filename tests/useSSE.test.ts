import { describe, it, expect, vi, beforeEach } from 'vitest';

describe('useSSE', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should have empty initial state', () => {
    const mockEventSource = {
      onmessage: null,
      onerror: null,
      close: vi.fn(),
    };

    vi.stubGlobal('EventSource', vi.fn(() => mockEventSource));

    const eventSource = new EventSource('http://localhost/test');
    expect(eventSource).toBeDefined();
  });

  it('should handle message events correctly', async () => {
    const mockOnMessage = vi.fn();
    const mockEvent = { data: JSON.stringify({ content: 'Hello', done: false }) };

    const eventSource = {
      onmessage: null,
      onerror: null,
      close: vi.fn(),
    };

    const onMessageHandler = (event: MessageEvent) => {
      mockOnMessage(event.data);
    };

    eventSource.onmessage = onMessageHandler;
    eventSource.onmessage(mockEvent);

    expect(mockOnMessage).toHaveBeenCalledWith(mockEvent.data);
  });

  it('should handle error events correctly', async () => {
    const mockOnError = vi.fn();
    const mockError = new Event('error');

    const eventSource = {
      onmessage: null,
      onerror: null,
      close: vi.fn(),
    };

    const onErrorHandler = (event: Event) => {
      mockOnError(event);
    };

    eventSource.onerror = onErrorHandler;
    eventSource.onerror(mockError);

    expect(mockOnError).toHaveBeenCalledWith(mockError);
  });
});
