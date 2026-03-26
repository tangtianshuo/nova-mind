<script setup lang="ts">
import { useThemeStore, type Theme } from '@/stores/themeStore';
import { Sun, Moon, Monitor } from 'lucide-vue-next';

const themeStore = useThemeStore();

const themes: { value: Theme; icon: typeof Sun; label: string }[] = [
  { value: 'light', icon: Sun, label: '浅色模式' },
  { value: 'dark', icon: Moon, label: '深色模式' },
  { value: 'auto', icon: Monitor, label: '跟随系统' },
];
</script>

<template>
  <div class="relative">
    <div class="flex items-center rounded-lg bg-slate-100 dark:bg-slate-700 p-0.5">
      <button
        v-for="theme in themes"
        :key="theme.value"
        @click="themeStore.setTheme(theme.value)"
        :title="theme.label"
        :aria-label="theme.label"
        :class="[
          'relative flex h-7 w-7 items-center justify-center rounded-md text-sm transition-all duration-200',
          themeStore.theme === theme.value
            ? 'bg-white dark:bg-slate-600 text-primary-500 shadow-sm'
            : 'text-slate-500 dark:text-slate-400 hover:text-slate-700 dark:hover:text-slate-200'
        ]"
      >
        <component :is="theme.icon" class="h-4 w-4" />
      </button>
    </div>
  </div>
</template>
