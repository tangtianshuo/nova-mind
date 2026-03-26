import { defineStore } from 'pinia';
import { ref, watch } from 'vue';
import { useStorage } from '@vueuse/core';

export interface Draft {
  conversationId: number;
  content: string;
  updatedAt: string;
}

const DRAFT_KEY = 'nova-mind-drafts';
const AUTO_SAVE_INTERVAL = 30000;

export const useDraftStore = defineStore('draft', () => {
  const drafts = useStorage<Record<number, Draft>>(DRAFT_KEY, {});
  const currentConversationId = ref<number | null>(null);
  const currentDraft = ref('');
  const autoSaveTimer = ref<number | null>(null);

  function getDraft(conversationId: number): string {
    const draft = drafts.value[conversationId];
    return draft?.content || '';
  }

  function saveDraft(conversationId: number, content: string) {
    if (!content.trim()) {
      deleteDraft(conversationId);
      return;
    }

    drafts.value = {
      ...drafts.value,
      [conversationId]: {
        conversationId,
        content,
        updatedAt: new Date().toISOString(),
      },
    };
  }

  function deleteDraft(conversationId: number) {
    const newDrafts = { ...drafts.value };
    delete newDrafts[conversationId];
    drafts.value = newDrafts;
  }

  function clearDraft(conversationId: number) {
    deleteDraft(conversationId);
    if (currentConversationId.value === conversationId) {
      currentDraft.value = '';
    }
  }

  function setCurrentConversation(conversationId: number | null) {
    if (currentConversationId.value !== null && currentConversationId.value !== conversationId) {
      if (currentDraft.value) {
        saveDraft(currentConversationId.value, currentDraft.value);
      }
    }

    currentConversationId.value = conversationId;

    if (conversationId !== null) {
      currentDraft.value = getDraft(conversationId);
    } else {
      currentDraft.value = '';
    }
  }

  function updateCurrentDraft(content: string) {
    currentDraft.value = content;
  }

  function startAutoSave() {
    stopAutoSave();

    autoSaveTimer.value = window.setInterval(() => {
      if (currentConversationId.value !== null && currentDraft.value) {
        saveDraft(currentConversationId.value, currentDraft.value);
      }
    }, AUTO_SAVE_INTERVAL);
  }

  function stopAutoSave() {
    if (autoSaveTimer.value !== null) {
      clearInterval(autoSaveTimer.value);
      autoSaveTimer.value = null;
    }
  }

  function persistCurrentDraft() {
    if (currentConversationId.value !== null && currentDraft.value) {
      saveDraft(currentConversationId.value, currentDraft.value);
    }
  }

  watch(currentConversationId, (newId) => {
    if (newId !== null) {
      startAutoSave();
    } else {
      stopAutoSave();
    }
  });

  return {
    drafts,
    currentDraft,
    currentConversationId,
    getDraft,
    saveDraft,
    deleteDraft,
    clearDraft,
    setCurrentConversation,
    updateCurrentDraft,
    startAutoSave,
    stopAutoSave,
    persistCurrentDraft,
  };
});
