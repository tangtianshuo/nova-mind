<script setup lang="ts">
import { ref, watch, nextTick } from 'vue';
import { useConversationStore } from '@/stores/conversationStore';

const props = defineProps<{
  conversationId?: number;
}>();

const conversationStore = useConversationStore();
const inputContent = ref('');
const messagesContainer = ref<HTMLElement | null>(null);
const isStreaming = ref(false);

watch(() => props.conversationId, async (newId) => {
  if (newId) {
    await conversationStore.fetchConversation(newId);
    scrollToBottom();
  }
}, { immediate: true });

watch(() => conversationStore.messages, () => {
  nextTick(() => scrollToBottom());
}, { deep: true });

function scrollToBottom() {
  if (messagesContainer.value) {
    messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
  }
}

async function handleSend() {
  if (!inputContent.value.trim() || !props.conversationId || isStreaming.value) return;

  const content = inputContent.value.trim();
  inputContent.value = '';

  const userMsg = await conversationStore.sendMessage(props.conversationId, content, 'user');
  if (!userMsg) return;

  isStreaming.value = true;

  setTimeout(async () => {
    await conversationStore.sendMessage(
      props.conversationId!,
      '这是 AI 助手的回复内容。',
      'assistant'
    );
    isStreaming.value = false;
  }, 100);
}

function handleKeydown(event: KeyboardEvent) {
  if (event.key === 'Enter' && !event.shiftKey) {
    event.preventDefault();
    handleSend();
  }
}

function formatTime(dateStr: string) {
  return new Date(dateStr).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
}
</script>

<template>
  <div class="flex h-full flex-col">
    <div
      v-if="!conversationId"
      class="flex flex-1 items-center justify-center"
    >
      <div class="text-center">
        <p class="text-gray-500">
          请选择一个会话或新建会话
        </p>
      </div>
    </div>

    <template v-else>
      <div class="mb-4 flex items-center justify-between border-b border-gray-700 pb-4">
        <h3 class="text-lg font-medium">
          {{ conversationStore.currentConversation?.title || '新会话' }}
        </h3>
      </div>

      <div
        ref="messagesContainer"
        class="flex-1 space-y-4 overflow-auto"
      >
        <div
          v-for="msg in conversationStore.sortedMessages"
          :key="msg.id"
          :class="[
            'flex',
            msg.role === 'user' ? 'justify-end' : 'justify-start'
          ]"
        >
          <div
            :class="[
              'max-w-[70%] rounded-lg p-3',
              msg.role === 'user'
                ? 'bg-primary-600 text-white'
                : 'bg-gray-700 text-gray-100'
            ]"
          >
            <p class="whitespace-pre-wrap">
              {{ msg.content }}
            </p>
            <p class="mt-1 text-xs opacity-60">
              {{ formatTime(msg.created_at) }}
            </p>
          </div>
        </div>

        <div
          v-if="isStreaming"
          class="flex justify-start"
        >
          <div class="max-w-[70%] rounded-lg bg-gray-700 p-3">
            <div class="flex gap-1">
              <div
                class="h-2 w-2 animate-bounce rounded-full bg-gray-400"
                style="animation-delay: 0ms"
              />
              <div
                class="h-2 w-2 animate-bounce rounded-full bg-gray-400"
                style="animation-delay: 150ms"
              />
              <div
                class="h-2 w-2 animate-bounce rounded-full bg-gray-400"
                style="animation-delay: 300ms"
              />
            </div>
          </div>
        </div>
      </div>

      <div class="mt-4 flex gap-2">
        <textarea
          v-model="inputContent"
          rows="2"
          placeholder="输入消息..."
          class="flex-1 resize-none rounded-lg border border-gray-600 bg-gray-700 px-4 py-2 text-white placeholder-gray-400 focus:border-primary-500 focus:outline-none"
          @keydown="handleKeydown"
        />
        <button
          :disabled="!inputContent.trim() || isStreaming"
          class="rounded-lg bg-primary-600 px-4 py-2 text-white hover:bg-primary-700 disabled:opacity-50"
          @click="handleSend"
        >
          发送
        </button>
      </div>
    </template>
  </div>
</template>
