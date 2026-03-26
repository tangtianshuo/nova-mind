import { ref, onMounted, onUnmounted } from 'vue';
import { useConversationStore } from '@/stores/conversationStore';
import { openclawApi, GatewayStatus } from '@/api';

export function useAIChat() {
  const conversationStore = useConversationStore();
  const isStreaming = ref(false);
  const error = ref<string | null>(null);
  const gatewayStatus = ref<GatewayStatus | null>(null);
  const abortController = ref<AbortController | null>(null);
  let statusCheckInterval: ReturnType<typeof setInterval> | null = null;
  const unlistenRef = ref<(() => void) | null>(null);

  async function checkGatewayStatus() {
    try {
      const status = await openclawApi.checkStatus();
      gatewayStatus.value = status;
      return status;
    } catch (e) {
      console.error('检查 Gateway 状态失败:', e);
      return null;
    }
  }

  async function connect() {
    try {
      const result = await openclawApi.connect();
      if (!result.success) {
        error.value = result.error || '连接失败';
      }
      await checkGatewayStatus();
      return result;
    } catch (e) {
      error.value = e instanceof Error ? e.message : '连接失败';
      return { success: false, error: error.value };
    }
  }

  async function retryConnect() {
    try {
      const result = await openclawApi.retryConnect();
      if (!result.success) {
        error.value = result.error || '重连失败';
      }
      await checkGatewayStatus();
      return result;
    } catch (e) {
      error.value = e instanceof Error ? e.message : '重连失败';
      return { success: false, error: error.value };
    }
  }

  async function sendMessage(conversationId: number, content: string) {
    isStreaming.value = true;
    error.value = null;

    await conversationStore.sendMessage(conversationId, content, 'user');

    const status = await checkGatewayStatus();
    if (!status?.connected) {
      error.value = 'OpenClaw Gateway 未连接，请先启动 Gateway';
      isStreaming.value = false;
      return;
    }

    abortController.value = new AbortController();

    try {
      const response = await fetch(
        `http://localhost:18789/api/agent`,
        {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({
            conversation_id: conversationId,
            message: content,
          }),
          signal: abortController.value.signal,
        }
      );

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }

      const reader = response.body?.getReader();
      const decoder = new TextDecoder();

      if (!reader) {
        throw new Error('无法读取响应流');
      }

      let fullContent = '';

      while (true) {
        const { done, value } = await reader.read();
        if (done) break;

        const chunk = decoder.decode(value, { stream: true });
        const lines = chunk.split('\n');

        for (const line of lines) {
          if (line.startsWith('data: ')) {
            const data = line.slice(6);
            if (data === '[DONE]') {
              break;
            }
            try {
              const parsed = JSON.parse(data);
              if (parsed.content) {
                fullContent += parsed.content;
              }
              if (parsed.delta) {
                fullContent += parsed.delta;
              }
            } catch {
              // ignore parse errors
            }
          }
        }
      }

      if (fullContent) {
        await conversationStore.sendMessage(conversationId, fullContent, 'assistant');
      }
    } catch (e) {
      if (e instanceof Error) {
        if (e.name === 'AbortError') {
          console.log('请求已取消');
        } else {
          error.value = e.message;
        }
      }
    } finally {
      isStreaming.value = false;
      abortController.value = null;
    }
  }

  function stopStream() {
    if (abortController.value) {
      abortController.value.abort();
      abortController.value = null;
    }
    isStreaming.value = false;
  }

  function startStatusCheck() {
    checkGatewayStatus();
    statusCheckInterval = setInterval(() => {
      checkGatewayStatus();
    }, 5000);
  }

  function stopStatusCheck() {
    if (statusCheckInterval !== null) {
      clearInterval(statusCheckInterval);
      statusCheckInterval = null;
    }
  }

  onMounted(() => {
    startStatusCheck();
  });

  onUnmounted(() => {
    stopStatusCheck();
    if (unlistenRef.value) {
      unlistenRef.value();
      unlistenRef.value = null;
    }
  });

  return {
    isStreaming,
    error,
    gatewayStatus,
    sendMessage,
    stopStream,
    checkGatewayStatus,
    connect,
    retryConnect,
  };
}
