<script setup lang="ts">
import { ref, computed } from 'vue';
import ConversationList from '@/components/ConversationList.vue';
import ChatPanel from '@/components/ChatPanel.vue';
import type { Conversation } from '@/types';

const selectedConversation = ref<Conversation | null>(null);

function handleSelectConversation(conv: Conversation) {
  selectedConversation.value = conv;
}
</script>

<template>
  <div class="flex h-full">
    <div class="w-64 border-r border-gray-700 p-4">
      <ConversationList @select="handleSelectConversation" />
    </div>
    <div class="flex-1 p-6">
      <ChatPanel :conversation-id="selectedConversation?.id" />
    </div>
  </div>
</template>
