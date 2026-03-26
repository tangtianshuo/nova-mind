<script setup lang="ts">
import { RouterLink, useRoute } from 'vue-router';
import {
  Home,
  Brain,
  MessageSquare,
  Layers,
  Settings,
} from 'lucide-vue-next';

const route = useRoute();

const menuItems = [
  { path: '/', name: '首页', icon: Home },
  { path: '/mindmap', name: '思维导图', icon: Brain },
  { path: '/chat', name: 'AI 对话', icon: MessageSquare },
  { path: '/skills', name: '技能管理', icon: Layers },
  { path: '/settings', name: '设置', icon: Settings },
];

const isActive = (path: string) => route.path === path;

const sidebarBgClass = 'bg-slate-900 dark:bg-slate-900';
const borderClass = 'border-slate-800 dark:border-slate-800';
</script>

<template>
  <aside
    :class="[
      'flex flex-col w-64 h-full',
      sidebarBgClass,
      borderClass,
      'border-r'
    ]"
  >
    <div :class="['flex items-center h-14 px-5', borderClass, 'border-b']">
      <div class="flex items-center gap-3">
        <img 
          src="/icon.png" 
          alt="Nova Mind" 
          class="w-8 h-8 rounded-lg"
        />
        <span class="text-lg font-bold bg-gradient-to-r from-sky-400 to-purple-500 bg-clip-text text-transparent">
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
            ? 'bg-gradient-to-r from-sky-500 to-purple-600 text-white shadow-lg shadow-sky-500/25 dark:shadow-sky-500/25'
            : 'text-slate-400 dark:text-slate-400 hover:bg-slate-800 dark:hover:bg-slate-800 hover:text-slate-200 dark:hover:text-slate-200'
        ]"
      >
        <component
          :is="item.icon"
          :class="[
            'h-5 w-5 transition-colors duration-200',
            isActive(item.path)
              ? 'text-white'
              : 'text-slate-500 group-hover:text-slate-300 dark:group-hover:text-slate-300'
          ]"
        />
        <span>{{ item.name }}</span>
        
        <div
          v-if="isActive(item.path)"
          class="absolute left-0 top-1/2 -translate-y-1/2 w-1 h-8 bg-sky-400 dark:bg-sky-400 rounded-r-full"
        />
      </RouterLink>
    </nav>
  </aside>
</template>
