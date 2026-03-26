<script setup lang="ts">
import { ref, computed } from 'vue';
import { Keyboard, Mic, X, AlertCircle } from 'lucide-vue-next';

const props = defineProps<{
  modelValue: {
    key: string;
    modifiers: ('ctrl' | 'alt' | 'shift' | 'meta')[];
  };
  disabled?: boolean;
  conflict?: boolean;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: { key: string; modifiers: ('ctrl' | 'alt' | 'shift' | 'meta')[] }): void;
}>();

const isRecording = ref(false);

function startRecording() {
  if (props.disabled) return;
  isRecording.value = true;
}

function handleKeyDown(event: KeyboardEvent) {
  if (!isRecording.value) return;

  event.preventDefault();
  event.stopPropagation();

  const modifiers: ('ctrl' | 'alt' | 'shift' | 'meta')[] = [];
  if (event.ctrlKey) modifiers.push('ctrl');
  if (event.altKey) modifiers.push('alt');
  if (event.shiftKey) modifiers.push('shift');
  if (event.metaKey) modifiers.push('meta');

  if (event.key !== 'Control' && event.key !== 'Alt' && event.key !== 'Shift' && event.key !== 'Meta') {
    emit('update:modelValue', { key: event.key, modifiers });
    isRecording.value = false;
  }
}

function handleBlur() {
  isRecording.value = false;
}

const modifierSymbols: Record<string, string> = {
  ctrl: '⌃',
  alt: '⌥',
  shift: '⇧',
  meta: '⌘'
};

const modifierNames: Record<string, string> = {
  ctrl: 'Ctrl',
  alt: 'Alt',
  shift: 'Shift',
  meta: 'Cmd'
};

const displayShortcut = computed(() => {
  if (isRecording.value) return '正在聆听...';

  if (!props.modelValue.key) return '点击设置快捷键';

  const parts: string[] = [];
  props.modelValue.modifiers.forEach(mod => {
    parts.push(`${modifierSymbols[mod]} ${modifierNames[mod]}`);
  });
  parts.push(props.modelValue.key.toUpperCase());

  return parts.join(' + ');
});
</script>

<template>
  <div
    class="group relative"
    :class="{ 'cursor-not-allowed': disabled }"
  >
    <div
      class="relative flex items-center gap-3 rounded-xl border bg-white dark:bg-slate-800 px-4 py-2.5 shadow-sm transition-all duration-200"
      :class="[
        disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer',
        isRecording ? 'border-primary-400 dark:border-primary-500 ring-2 ring-primary-400/30 dark:ring-primary-500/30' : 'border-slate-200 dark:border-slate-700',
        conflict ? 'border-red-400 dark:border-red-500 ring-2 ring-red-400/30' : '',
        !disabled && !isRecording && !conflict && 'hover:border-primary-300 dark:hover:border-primary-400/50 hover:shadow-md dark:hover:shadow-slate-900/50'
      ]"
      tabindex="0"
      role="button"
      :aria-label="`快捷键设置，当前: ${displayShortcut}`"
      :aria-pressed="isRecording"
      @click="startRecording"
      @keydown="handleKeyDown"
      @blur="handleBlur"
      @keydown.enter.space.prevent="startRecording"
    >
      <div class="flex-shrink-0 text-slate-400 dark:text-slate-500 transition-colors duration-200">
        <Keyboard
          v-if="!isRecording"
          class="h-4 w-4"
        />
        <div
          v-else
          class="relative"
        >
          <Mic class="h-4 w-4 text-primary-500 animate-pulse" />
        </div>
      </div>

      <span
        class="flex-1 truncate text-sm font-medium tabular-nums transition-colors duration-200"
        :class="[
          isRecording ? 'text-primary-600 dark:text-primary-400' : 'text-slate-700 dark:text-slate-200',
          conflict ? 'text-red-600 dark:text-red-400' : ''
        ]"
      >
        {{ displayShortcut }}
      </span>

      <button
        v-if="!isRecording && !disabled"
        type="button"
        class="flex-shrink-0 rounded-lg p-1.5 text-slate-400 dark:text-slate-500 opacity-0 transition-all duration-200 hover:bg-slate-100 dark:hover:bg-slate-700 hover:text-slate-600 dark:hover:text-slate-300 group-hover:opacity-100"
        aria-label="录制快捷键"
        @click.stop="startRecording"
      >
        <Mic class="h-3.5 w-3.5" />
      </button>

      <button
        v-if="isRecording"
        type="button"
        class="flex-shrink-0 rounded-lg p-1.5 text-slate-400 dark:text-slate-500 transition-colors duration-200 hover:bg-slate-100 dark:hover:bg-slate-700 hover:text-slate-600 dark:hover:text-slate-300"
        aria-label="取消录制"
        @click.stop="handleBlur"
      >
        <X class="h-3.5 w-3.5" />
      </button>
    </div>

    <Transition
      enter-active-class="transition duration-200 ease-out"
      enter-from-class="opacity-0 -translate-y-1"
      enter-to-class="opacity-100 translate-y-0"
      leave-active-class="transition duration-150 ease-in"
      leave-from-class="opacity-100 translate-y-0"
      leave-to-class="opacity-0 -translate-y-1"
    >
      <div
        v-if="conflict"
        class="absolute -top-7 left-0 flex items-center gap-1 text-xs font-medium text-red-500 dark:text-red-400"
      >
        <AlertCircle class="h-3 w-3" />
        <span>快捷键冲突</span>
      </div>
    </Transition>

    <div
      v-if="isRecording"
      class="pointer-events-none absolute inset-0 rounded-xl bg-primary-500/5 animate-pulse"
    />
  </div>
</template>
