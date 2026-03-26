import { ref } from 'vue';
import { useConversationStore } from '@/stores/conversationStore';

export function useAIChat() {
  const conversationStore = useConversationStore();
  const isStreaming = ref(false);
  const error = ref<string | null>(null);
  const abortController = ref<AbortController | null>(null);

  async function sendMessage(conversationId: number, content: string) {
    isStreaming.value = true;
    error.value = null;

    await conversationStore.sendMessage(conversationId, content, 'user');

    abortController.value = new AbortController();

    try {
      const response = await fetch(
        `http://localhost:8080/api/chat`,
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

      // eslint-disable-next-line no-constant-condition
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

  return {
    isStreaming,
    error,
    sendMessage,
    stopStream,
  };
}
