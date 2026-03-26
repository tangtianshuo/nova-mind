import { onMounted, onUnmounted, ref } from 'vue';
import { ref } from 'vue';



export interface Shortcut {
  key: string;
  modifiers: ('ctrl' | 'alt' | 'shift' | 'meta')[];
  action: string;
  description: string;
}

const defaultShortcuts: Shortcut[] = [
  { key: 'Enter', modifiers: ['ctrl'], action: 'send', description: '发送消息' },
  { key: 'n', modifiers: ['ctrl'], action: 'newSession', description: '新建会话' },
  { key: 's', modifiers: ['ctrl'], action: 'save', description: '保存草稿' },
  { key: 'e', modifiers: ['ctrl'], action: 'export', description: '导出文档' },
  { key: 't', modifiers: ['ctrl', 'shift'], action: 'toggleTheme', description: '切换主题' },
  { key: ',', modifiers: ['ctrl'], action: 'openSettings', description: '打开设置' },
  { key: 'k', modifiers: ['ctrl'], action: 'search', description: '搜索' },
  { key: 'Escape', modifiers: [], action: 'closeModal', description: '关闭弹窗' },
];

export function useKeyboard(shortcuts: Shortcut[] = defaultShortcuts) {
  const activeShortcuts = ref<Shortcut[]>(shortcuts);
  const conflictKey = ref<string | null>(null);

  function handleKeyDown(event: KeyboardEvent) {
    const pressed = `${event.ctrlKey ? 'ctrl+' : ''}${event.altKey ? 'alt+' : ''}${event.shiftKey ? 'shift+' : ''}${event.key}`;

    for (const shortcut of activeShortcuts.value) {
      const shortcutKey = [
        ...shortcut.modifiers.map(m => m.toLowerCase()),
        shortcut.key.toLowerCase()
      ].join('+');

      if (pressed.toLowerCase() === shortcutKey) {
        event.preventDefault();
        return shortcut.action;
      }
    }

    return null;
  }

  function registerShortcut(shortcut: Shortcut) {
    const exists = activeShortcuts.value.some(
      s => s.action === shortcut.action
    );
    if (!exists) {
      activeShortcuts.value.push(shortcut);
    }
  }

  function unregisterShortcut(action: string) {
    activeShortcuts.value = activeShortcuts.value.filter(s => s.action !== action);
  }

  function updateShortcut(action: string, newKey: string, newModifiers: Shortcut['modifiers']) {
    const shortcut = activeShortcuts.value.find(s => s.action === action);
    if (shortcut) {
      shortcut.key = newKey;
      shortcut.modifiers = newModifiers;
    }
  }

  function hasConflict(key: string, modifiers: Shortcut['modifiers']): boolean {
    const keyCombo = [...modifiers, key].map(k => k.toLowerCase()).sort().join('+');
    return activeShortcuts.value.some(s => {
      const existing = [...s.modifiers, s.key].map(k => k.toLowerCase()).sort().join('+');
      return keyCombo === existing;
    });
  }

  function formatShortcut(shortcut: Shortcut): string {
    const mods = shortcut.modifiers.map(m => {
      const map: Record<string, string> = {
        ctrl: 'Ctrl',
        alt: 'Alt',
        shift: 'Shift',
        meta: 'Cmd'
      };
      return map[m] || m;
    });
    return [...mods, shortcut.key].join(' + ');
  }

  return {
    activeShortcuts,
    handleKeyDown,
    registerShortcut,
    unregisterShortcut,
    updateShortcut,
    hasConflict,
    formatShortcut,
    conflictKey,
    defaultShortcuts
  };
}
