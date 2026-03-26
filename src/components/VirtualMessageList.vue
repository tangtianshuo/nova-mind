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

const rowVirtualizer = useVirtualizer(
  computed(() => ({
    count: props.messages.length,
    getScrollElement: () => containerRef.value,
    estimateSize: () => 100,
    overscan: 5,
    getItemKey: (index: number) => props.messages[index]?.id ?? index,
  }))
);

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
          :key="virtualRow.index"
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
              <template v-else>
                {{ messages[virtualRow.index]?.content }}
              </template>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div
      v-if="isLoading"
      class="absolute bottom-4 left-1/2 -translate-x-1/2"
    >
      <div class="flex items-center gap-2 rounded-full bg-gray-100 dark:bg-gray-800 px-4 py-2 shadow-lg">
        <div class="h-4 w-4 animate-spin rounded-full border-2 border-primary-500 border-t-transparent" />
        <span class="text-sm text-gray-600 dark:text-gray-400">加载中...</span>
      </div>
    </div>
  </div>
</template>
