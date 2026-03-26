import { ref, computed, onMounted, onUnmounted, watch } from 'vue';
import { useVirtualizer } from '@tanstack/vue-virtual';

export interface UseVirtualScrollOptions<T> {
  items: () => T[];
  estimateSize?: number;
  overscan?: number;
  getItemKey?: (item: T, index: number) => string | number;
}

export function useVirtualScroll<T>(options: UseVirtualScrollOptions<T>) {
  const containerRef = ref<HTMLElement | null>(null);
  const items = computed(() => options.items());
  const estimateSize = options.estimateSize ?? 80;
  const overscan = options.overscan ?? 5;

  const getItemKey = options.getItemKey ?? ((item: T, index: number) => index);

  const virtualizer = useVirtualizer({
    count: computed(() => items.value.length),
    getScrollElement: () => containerRef.value,
    estimateSize: () => estimateSize,
    overscan,
    getItemKey,
  });

  const virtualItems = computed(() => virtualizer.value.getVirtualItems());
  const totalSize = computed(() => virtualizer.value.getTotalSize());

  function scrollToIndex(index: number, align: 'start' | 'center' | 'end' = 'start') {
    virtualizer.value.scrollToIndex(index, { align });
  }

  function scrollToBottom() {
    if (items.value.length > 0) {
      scrollToIndex(items.value.length - 1, 'end');
    }
  }

  function scrollToTop() {
    scrollToIndex(0, 'start');
  }

  function getScrollProgress() {
    return {
      scrollTop: containerRef.value?.scrollTop ?? 0,
      scrollHeight: containerRef.value?.scrollHeight ?? 0,
      clientHeight: containerRef.value?.clientHeight ?? 0,
      progress: containerRef.value
        ? (containerRef.value.scrollTop / (containerRef.value.scrollHeight - containerRef.value.clientHeight)) * 100
        : 0,
    };
  }

  watch(items, () => {
    virtualizer.value.measure();
  });

  return {
    containerRef,
    virtualItems,
    totalSize,
    scrollToIndex,
    scrollToBottom,
    scrollToTop,
    getScrollProgress,
    virtualizer,
  };
}

export function useInfiniteScroll(
  loadMore: () => Promise<void>,
  options: {
    threshold?: number;
    enabled?: () => boolean;
  } = {}
) {
  const { threshold = 100, enabled = () => true } = options;
  const isLoading = ref(false);
  const hasMore = ref(true);
  const containerRef = ref<HTMLElement | null>(null);

  function handleScroll() {
    if (!containerRef.value || !enabled() || isLoading.value || !hasMore.value) {
      return;
    }

    const { scrollTop, scrollHeight, clientHeight } = containerRef.value;
    const distanceFromBottom = scrollHeight - scrollTop - clientHeight;

    if (distanceFromBottom < threshold) {
      loadMoreItems();
    }
  }

  async function loadMoreItems() {
    if (isLoading.value || !hasMore.value) return;

    isLoading.value = true;
    try {
      await loadMore();
    } catch (error) {
      console.error('Failed to load more items:', error);
    } finally {
      isLoading.value = false;
    }
  }

  function reset() {
    hasMore.value = true;
    isLoading.value = false;
  }

  function setHasMore(value: boolean) {
    hasMore.value = value;
  }

  onMounted(() => {
    if (containerRef.value) {
      containerRef.value.addEventListener('scroll', handleScroll);
    }
  });

  onUnmounted(() => {
    if (containerRef.value) {
      containerRef.value.removeEventListener('scroll', handleScroll);
    }
  });

  return {
    containerRef,
    isLoading,
    hasMore,
    loadMoreItems,
    reset,
    setHasMore,
  };
}
