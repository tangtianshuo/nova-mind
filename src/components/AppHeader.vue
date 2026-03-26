<script setup lang="ts">
import { ref } from 'vue';
import { Sparkles } from 'lucide-vue-next';
import ThemeToggle from './ThemeToggle.vue';
import HealthStatus from './HealthStatus.vue';

const appVersion = ref('1.0.0');
</script>

<template>
  <header
    class="flex items-center justify-between h-16 px-6
                 bg-white/80 dark:bg-slate-900/80
                 border-b border-slate-200 dark:border-slate-800
                 backdrop-blur-xl"
  >
    <div class="flex items-center gap-4">
      <h2 class="text-lg font-semibold text-slate-700 dark:text-slate-200">
        AI 产品管理
      </h2>
      <span class="text-xs text-slate-400 dark:text-slate-500">v{{ appVersion }}</span>
    </div>

    <div class="flex items-center gap-3">
      <ThemeToggle />
      <HealthStatus />
      
      <button
        class="inline-flex items-center gap-2 px-4 py-2 rounded-xl
                     bg-gradient-to-r from-primary-500 to-primary-600
                     text-sm font-medium text-white
                     shadow-lg shadow-primary-500/25
                     transition-all duration-200
                     hover:from-primary-400 hover:to-primary-500
                     hover:shadow-xl hover:shadow-primary-500/30
                     active:from-primary-600 active:to-primary-700
                     focus:outline-none focus:ring-2 focus:ring-primary-400 focus:ring-offset-2
                     dark:focus:ring-offset-slate-900"
      >
        <Sparkles class="w-4 h-4" />
        新建思维导图
      </button>
    </div>
  </header>
</template>
