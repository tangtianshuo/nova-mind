import { ref, watch } from 'vue';
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

function saveSettings(settings: Settings) {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
}

export function useSettings() {
  const settings = ref<Settings>(loadSettings());

  watch(settings, (newSettings) => {
    saveSettings(newSettings);
  }, { deep: true });

  function updateSetting<K extends keyof Settings>(key: K, value: Settings[K]) {
    settings.value[key] = value;
  }

  function resetSettings() {
    settings.value = { ...defaultSettings };
  }

  function setApiKey(key: string) {
    settings.value.apiKey = key;
  }

  function getApiKeyMasked(): string {
    if (!settings.value.apiKey) return '';
    const key = settings.value.apiKey;
    if (key.length <= 8) return '*'.repeat(key.length);
    return key.slice(0, 4) + '****' + key.slice(-4);
  }

  return {
    settings,
    updateSetting,
    resetSettings,
    setApiKey,
    getApiKeyMasked
  };
}
