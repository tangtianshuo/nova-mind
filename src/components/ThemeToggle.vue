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
    <div class="flex items-center rounded-xl bg-slate-100 dark:bg-slate-800 p-1 border border-slate-200 dark:border-slate-700">
      <button
        v-for="theme in themes"
        :key="theme.value"
        :title="theme.label"
        :aria-label="theme.label"
        :class="[
          'relative flex h-8 w-8 items-center justify-center rounded-lg text-sm',
          'transition-all duration-200',
          themeStore.theme === theme.value
            ? 'bg-gradient-to-br from-primary-500 to-primary-600 text-white shadow-lg shadow-primary-500/25'
            : 'text-slate-500 dark:text-slate-400 hover:text-slate-700 dark:hover:text-slate-200'
        ]"
        @click="themeStore.setTheme(theme.value)"
      >
        <component
          :is="theme.icon"
          class="h-4 w-4"
        />
        
        <span
          v-if="themeStore.theme === theme.value"
          class="absolute inset-0 rounded-lg animate-pulse bg-primary-400/20"
        />
      </button>
    </div>
  </div>
</template>
