<script setup lang="ts">
import { ref } from 'vue';
import ConversationList from '@/components/ConversationList.vue';
import ChatPanel from '@/components/ChatPanel.vue';
import ExportDialog from '@/components/ExportDialog.vue';
import type { Conversation } from '@/types';
import { Download } from 'lucide-vue-next';

const selectedConversation = ref<Conversation | null>(null);
const showExportDialog = ref(false);

function handleSelectConversation(conv: Conversation) {
  selectedConversation.value = conv;
}

function handleOpenExport() {
  if (selectedConversation.value?.id) {
    showExportDialog.value = true;
  }
}

function handleExportSuccess(filePath: string) {
  console.log('导出成功:', filePath);
}

function handleExportError(error: string) {
  console.error('导出失败:', error);
}
</script>

<template>
  <div class="flex h-full relative">
    <div class="w-64 border-r border-gray-700 dark:border-slate-700 p-4 flex-shrink-0">
      <ConversationList @select="handleSelectConversation" />
    </div>
    <div class="flex-1 p-6 flex flex-col">
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-xl font-semibold text-slate-800 dark:text-slate-100">
          AI 对话
        </h2>
        <button
          v-if="selectedConversation?.id"
          class="inline-flex items-center gap-2 px-4 py-2 rounded-lg bg-slate-700 hover:bg-slate-600 dark:bg-slate-800 dark:hover:bg-slate-700 text-sm text-slate-200 transition-colors"
          @click="handleOpenExport"
        >
          <Download class="w-4 h-4" />
          导出对话
        </button>
      </div>
      <ChatPanel :conversation-id="selectedConversation?.id" />
    </div>

    <ExportDialog
      v-if="showExportDialog && selectedConversation"
      :conversation-id="selectedConversation.id!"
      :title="selectedConversation.title"
      @close="showExportDialog = false"
      @success="handleExportSuccess"
      @error="handleExportError"
    />
  </div>
</template>
