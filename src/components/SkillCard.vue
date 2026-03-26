<script setup lang="ts">
import { Pencil, Trash2, Tag } from 'lucide-vue-next';
import type { Skill } from '@/types';

defineProps<{ skill: Skill }>();
const emit = defineEmits<{ (e: 'edit', skill: Skill): void; (e: 'delete', id: number): void }>();
</script>

<template>
  <div
    class="group relative overflow-hidden rounded-2xl border border-slate-200 dark:border-slate-700
           bg-white dark:bg-slate-800 p-5
           shadow-sm
           transition-all duration-200
           hover:border-primary-300/50 dark:hover:border-primary-400/30
           hover:shadow-lg"
  >
    <div class="absolute inset-0 bg-gradient-to-br from-primary-500/5 via-transparent to-secondary-500/5 opacity-0 transition-opacity duration-200 group-hover:opacity-100" />

    <div class="relative">
      <div class="flex items-start justify-between mb-3">
        <h3 class="font-semibold text-slate-800 dark:text-slate-100">
          {{ skill.name }}
        </h3>
        <span
          v-if="skill.category"
          class="inline-flex items-center gap-1 rounded-full px-2.5 py-0.5 text-xs font-medium
                 bg-primary-100/80 dark:bg-primary-900/30 text-primary-700 dark:text-primary-300
                 border border-primary-200/50 dark:border-primary-700/50"
        >
          <Tag class="w-3 h-3" />
          {{ skill.category }}
        </span>
      </div>

      <p
        v-if="skill.description"
        class="mb-4 text-sm text-slate-600 dark:text-slate-400 line-clamp-2"
      >
        {{ skill.description }}
      </p>

      <div class="rounded-xl bg-slate-50/80 dark:bg-slate-800/50 p-3 mb-4">
        <div class="flex items-center justify-between text-xs text-slate-500 dark:text-slate-400">
          <span>版本 {{ skill.version }}</span>
          <span>{{ new Date(skill.created_at).toLocaleDateString() }}</span>
        </div>
      </div>

      <div class="flex gap-2 opacity-0 translate-y-1 transition-all duration-200 group-hover:opacity-100 group-hover:translate-y-0">
        <button
          class="flex-1 inline-flex items-center justify-center gap-1.5 rounded-lg
                 px-3 py-2 text-sm font-medium
                 bg-slate-100 dark:bg-slate-700
                 text-slate-700 dark:text-slate-200
                 hover:bg-primary-100 dark:hover:bg-primary-900/30
                 hover:text-primary-700 dark:hover:text-primary-300
                 transition-colors duration-200"
          @click="emit('edit', skill)"
        >
          <Pencil class="w-4 h-4" />
          编辑
        </button>
        <button
          class="flex-1 inline-flex items-center justify-center gap-1.5 rounded-lg
                 px-3 py-2 text-sm font-medium
                 bg-red-50 dark:bg-red-900/20
                 text-red-600 dark:text-red-400
                 hover:bg-red-100 dark:hover:bg-red-900/30
                 transition-colors duration-200"
          @click="emit('delete', skill.id!)"
        >
          <Trash2 class="w-4 h-4" />
          删除
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
