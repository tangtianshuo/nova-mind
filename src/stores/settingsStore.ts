import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { Settings } from '@/types';

const STORAGE_KEY = 'nova-mind-settings';

const defaultSettings: Settings = {
  theme: 'dark',
  autoSave: true,
  saveInterval: 30000,
  apiKey: '',
  sandboxPath: '',
  shortcuts: {},
  editorSettings: {
    wordWrap: true,
    lineNumbers: true,
    minimap: true,
  },
  fontSize: 14,
};

function loadSettings(): Settings {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    return stored ? { ...defaultSettings, ...JSON.parse(stored) } : { ...defaultSettings };
  } catch {
    return { ...defaultSettings };
  }
}

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<Settings>(loadSettings());
  const isLoading = ref(false);
  const error = ref<string | null>(null);

  function saveSettings() {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(settings.value));
  }

  function updateSetting<K extends keyof Settings>(key: K, value: Settings[K]) {
    settings.value[key] = value;
    saveSettings();
  }

  function resetSettings() {
    settings.value = { ...defaultSettings };
    saveSettings();
  }

  function setApiKey(key: string) {
    settings.value.apiKey = key;
    saveSettings();
  }

  function getApiKeyMasked(): string {
    if (!settings.value.apiKey) return '';
    const key = settings.value.apiKey;
    if (key.length <= 8) return '*'.repeat(key.length);
    return key.slice(0, 4) + '****' + key.slice(-4);
  }

  async function exportSettings(): Promise<string> {
    const data = JSON.stringify(settings.value, null, 2);
    const blob = new Blob([data], { type: 'application/json' });
    return URL.createObjectURL(blob);
  }

  async function importSettings(file: File): Promise<boolean> {
    try {
      const text = await file.text();
      const data = JSON.parse(text);
      settings.value = { ...defaultSettings, ...data };
      saveSettings();
      return true;
    } catch {
      error.value = '导入设置失败';
      return false;
    }
  }

  function clearCache() {
    localStorage.removeItem(STORAGE_KEY);
    settings.value = { ...defaultSettings };
  }

  return {
    settings,
    isLoading,
    error,
    updateSetting,
    resetSettings,
    setApiKey,
    getApiKeyMasked,
    exportSettings,
    importSettings,
    clearCache,
  };
});
