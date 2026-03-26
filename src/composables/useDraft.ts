import { ref, watch, onMounted, onUnmounted } from 'vue';
import { useDraftStore } from '@/stores/draftStore';

export interface UseDraftOptions {
  conversationId: number;
  autoSaveInterval?: number;
  debounceDelay?: number;
}

export function useDraft(options: UseDraftOptions) {
  const draftStore = useDraftStore();
  const { conversationId, autoSaveInterval = 30000, debounceDelay = 1000 } = options;

  const localDraft = ref('');
  const isDirty = ref(false);
  let debounceTimer: number | null = null;
  let autoSaveTimer: number | null = null;

  function loadDraft() {
    localDraft.value = draftStore.getDraft(conversationId);
    isDirty.value = false;
  }

  function saveDraft() {
    if (localDraft.value !== draftStore.getDraft(conversationId)) {
      draftStore.saveDraft(conversationId, localDraft.value);
      isDirty.value = false;
    }
  }

  function clearDraft() {
    localDraft.value = '';
    draftStore.clearDraft(conversationId);
    isDirty.value = false;
  }

  function updateDraft(content: string) {
    localDraft.value = content;
    isDirty.value = true;

    if (debounceTimer !== null) {
      clearTimeout(debounceTimer);
    }

    debounceTimer = window.setTimeout(() => {
      saveDraft();
    }, debounceDelay);
  }

  function startAutoSave() {
    stopAutoSave();

    autoSaveTimer = window.setInterval(() => {
      if (isDirty.value) {
        saveDraft();
      }
    }, autoSaveInterval);
  }

  function stopAutoSave() {
    if (autoSaveTimer !== null) {
      clearInterval(autoSaveTimer);
      autoSaveTimer = null;
    }
  }

  function persistAndClear() {
    if (debounceTimer !== null) {
      clearTimeout(debounceTimer);
      debounceTimer = null;
    }
    saveDraft();
  }

  watch(() => conversationId, (newId) => {
    if (newId !== undefined) {
      loadDraft();
    }
  }, { immediate: true });

  onMounted(() => {
    loadDraft();
    startAutoSave();
  });

  onUnmounted(() => {
    stopAutoSave();
    if (debounceTimer !== null) {
      clearTimeout(debounceTimer);
    }
  });

  return {
    draft: localDraft,
    isDirty,
    updateDraft,
    saveDraft,
    clearDraft,
    loadDraft,
    startAutoSave,
    stopAutoSave,
    persistAndClear,
  };
}
