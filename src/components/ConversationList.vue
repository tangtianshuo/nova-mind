<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useConversationStore } from '@/stores/conversationStore';
import type { Conversation } from '@/types';

const emit = defineEmits<{
  select: [conversation: Conversation];
}>();

const conversationStore = useConversationStore();
const showNewDialog = ref(false);
const newTitle = ref('');

onMounted(() => {
  conversationStore.fetchConversations();
});

async function handleNewConversation() {
  if (!newTitle.value.trim()) return;

  const id = await conversationStore.createConversation(null, newTitle.value.trim());
  if (id) {
    const conv = conversationStore.conversations.find(c => c.id === id);
    if (conv) {
      emit('select', conv);
    }
  }
  showNewDialog.value = false;
  newTitle.value = '';
}

async function handleDeleteConversation(id: number, event: Event) {
  event.stopPropagation();
  if (confirm('确定要删除这个会话吗？')) {
    await conversationStore.deleteConversation(id);
  }
}

function formatDate(dateStr: string) {
  const date = new Date(dateStr);
  const now = new Date();
  const diff = now.getTime() - date.getTime();
  const days = Math.floor(diff / (1000 * 60 * 60 * 24));

  if (days === 0) {
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  } else if (days === 1) {
    return '昨天';
  } else if (days < 7) {
    return `${days}天前`;
  } else {
    return date.toLocaleDateString();
  }
}
</script>

<template>
  <div class="flex h-full flex-col">
    <div class="mb-4 flex items-center justify-between">
      <h3 class="text-sm font-medium text-gray-400">
        会话列表
      </h3>
      <button
        class="rounded bg-primary-600 px-2 py-1 text-xs text-white hover:bg-primary-700"
        @click="showNewDialog = true"
      >
        新建
      </button>
    </div>

    <div
      v-if="conversationStore.isLoading"
      class="flex flex-1 items-center justify-center"
    >
      <div class="h-6 w-6 animate-spin rounded-full border-2 border-primary-600 border-t-transparent" />
    </div>

    <div
      v-else-if="conversationStore.conversations.length === 0"
      class="flex flex-1 items-center justify-center"
    >
      <p class="text-sm text-gray-500">
        暂无会话
      </p>
    </div>

    <div
      v-else
      class="flex-1 overflow-auto space-y-1"
    >
      <div
        v-for="conv in conversationStore.conversations"
        :key="conv.id"
        :class="[
          'group relative cursor-pointer rounded-lg p-3 transition-colors',
          conversationStore.currentConversation?.id === conv.id
            ? 'bg-primary-600/20'
            : 'hover:bg-gray-700'
        ]"
        @click="emit('select', conv); conversationStore.selectConversation(conv)"
      >
        <div class="flex items-start justify-between">
          <div class="flex-1 min-w-0">
            <h4 class="truncate text-sm font-medium text-white">
              {{ conv.title }}
            </h4>
            <p class="mt-1 text-xs text-gray-500">
              {{ formatDate(conv.updated_at) }}
            </p>
          </div>
          <button
            class="opacity-0 group-hover:opacity-100 text-gray-400 hover:text-red-400"
            @click="handleDeleteConversation(conv.id!, $event)"
          >
            ✕
          </button>
        </div>
      </div>
    </div>

    <div
      v-if="showNewDialog"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
    >
      <div class="w-full max-w-sm rounded-lg bg-gray-800 p-4">
        <h3 class="mb-4 text-lg font-medium">
          新建会话
        </h3>
        <input
          v-model="newTitle"
          type="text"
          placeholder="会话标题"
          class="mb-4 w-full rounded-lg border border-gray-600 bg-gray-700 px-4 py-2 text-white placeholder-gray-400 focus:border-primary-500 focus:outline-none"
          @keyup.enter="handleNewConversation"
        >
        <div class="flex justify-end gap-2">
          <button
            class="rounded-lg border border-gray-600 px-4 py-2 text-gray-300 hover:bg-gray-700"
            @click="showNewDialog = false"
          >
            取消
          </button>
          <button
            class="rounded-lg bg-primary-600 px-4 py-2 text-white hover:bg-primary-700"
            @click="handleNewConversation"
          >
            创建
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
