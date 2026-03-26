import { defineStore } from 'pinia';
import { ref, computed } from 'vue';

export interface Shortcut {
  action: string;
  key: string;
  modifiers: ('ctrl' | 'alt' | 'shift' | 'meta')[];
  description: string;
  enabled: boolean;
}

const STORAGE_KEY = 'nova-mind-shortcuts';

const defaultShortcuts: Shortcut[] = [
  { action: 'send', key: 'Enter', modifiers: ['ctrl'], description: '发送消息', enabled: true },
  { action: 'newSession', key: 'n', modifiers: ['ctrl'], description: '新建会话', enabled: true },
  { action: 'save', key: 's', modifiers: ['ctrl'], description: '保存草稿', enabled: true },
  { action: 'export', key: 'e', modifiers: ['ctrl'], description: '导出文档', enabled: true },
  { action: 'toggleTheme', key: 't', modifiers: ['ctrl', 'shift'], description: '切换主题', enabled: true },
  { action: 'openSettings', key: ',', modifiers: ['ctrl'], description: '打开设置', enabled: true },
  { action: 'search', key: 'k', modifiers: ['ctrl'], description: '搜索', enabled: true },
  { action: 'closeModal', key: 'Escape', modifiers: [], description: '关闭弹窗', enabled: true },
];

function loadShortcuts(): Shortcut[] {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    return stored ? JSON.parse(stored) : [...defaultShortcuts];
  } catch {
    return [...defaultShortcuts];
  }
}

export const useShortcutStore = defineStore('shortcuts', () => {
  const shortcuts = ref<Shortcut[]>(loadShortcuts());

  const activeShortcuts = computed(() => shortcuts.value.filter(s => s.enabled));

  function saveShortcuts() {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(shortcuts.value));
  }

  function formatShortcut(shortcut: Shortcut): string {
    const parts: string[] = [];
    if (shortcut.modifiers.includes('ctrl')) parts.push('Ctrl');
    if (shortcut.modifiers.includes('alt')) parts.push('Alt');
    if (shortcut.modifiers.includes('shift')) parts.push('Shift');
    if (shortcut.modifiers.includes('meta')) parts.push('⌘');
    parts.push(shortcut.key === ' ' ? 'Space' : shortcut.key);
    return parts.join(' + ');
  }

  function hasConflict(key: string, modifiers: string[], excludeAction?: string): boolean {
    const keyCombo = [...modifiers, key].map(k => k.toLowerCase()).sort().join('+');
    return shortcuts.value.some(s => {
      if (excludeAction && s.action === excludeAction) return false;
      const existing = [...s.modifiers, s.key].map(k => k.toLowerCase()).sort().join('+');
      return keyCombo === existing;
    });
  }

  function updateShortcut(action: string, newKey: string, newModifiers: Shortcut['modifiers']) {
    const shortcut = shortcuts.value.find(s => s.action === action);
    if (shortcut) {
      shortcut.key = newKey;
      shortcut.modifiers = newModifiers;
      saveShortcuts();
    }
  }

  function toggleShortcut(action: string, enabled: boolean) {
    const shortcut = shortcuts.value.find(s => s.action === action);
    if (shortcut) {
      shortcut.enabled = enabled;
      saveShortcuts();
    }
  }

  function resetShortcuts() {
    shortcuts.value = [...defaultShortcuts];
    saveShortcuts();
  }

  function getShortcutByAction(action: string): Shortcut | undefined {
    return shortcuts.value.find(s => s.action === action);
  }

  return {
    shortcuts,
    activeShortcuts,
    formatShortcut,
    hasConflict,
    updateShortcut,
    toggleShortcut,
    resetShortcuts,
    getShortcutByAction,
  };
});
