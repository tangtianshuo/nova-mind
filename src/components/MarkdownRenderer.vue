<script setup lang="ts">
import { computed } from 'vue';
import { marked } from 'marked';
import hljs from 'highlight.js';
import 'highlight.js/styles/github-dark.css';

const props = defineProps<{ content: string; class?: string }>();

marked.setOptions({
  breaks: true,
  gfm: true,
});

const renderedContent = computed(() => marked.parse(props.content) as string);
</script>

<template>
  <div
    :class="['prose prose-invert max-w-none', props.class]"
    v-html="renderedContent"
  />
</template>

<style scoped>
.prose :deep(pre) {
  background: #0F172A;
  border: 1px solid #1E293B;
  border-radius: 10px;
  padding: 1rem;
  overflow-x: auto;
  box-shadow: 0 0 20px rgba(14, 165, 233, 0.1);
}

.prose :deep(code) {
  background: rgba(14, 165, 233, 0.1);
  color: #38BDF8;
  padding: 0.2rem 0.4rem;
  border-radius: 4px;
  font-size: 0.9em;
}

.prose :deep(pre code) {
  background: transparent;
  padding: 0;
}

.prose :deep(table) {
  border-collapse: collapse;
  width: 100%;
}

.prose :deep(th), .prose :deep(td) {
  border: 1px solid #334155;
  padding: 0.5rem;
}

.prose :deep(th) {
  background: rgba(14, 165, 233, 0.1);
}

.prose :deep(a) {
  color: #38BDF8;
  text-decoration: none;
}

.prose :deep(a:hover) {
  text-decoration: underline;
  text-shadow: 0 0 10px rgba(14, 165, 233, 0.3);
}
</style>
