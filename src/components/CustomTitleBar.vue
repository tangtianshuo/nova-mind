<script setup lang="ts">
import { ref } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { Minus, Square, X, Maximize2 } from 'lucide-vue-next';

const appWindow = getCurrentWindow();
const isMaximized = ref(false);

async function handleMinimize() {
  await appWindow.minimize();
}

async function handleMaximize() {
  if (isMaximized.value) {
    await appWindow.unmaximize();
  } else {
    await appWindow.maximize();
  }
  isMaximized.value = !isMaximized.value;
}

async function handleClose() {
  await appWindow.close();
}
</script>

<template>
  <div 
    class="flex items-center justify-between h-10 px-4 select-none
           bg-slate-900 dark:bg-slate-900
           border-b border-slate-800"
    data-tauri-drag-region
  >
    <div
      class="flex items-center gap-3"
      data-tauri-drag-region
    >
      <img 
        src="/icon.png" 
        alt="Nova Mind" 
        class="w-6 h-6 rounded-md"
      />
      <span class="text-sm font-semibold text-slate-200 dark:text-slate-200">
        Nova Mind
      </span>
    </div>

    <div
      class="flex items-center -mr-2"
    >
      <button
        class="flex items-center justify-center w-11 h-10 text-slate-400 hover:text-slate-200 hover:bg-slate-800 transition-all duration-150"
        title="最小化"
        @click.stop="handleMinimize"
      >
        <Minus class="w-4 h-4 transition-transform duration-150" />
      </button>

      <button
        class="flex items-center justify-center w-11 h-10 text-slate-400 hover:text-slate-200 hover:bg-slate-800 transition-all duration-150"
        :title="isMaximized ? '还原' : '最大化'"
        @click.stop="handleMaximize"
      >
        <Square
          v-if="!isMaximized"
          class="w-3.5 h-3.5 transition-transform duration-150"
        />
        <Maximize2
          v-else
          class="w-3.5 h-3.5 transition-transform duration-150"
        />
      </button>

      <button
        class="flex items-center justify-center w-11 h-10 text-slate-400 hover:text-white hover:bg-red-500 transition-all duration-150"
        title="关闭"
        @click.stop="handleClose"
      >
        <X class="w-4 h-4 transition-transform duration-150" />
      </button>
    </div>
  </div>
</template>
