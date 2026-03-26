import { defineStore } from 'pinia';
import { ref, watch } from 'vue';

export type Theme = 'light' | 'dark' | 'auto';

const STORAGE_KEY = 'nova-mind-theme';

function loadStoredTheme(): Theme {
  const stored = localStorage.getItem(STORAGE_KEY);
  if (stored === 'light' || stored === 'dark' || stored === 'auto') {
    return stored;
  }
  return 'dark';
}

function getSystemTheme(): 'light' | 'dark' {
  if (typeof window !== 'undefined' && window.matchMedia) {
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
  }
  return 'light';
}

function getEffectiveTheme(theme: Theme): 'light' | 'dark' {
  if (theme === 'auto') {
    return getSystemTheme();
  }
  return theme;
}

function applyTheme(theme: 'light' | 'dark') {
  const html = document.documentElement;

  html.classList.remove('light', 'dark');

  if (theme === 'dark') {
    html.classList.add('dark');
  }
}

export const useThemeStore = defineStore('theme', () => {
  const storedTheme = ref<Theme>(loadStoredTheme());
  const effectiveTheme = ref<'light' | 'dark'>(getEffectiveTheme(storedTheme.value));

  function updateEffectiveTheme() {
    const newEffective = getEffectiveTheme(storedTheme.value);
    if (effectiveTheme.value !== newEffective) {
      effectiveTheme.value = newEffective;
      applyTheme(newEffective);
    }
    localStorage.setItem(STORAGE_KEY, storedTheme.value);
  }

  function setTheme(theme: Theme) {
    storedTheme.value = theme;
    updateEffectiveTheme();
    dispatchThemeChangeEvent(theme);
  }

  function toggleTheme() {
    const newTheme = effectiveTheme.value === 'dark' ? 'light' : 'dark';
    setTheme(newTheme);
  }

  function dispatchThemeChangeEvent(theme: Theme) {
    if (typeof window !== 'undefined') {
      window.dispatchEvent(new CustomEvent('themechange', { detail: { theme } }));
    }
  }

  function initTheme() {
    updateEffectiveTheme();
    applyTheme(effectiveTheme.value);

    if (typeof window !== 'undefined' && window.matchMedia) {
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');

      const handler = (e: MediaQueryListEvent) => {
        if (storedTheme.value === 'auto') {
          effectiveTheme.value = e.matches ? 'dark' : 'light';
          applyTheme(effectiveTheme.value);
        }
      };

      mediaQuery.addEventListener('change', handler);
    }
  }

  watch(storedTheme, updateEffectiveTheme);

  return {
    theme: storedTheme,
    effectiveTheme,
    setTheme,
    toggleTheme,
    initTheme,
  };
});
