<script setup lang="ts">
import { ref } from 'vue';
import { exportApi } from '@/api';
import { X, Download, FileText, File, Loader2 } from 'lucide-vue-next';

const props = defineProps<{
  conversationId: number;
  title: string;
}>();

const emit = defineEmits<{
  close: [];
  success: [filePath: string];
  error: [message: string];
}>();

const format = ref<'markdown' | 'word'>('markdown');
const includeMetadata = ref(true);
const isExporting = ref(false);
const error = ref<string | null>(null);

async function handleExport() {
  isExporting.value = true;
  error.value = null;

  try {
    const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
    const baseName = `${props.title}_${timestamp}`;

    let result;
    if (format.value === 'markdown') {
      const filePath = `${baseName}.md`;
      result = await exportApi.exportMarkdown({
        conversationId: props.conversationId,
        title: props.title,
        filePath,
        includeMetadata: includeMetadata.value,
      });
    } else {
      const filePath = `${baseName}.docx`;
      result = await exportApi.exportWord({
        conversationId: props.conversationId,
        title: props.title,
        filePath,
      });
    }

    if (result.success && result.data) {
      emit('success', result.data);
      emit('close');
    } else {
      error.value = result.error || '导出失败';
      emit('error', error.value);
    }
  } catch (err) {
    error.value = err instanceof Error ? err.message : '导出失败';
    emit('error', error.value);
  } finally {
    isExporting.value = false;
  }
}

function handleClose() {
  if (!isExporting.value) {
    emit('close');
  }
}
</script>

<template>
  <div class="fixed inset-0 z-50 flex items-center justify-center">
    <div
      class="absolute inset-0 bg-black/50 backdrop-blur-sm"
      @click="handleClose"
    />
    <div class="relative w-full max-w-md rounded-2xl bg-slate-800 dark:bg-slate-900 border border-slate-700 dark:border-slate-700 shadow-2xl">
      <div class="flex items-center justify-between p-6 border-b border-slate-700 dark:border-slate-700">
        <h3 class="text-lg font-semibold text-slate-100">
          导出对话
        </h3>
        <button
          class="p-1 rounded-lg hover:bg-slate-700 transition-colors"
          :disabled="isExporting"
          @click="handleClose"
        >
          <X class="w-5 h-5 text-slate-400" />
        </button>
      </div>

      <div class="p-6 space-y-6">
        <div>
          <label class="block text-sm font-medium text-slate-300 mb-3">
            导出格式
          </label>
          <div class="grid grid-cols-2 gap-3">
            <button
              :class="[
                'flex items-center gap-3 p-4 rounded-xl border-2 transition-all',
                format === 'markdown'
                  ? 'border-primary-500 bg-primary-500/10'
                  : 'border-slate-600 hover:border-slate-500'
              ]"
              @click="format = 'markdown'"
            >
              <FileText
                :class="[
                  'w-6 h-6',
                  format === 'markdown' ? 'text-primary-400' : 'text-slate-400'
                ]"
              />
              <div class="text-left">
                <div class="font-medium text-slate-200">
                  Markdown
                </div>
                <div class="text-xs text-slate-400">
                  .md 文件
                </div>
              </div>
            </button>

            <button
              :class="[
                'flex items-center gap-3 p-4 rounded-xl border-2 transition-all',
                format === 'word'
                  ? 'border-primary-500 bg-primary-500/10'
                  : 'border-slate-600 hover:border-slate-500'
              ]"
              @click="format = 'word'"
            >
              <File
                :class="[
                  'w-6 h-6',
                  format === 'word' ? 'text-primary-400' : 'text-slate-400'
                ]"
              />
              <div class="text-left">
                <div class="font-medium text-slate-200">
                  Word
                </div>
                <div class="text-xs text-slate-400">
                  .docx 文件
                </div>
              </div>
            </button>
          </div>
        </div>

        <div
          v-if="format === 'markdown'"
          class="flex items-center gap-3"
        >
          <input
            id="include-metadata"
            v-model="includeMetadata"
            type="checkbox"
            class="w-4 h-4 rounded border-slate-600 bg-slate-700 text-primary-500 focus:ring-primary-500 focus:ring-offset-0"
          />
          <label
            for="include-metadata"
            class="text-sm text-slate-300"
          >
            包含元数据（导出时间等）
          </label>
        </div>

        <div
          v-if="error"
          class="p-3 rounded-lg bg-red-500/10 border border-red-500/20"
        >
          <p class="text-sm text-red-400">
            {{ error }}
          </p>
        </div>
      </div>

      <div class="flex items-center justify-end gap-3 p-6 border-t border-slate-700 dark:border-slate-700">
        <button
          class="px-4 py-2 text-sm font-medium text-slate-300 hover:text-slate-100 transition-colors"
          :disabled="isExporting"
          @click="handleClose"
        >
          取消
        </button>
        <button
          class="inline-flex items-center gap-2 px-6 py-2.5 rounded-xl bg-gradient-to-r from-primary-500 to-primary-600 text-sm font-medium text-white shadow-lg shadow-primary-500/25 transition-all hover:from-primary-400 hover:to-primary-500 disabled:opacity-50"
          :disabled="isExporting"
          @click="handleExport"
        >
          <Loader2
            v-if="isExporting"
            class="w-4 h-4 animate-spin"
          />
          <Download
            v-else
            class="w-4 h-4"
          />
          {{ isExporting ? '导出中...' : '导出' }}
        </button>
      </div>
    </div>
  </div>
</template>
