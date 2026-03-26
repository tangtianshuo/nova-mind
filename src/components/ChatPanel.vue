<script setup lang="ts">
import { ref, watch, nextTick } from 'vue';
import { useConversationStore } from '@/stores/conversationStore';
import MarkdownRenderer from './MarkdownRenderer.vue';

const props = defineProps<{ conversationId?: number }>();

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
    await conversationStore.sendMessage(props.conversationId!, '# AI 助手\n\n这是一个**科技感**界面示例', 'assistant');
    isStreaming.value = false;
  }, 1000);
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
  <div class="flex flex-col h-full">
    <div
      v-if="!props.conversationId"
      class="flex flex-1 items-center justify-center"
    >
      <div class="text-center">
        <p class="text-gray-500">
          选择一个会话开始对话
        </p>
      </div>
    </div>

    <template v-else>
      <div class="flex items-center justify-between pb-4 mb-4 border-b border-gray-800">
        <h3 class="text-lg font-medium text-white">
          {{ conversationStore.currentConversation?.title || '新对话' }}
        </h3>
      </div>

      <div
        ref="messagesContainer"
        class="flex-1 overflow-y-auto space-y-4"
      >
        <div
          v-for="msg in conversationStore.sortedMessages"
          :key="msg.id"
          :class="['flex', msg.role === 'user' ? 'justify-end' : 'justify-start']"
        >
          <div
            :class="[
              'max-w-[70%] rounded-2xl px-5 py-3 relative',
              msg.role === 'user'
                ? 'bg-gradient-to-br from-primary-500 to-primary-600 text-white'
                : 'bg-gray-800 text-gray-200 border border-gray-700'
            ]"
            :style="msg.role === 'user' ? 'box-shadow: 0 0 20px rgba(14, 165, 233, 0.3)' : ''"
          >
            <MarkdownRenderer
              v-if="msg.role === 'assistant'"
              :content="msg.content"
              class="text-inherit"
            />
            <p
              v-else
              class="whitespace-pre-wrap"
            >
              {{ msg.content }}
            </p>
            <p class="mt-2 text-xs opacity-60">
              {{ formatTime(msg.created_at) }}
            </p>
            <span
              v-if="msg.role === 'user'"
              class="absolute inset-0 rounded-2xl animate-glow opacity-30 pointer-events-none"
            />
          </div>
        </div>

        <div
          v-if="isStreaming"
          class="flex justify-start"
        >
          <div class="max-w-[70%] rounded-2xl bg-gray-800 px-5 py-3 border border-gray-700">
            <div class="flex gap-1">
              <span
                class="w-2 h-2 rounded-full bg-primary-400 animate-bounce"
                style="animation-delay: 0ms; background: #38BDF8; box-shadow: 0 0 10px rgba(14, 165, 233, 0.5)"
              />
              <span
                class="w-2 h-2 rounded-full animate-bounce bg-primary-400"
                style="animation-delay: 150ms; background: #38BDF8; box-shadow: 0 0 10px rgba(14, 165, 233, 0.5)"
              />
              <span
                class="w-2 h-2 rounded-full animate-bounce bg-primary-400"
                style="animation-delay: 300ms; background: #38BDF8; box-shadow: 0 0 10px rgba(14, 165, 233, 0.5)"
              />
            </div>
          </div>
        </div>
      </div>

      <div class="flex gap-3 mt-4">
        <textarea
          v-model="inputContent"
          rows="3"
          placeholder="输入消息..."
          class="input flex-1 resize-none"
          @keydown="handleKeydown"
        />
        <button
          :disabled="!inputContent.trim() || isStreaming"
          class="btn btn-primary self-end"
          @click="handleSend"
        >
          发送
        </button>
      </div>
    </template>
  </div>
</template>
