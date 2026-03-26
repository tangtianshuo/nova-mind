<script setup lang="ts">
import { ref, watch, nextTick, onMounted, computed, onUnmounted } from 'vue';
import { useConversationStore } from '@/stores/conversationStore';
import { useDraftStore } from '@/stores/draftStore';
import { useAIChat } from '@/composables/useAIChat';
import MarkdownRenderer from './MarkdownRenderer.vue';
import { Send, Square, Loader2, Wifi, WifiOff, RefreshCw } from 'lucide-vue-next';

const props = defineProps<{ conversationId?: number }>();

const conversationStore = useConversationStore();
const draftStore = useDraftStore();
const { isStreaming, error, sendMessage, stopStream, gatewayStatus, retryConnect } = useAIChat();

const isGatewayConnected = computed(() => gatewayStatus.value?.connected ?? false);

const inputContent = ref('');
const messagesContainer = ref<HTMLElement | null>(null);
const streamingContent = ref('');
const streamingMessageId = ref<number | null>(null);
const inputRef = ref<HTMLTextAreaElement | null>(null);

watch(() => props.conversationId, async (newId) => {
  if (newId) {
    await conversationStore.fetchConversation(newId);
    draftStore.setCurrentConversation(newId);
    const draft = draftStore.getDraft(newId);
    if (draft) {
      inputContent.value = draft;
    }
    scrollToBottom();
  } else {
    draftStore.setCurrentConversation(null);
  }
}, { immediate: true });

watch(() => conversationStore.messages, () => {
  nextTick(() => scrollToBottom());
}, { deep: true });

watch(isStreaming, (streaming) => {
  if (!streaming && streamingContent.value) {
    streamingContent.value = '';
    streamingMessageId.value = null;
  }
});

const canSend = computed(() => {
  return inputContent.value.trim() && props.conversationId && !isStreaming.value;
});

function scrollToBottom() {
  if (messagesContainer.value) {
    messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
  }
}

async function handleSend() {
  if (!inputContent.value.trim() || !props.conversationId || isStreaming.value) return;

  const content = inputContent.value.trim();
  inputContent.value = '';
  draftStore.clearDraft(props.conversationId);

  const userMsg = await conversationStore.sendMessage(props.conversationId, content, 'user');
  if (!userMsg) return;

  streamingContent.value = '';
  streamingMessageId.value = userMsg.id ?? null;

  try {
    await sendMessage(props.conversationId, content);
  } catch (err) {
    console.error('发送消息失败:', err);
    if (streamingMessageId.value) {
      await conversationStore.deleteMessage(streamingMessageId.value);
    }
    streamingContent.value = '';
    streamingMessageId.value = null;
  }
}

function handleStop() {
  stopStream();
}

function handleKeydown(event: KeyboardEvent) {
  if (event.key === 'Enter' && !event.shiftKey) {
    event.preventDefault();
    handleSend();
  }
}

function handleInput() {
  autoResize();
  if (props.conversationId) {
    draftStore.saveDraft(props.conversationId, inputContent.value);
  }
}

function autoResize() {
  if (inputRef.value) {
    inputRef.value.style.height = 'auto';
    inputRef.value.style.height = Math.min(inputRef.value.scrollHeight, 150) + 'px';
  }
}

function formatTime(dateStr: string) {
  return new Date(dateStr).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
}

function insertAtCursor(text: string) {
  if (inputRef.value) {
    const start = inputRef.value.selectionStart;
    const end = inputRef.value.selectionEnd;
    inputContent.value = inputContent.value.slice(0, start) + text + inputContent.value.slice(end);
    nextTick(() => {
      inputRef.value!.selectionStart = inputRef.value!.selectionEnd = start + text.length;
      inputRef.value!.focus();
    });
  }
}

onMounted(() => {
  nextTick(() => autoResize());
  if (props.conversationId) {
    draftStore.startAutoSave();
  }
});

onUnmounted(() => {
  draftStore.persistCurrentDraft();
  draftStore.stopAutoSave();
});
</script>

