<script setup lang="ts">
import { ref, computed } from 'vue';
import { useVirtualizer } from '@tanstack/vue-virtual';
import MarkdownRenderer from './MarkdownRenderer.vue';
import type { Message } from '@/types';

const props = defineProps<{
  messages: Message[];
  isLoading?: boolean;
}>();

const emit = defineEmits<{
  loadMore: [];
}>();

const containerRef = ref<HTMLElement | null>(null);
const isNearTop = ref(false);

const rowVirtualizer = useVirtualizer({
  count: props.messages.length,
  getScrollElement: () => containerRef.value,
  estimateSize: () => 100,
  overscan: 5,
});

const virtualRows = computed(() => rowVirtualizer.value.getVirtualItems());
const totalSize = computed(() => rowVirtualizer.value.getTotalSize());

function handleScroll() {
  if (!containerRef.value) return;
  const { scrollTop } = containerRef.value;
  if (scrollTop < 100 && !isNearTop.value) {
    isNearTop.value = true;
    emit('loadMore');
    setTimeout(() => {
      isNearTop.value = false;
    }, 1000);
  }
}

function scrollToBottom() {
  if (props.messages.length > 0) {
    rowVirtualizer.value.scrollToIndex(props.messages.length - 1, { align: 'end' });
  }
}

function scrollToIndex(index: number) {
  rowVirtualizer.value.scrollToIndex(index, { align: 'start' });
}

function formatTime(dateStr: string): string {
  return new Date(dateStr).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
}

defineExpose({
  scrollToBottom,
  scrollToIndex,
});
</script>

<template>
  <div class="relative h-full w-full">
    <div
      ref="containerRef"
      class="h-full overflow-auto"
      @scroll="handleScroll"
    >
      <div
        class="relative w-full"
        :style="{ height: `${totalSize}px` }"
      >
        <div
          v-for="virtualRow in virtualRows"
          :key="virtualRow.key"
          :ref="(el) => el && rowVirtualizer.value.measureElement(el)"
          :data-index="virtualRow.index"
          class="absolute left-0 w-full px-4 py-2"
          :style="{
            transform: `translateY(${virtualRow.start}px)`,
          }"
        >
          <div
            :class="[
              'flex',
              messages[virtualRow.index]?.role === 'user' ? 'justify-end' : 'justify-start'
            ]"
          >
            <div
              :class="[
                'max-w-[70%] rounded-lg p-4',
                messages[virtualRow.index]?.role === 'user'
                  ? 'bg-primary-600 text-white'
                  : 'bg-gray-100 dark:bg-gray-800 text-gray-900 dark:text-gray-100'
              ]"
            >
              <MarkdownRenderer
                v-if="messages[virtualRow.index]?.role === 'assistant'"
                :content="messages[virtualRow.index]?.content || ''"
                class="!text-gray-900 dark:!text-gray-100"
              />
              <p
                v-else
                class="whitespace-pre-wrap"
              >
                {{ messages[virtualRow.index]?.content }}
              </p>
              <p class="mt-2 text-xs opacity-60">
                {{ formatTime(messages[virtualRow.index]?.created_at || '') }}
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div
      v-if="isLoading"
      class="absolute top-2 left-1/2 transform -translate-x-1/2"
    >
      <div class="flex items-center gap-2 rounded-full bg-gray-800 px-4 py-2 text-white shadow-lg">
        <div class="h-4 w-4 animate-spin rounded-full border-2 border-white border-t-transparent" />
        <span class="text-sm">加载更多...</span>
      </div>
    </div>

    <button
      v-if="virtualRows.length > 10"
      class="absolute bottom-4 right-4 flex h-10 w-10 items-center justify-center rounded-full bg-primary-600 text-white shadow-lg transition-colors hover:bg-primary-700"
      title="滚动到底部"
      @click="scrollToBottom"
    >
      ↓
    </button>
  </div>
</template>
