<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

type StatusType = 'uninitialized' | 'downloading' | 'ready' | 'running' | 'error';

interface StatusConfig {
  text: string;
  dotClass: string;
  textClass: string;
  glowClass?: string;
  animate?: boolean;
}

const status = ref<StatusType>('uninitialized');
const version = ref('');
const isLoading = ref(true);
const error = ref<string | null>(null);
let pollInterval: ReturnType<typeof setInterval> | null = null;

const statusConfig: Record<StatusType, StatusConfig> = {
  uninitialized: {
    text: '未初始化',
    dotClass: 'bg-gray-500 dark:bg-gray-600',
    textClass: 'text-gray-500 dark:text-gray-400'
  },
  downloading: {
    text: '下载中',
    dotClass: 'bg-yellow-500',
    textClass: 'text-yellow-500 dark:text-yellow-400',
    animate: true
  },
  ready: {
    text: '就绪',
    dotClass: 'bg-primary-500',
    textClass: 'text-primary-500 dark:text-primary-400',
    glowClass: 'shadow-primary-500/30'
  },
  running: {
    text: '运行中',
    dotClass: 'bg-green-500',
    textClass: 'text-green-500 dark:text-green-400',
    glowClass: 'shadow-green-500/30'
  },
  error: {
    text: '异常',
    dotClass: 'bg-red-500',
    textClass: 'text-red-500 dark:text-red-400',
    glowClass: 'shadow-red-500/30'
  },
};

async function fetchStatus() {
  try {
    isLoading.value = true;
    error.value = null;
    const info = await invoke<{
      status: string;
      version: string;
      path: string;
      port?: number;
    }>('get_sandbox_status');
    status.value = info.status as StatusType;
    version.value = info.version;
  } catch (e) {
    error.value = e instanceof Error ? e.message : '获取状态失败';
    status.value = 'error';
  } finally {
    isLoading.value = false;
  }
}

onMounted(() => {
  fetchStatus();
  pollInterval = setInterval(fetchStatus, 30000);
});

onUnmounted(() => {
  if (pollInterval) {
    clearInterval(pollInterval);
  }
});

const currentConfig = computed(() => statusConfig[status.value]);
</script>

<template>
  <div class="relative">
    <div
      class="inline-flex items-center gap-2 px-3 py-1.5 rounded-lg border transition-all duration-200 dark:border-slate-700"
      :class="[currentConfig.textClass]"
    >
      <template v-if="isLoading">
        <div class="w-2 h-2 rounded-full bg-gray-500 dark:bg-gray-600 animate-pulse opacity-50" />
        <span class="text-xs text-gray-500 dark:text-gray-400">检查中...</span>
      </template>

      <template v-else>
        <div class="relative">
          <div
            :class="['w-2 h-2 rounded-full', currentConfig.dotClass]"
          />
          <div
            v-if="currentConfig.animate"
            class="absolute inset-0 rounded-full animate-ping opacity-50"
            style="background-color: currentColor; animation-duration: 1.5s;"
          />
        </div>

        <span class="text-xs font-medium">{{ currentConfig.text }}</span>

        <span
          v-if="version"
          class="text-xs opacity-60"
        >v{{ version }}</span>
      </template>
    </div>

    <Transition
      enter-active-class="transition duration-200 ease-out"
      enter-from-class="opacity-0 translate-y-1"
      enter-to-class="opacity-100 translate-y-0"
      leave-active-class="transition duration-150 ease-in"
      leave-from-class="opacity-100 translate-y-0"
      leave-to-class="opacity-0 translate-y-1"
    >
      <div
        v-if="error"
        class="absolute top-full left-0 mt-2 z-50 px-3 py-2 rounded-lg text-xs whitespace-nowrap
               bg-slate-800 dark:bg-slate-900 border border-slate-700 dark:border-slate-600
               shadow-xl"
      >
        <p class="text-red-400 dark:text-red-300">
          {{ error }}
        </p>
        <button
          class="mt-1 text-xs text-blue-400 hover:text-blue-300 dark:text-blue-300 dark:hover:text-blue-200"
          @click="fetchStatus"
        >
          重试
        </button>
      </div>
    </Transition>
  </div>
</template>