<template>
  <div class="flex flex-col h-full">
    <div
      v-if="!props.conversationId"
      class="flex flex-1 items-center justify-center"
    >
      <div class="text-center">
        <div class="w-16 h-16 rounded-2xl bg-gradient-to-br from-primary-100 to-primary-50 dark:from-primary-900/30 dark:to-primary-800/20 flex items-center justify-center mx-auto mb-4">
          <Send class="w-8 h-8 text-primary-500" />
        </div>
        <p class="text-slate-500 dark:text-slate-400 mb-2">
          选择一个会话开始对话
        </p>
        <p class="text-sm text-slate-400 dark:text-slate-500">
          或在左侧创建新的会话
        </p>
      </div>
    </div>

    <template v-else>
      <div class="flex items-center justify-between pb-4 mb-4 border-b border-slate-700 dark:border-slate-700">
        <div>
          <h3 class="text-lg font-medium text-slate-800 dark:text-slate-100">
            {{ conversationStore.currentConversation?.title || '新对话' }}
          </h3>
          <p class="text-sm text-slate-500 dark:text-slate-400">
            {{ conversationStore.messages.length }} 条消息
          </p>
        </div>
        <div class="flex items-center gap-3">
          <div
            v-if="isGatewayConnected"
            class="flex items-center gap-1.5 text-green-500"
          >
            <Wifi class="w-4 h-4" />
            <span class="text-xs">Gateway 已连接</span>
          </div>
          <div
            v-else
            class="flex items-center gap-1.5 text-red-500"
          >
            <WifiOff class="w-4 h-4" />
            <span class="text-xs">Gateway 未连接</span>
            <button
              class="p-1 hover:bg-slate-700 rounded transition-colors"
              title="重连"
              @click="retryConnect"
            >
              <RefreshCw class="w-3 h-3" />
            </button>
          </div>
          <div v-if="isStreaming" class="flex items-center gap-2 text-primary-500">
            <Loader2 class="w-4 h-4 animate-spin" />
            <span class="text-sm">AI 正在思考...</span>
          </div>
        </div>
      </div>

      <div
        ref="messagesContainer"
        class="flex-1 overflow-y-auto space-y-4 pr-2"
      >
        <div
          v-if="conversationStore.sortedMessages.length === 0 && !isStreaming"
          class="flex flex-col items-center justify-center h-full"
        >
          <div class="text-center">
            <div class="w-20 h-20 rounded-2xl bg-gradient-to-br from-primary-100 to-primary-50 dark:from-primary-900/30 dark:to-primary-800/20 flex items-center justify-center mx-auto mb-4">
              <svg class="w-10 h-10 text-primary-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
              </svg>
            </div>
            <p class="text-slate-500 dark:text-slate-400 mb-2">
              开始与 AI 对话
            </p>
            <p class="text-sm text-slate-400 dark:text-slate-500">
              输入消息获取 AI 助手帮助
            </p>
          </div>
        </div>

        <div
          v-for="msg in conversationStore.sortedMessages"
          :key="msg.id"
          :class="['flex', msg.role === 'user' ? 'justify-end' : 'justify-start']"
        >
          <div
            :class="[
              'max-w-[80%] rounded-2xl px-5 py-3 relative group',
              msg.role === 'user'
                ? 'bg-gradient-to-br from-primary-500 to-primary-600 text-white shadow-lg shadow-primary-500/25'
                : 'bg-slate-800 dark:bg-slate-900 text-slate-200 border border-slate-700 dark:border-slate-700'
            ]"
          >
            <MarkdownRenderer
              v-if="msg.role === 'assistant'"
              :content="msg.content"
              class="text-inherit"
            />
            <p
              v-else
              class="whitespace-pre-wrap break-words"
            >
              {{ msg.content }}
            </p>
            <div class="mt-2 flex items-center justify-between">
              <p class="text-xs opacity-60">
                {{ formatTime(msg.created_at) }}
              </p>
              <div
                v-if="msg.role === 'user'"
                class="flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity"
              >
                <button
                  class="p-1 hover:bg-white/20 rounded transition-colors"
                  title="复制"
                  @click="insertAtCursor(msg.content)"
                >
                  <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                  </svg>
                </button>
              </div>
            </div>
          </div>
        </div>

        <div
          v-if="isStreaming && streamingContent"
          class="flex justify-start"
        >
          <div class="max-w-[80%] rounded-2xl bg-slate-800 dark:bg-slate-900 px-5 py-3 border border-slate-700 dark:border-slate-700">
            <MarkdownRenderer
              :content="streamingContent"
              class="text-slate-200"
            />
            <div class="flex items-center gap-1 mt-2">
              <span class="w-2 h-2 rounded-full bg-primary-400 animate-pulse" style="animation-duration: 1s;" />
              <span class="w-2 h-2 rounded-full bg-primary-400 animate-pulse" style="animation-duration: 1s; animation-delay: 0.2s;" />
              <span class="w-2 h-2 rounded-full bg-primary-400 animate-pulse" style="animation-duration: 1s; animation-delay: 0.4s;" />
            </div>
          </div>
        </div>

        <div
          v-if="error"
          class="flex justify-start"
        >
          <div class="max-w-[80%] rounded-2xl bg-red-900/20 border border-red-800 px-5 py-3">
            <p class="text-red-400 text-sm">
              发送消息失败: {{ error }}
            </p>
            <p
              v-if="!isGatewayConnected"
              class="text-red-400/70 text-xs mt-1"
            >
              请确保 OpenClaw Gateway 正在运行（运行命令: openclaw gateway --port 18789）
            </p>
          </div>
        </div>

        <div
          v-if="!isGatewayConnected && conversationStore.sortedMessages.length === 0"
          class="flex justify-start"
        >
          <div class="max-w-[80%] rounded-2xl bg-yellow-900/20 border border-yellow-800 px-5 py-3">
            <p class="text-yellow-400 text-sm">
              OpenClaw Gateway 未连接
            </p>
            <p class="text-yellow-400/70 text-xs mt-1">
              请先启动 OpenClaw Gateway：<code class="bg-yellow-900/30 px-1 rounded">openclaw gateway --port 18789</code>
            </p>
            <button
              class="mt-2 px-3 py-1 bg-yellow-600 hover:bg-yellow-700 text-white text-xs rounded transition-colors"
              @click="retryConnect"
            >
              尝试重连
            </button>
          </div>
        </div>
      </div>

      <div class="mt-4 flex gap-3">
        <div class="flex-1 relative">
          <textarea
            ref="inputRef"
            v-model="inputContent"
            rows="1"
            placeholder="输入消息... (Enter 发送, Shift+Enter 换行)"
            class="input w-full resize-none pr-12"
            :disabled="isStreaming"
            @keydown="handleKeydown"
            @input="handleInput"
          />
          <button
            v-if="isStreaming"
            class="absolute right-2 bottom-2 p-2 rounded-lg bg-red-500 hover:bg-red-600 text-white transition-colors"
            title="停止生成"
            @click="handleStop"
          >
            <Square class="w-4 h-4" />
          </button>
          <button
            v-else
            class="absolute right-2 bottom-2 p-2 rounded-lg bg-primary-500 hover:bg-primary-600 text-white transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            :disabled="!canSend"
            title="发送消息"
            @click="handleSend"
          >
            <Send class="w-4 h-4" />
          </button>
        </div>
      </div>
    </template>
  </div>
</template>

<style scoped>
.input {
  @apply w-full rounded-xl border border-slate-700 dark:border-slate-600 
         bg-slate-800 dark:bg-slate-900 text-slate-100 
         px-4 py-3 placeholder-slate-500
         focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent
         transition-all duration-200;
}

.input:disabled {
  @apply opacity-50 cursor-not-allowed;
}
</style>
