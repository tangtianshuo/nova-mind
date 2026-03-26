<script setup lang="ts">
import { ref, onMounted } from 'vue';

const isConnected = ref(false);
const isLoading = ref(true);

onMounted(() => {
  setTimeout(() => {
    isConnected.value = true;
    isLoading.value = false;
  }, 1000);
});
</script>

<template>
  <div class="flex items-center gap-1.5">
    <template v-if="isLoading">
      <div class="w-2 h-2 rounded-full bg-slate-400 dark:bg-slate-500 animate-pulse" />
      <span class="text-xs text-slate-500 dark:text-slate-400">连接中...</span>
    </template>
    <template v-else>
      <div class="relative">
        <div
          :class="[
            'w-2 h-2 rounded-full',
            isConnected 
              ? 'bg-green-500 dark:bg-green-500' 
              : 'bg-red-500 dark:bg-red-500'
          ]"
        >
          <span
            v-if="isConnected"
            class="absolute inset-0 rounded-full animate-ping opacity-75 bg-green-400 dark:bg-green-400"
          />
        </div>
      </div>
      <span
        :class="[
          'text-xs font-medium',
          isConnected 
            ? 'text-green-500 dark:text-green-400' 
            : 'text-red-500 dark:text-red-400'
        ]"
      >
        {{ isConnected ? '已连接' : '未连接' }}
      </span>
    </template>
  </div>
</template>
