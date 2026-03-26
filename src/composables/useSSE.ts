import { ref, onUnmounted } from 'vue';

export interface SSEOptions {
  onMessage?: (data: string) => void;
  onError?: (error: Event) => void;
  onComplete?: () => void;
}

export function useSSE(url: string, options: SSEOptions = {}) {
  const content = ref('');
  const isConnected = ref(false);
  const error = ref<Event | null>(null);
  let eventSource: EventSource | null = null;

  function connect() {
    if (eventSource) {
      eventSource.close();
    }

    content.value = '';
    error.value = null;
    isConnected.value = true;

    eventSource = new EventSource(url);

    eventSource.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        if (data.content) {
          content.value += data.content;
          options.onMessage?.(data.content);
        }
        if (data.done) {
          options.onComplete?.();
          disconnect();
        }
      } catch (e) {
        console.error('SSE 解析错误:', e);
      }
    };

    eventSource.onerror = (e) => {
      error.value = e;
      isConnected.value = false;
      options.onError?.(e);
    };
  }

  function disconnect() {
    if (eventSource) {
      eventSource.close();
      eventSource = null;
    }
    isConnected.value = false;
  }

  function reset() {
    disconnect();
    content.value = '';
    error.value = null;
  }

  onUnmounted(() => {
    disconnect();
  });

  return {
    content,
    isConnected,
    error,
    connect,
    disconnect,
    reset,
  };
}
