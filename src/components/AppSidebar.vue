<script setup lang="ts">
import { RouterLink, useRoute } from 'vue-router';
import {
  Home,
  Brain,
  MessageSquare,
  Layers,
  Settings,
  Monitor,
} from 'lucide-vue-next';
import SandboxStatus from './SandboxStatus.vue';

const route = useRoute();

const menuItems = [
  { path: '/', name: '首页', icon: Home },
  { path: '/mindmap', name: '思维导图', icon: Brain },
  { path: '/chat', name: 'AI 对话', icon: MessageSquare },
  { path: '/skills', name: '技能管理', icon: Layers },
  { path: '/settings', name: '设置', icon: Settings },
];

const isActive = (path: string) => route.path === path;
</script>

<template>
  <aside
    class="flex flex-col w-64 h-full
                bg-white/80 dark:bg-slate-900/80
                border-r border-slate-200 dark:border-slate-800
                backdrop-blur-xl"
  >
    <div class="flex items-center h-16 px-5 border-b border-slate-200 dark:border-slate-800">
      <div class="flex items-center gap-3">
        <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-primary-500 to-primary-600 flex items-center justify-center shadow-lg shadow-primary-500/25">
          <Monitor class="w-4 h-4 text-white" />
        </div>
        <span class="text-lg font-bold bg-gradient-to-r from-primary-500 to-secondary-500 bg-clip-text text-transparent">
          Nova Mind
        </span>
      </div>
    </div>

    <nav class="flex-1 p-3 space-y-1 overflow-y-auto">
      <RouterLink
        v-for="item in menuItems"
        :key="item.path"
        :to="item.path"
        :class="[
          'group relative flex items-center gap-3 px-4 py-3 rounded-xl text-sm font-medium',
          'transition-all duration-200',
          isActive(item.path)
            ? 'bg-gradient-to-r from-primary-500 to-primary-600 text-white shadow-lg shadow-primary-500/25'
            : 'text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-800/80 hover:text-slate-900 dark:hover:text-slate-200'
        ]"
      >
        <component
          :is="item.icon"
          :class="[
            'h-5 w-5 transition-colors duration-200',
            isActive(item.path)
              ? 'text-white'
              : 'text-slate-400 group-hover:text-slate-600 dark:text-slate-500 dark:group-hover:text-slate-300'
          ]"
        />
        <span>{{ item.name }}</span>
        
        <div
          v-if="isActive(item.path)"
          class="absolute left-0 top-1/2 -translate-y-1/2 w-1 h-8 bg-primary-400 rounded-r-full"
        />
      </RouterLink>
    </nav>

    <div class="p-4 border-t border-slate-200 dark:border-slate-800">
      <div class="flex items-center gap-2 mb-3">
        <div class="w-2 h-2 rounded-full bg-slate-400" />
        <span class="text-xs text-slate-500 dark:text-slate-400 font-medium">OpenClaw 沙箱</span>
      </div>
      <SandboxStatus />
    </div>
  </aside>
</template>
