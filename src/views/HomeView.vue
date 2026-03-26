<script setup lang="ts">
import { ref } from 'vue';
import { 
  FileText, 
  Sparkles, 
  Download, 
  Upload,
  ArrowRight,
  Clock
} from 'lucide-vue-next';

const recentFiles = ref([
  { id: 1, name: '产品需求文档', path: '/docs/prd.md', updatedAt: '2024-03-20' },
  { id: 2, name: '竞品分析', path: '/docs/competitor.md', updatedAt: '2024-03-19' },
  { id: 3, name: '用户故事地图', path: '/docs/user-story.md', updatedAt: '2024-03-18' },
]);
</script>

<template>
  <div class="h-full p-8 space-y-8">
    <div class="relative overflow-hidden rounded-2xl bg-gradient-to-br from-primary-600 via-primary-500 to-secondary-500 p-8 shadow-xl shadow-primary-500/20">
      <div class="absolute inset-0 bg-[url('data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNjAiIGhlaWdodD0iNjAiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+PGRlZnM+PHBhdHRlcm4gaWQ9ImdyaWQiIHdpZHRoPSI2MCIgaGVpZ2h0PSI2MCIgcGF0dGVyblVuaXRzPSJ1c2VyU3BhY2VPblVzZSI+PHBhdGggZD0iTSA2MCAwIEwgMCAwIDAgNjAiIGZpbGw9Im5vbmUiIHN0cm9rZT0icmdiYSgyNTUsMjU1LDI1NSwwLjA1KSIgc3Ryb2tlLXdpZHRoPSIxIi8+PC9wYXR0ZXJuPjwvZGVmcz48cmVjdCB3aWR0aD0iMTAwJSIgaGVpZ2h0PSIxMDAlIiBmaWxsPSJ1cmwoI2dyaWQpIi8+PC9zdmc+')] opacity-50" />
      
      <div class="relative">
        <div class="flex items-center gap-2 mb-2">
          <Sparkles class="w-5 h-5 text-white/80" />
          <span class="text-sm font-medium text-white/80">AI 驱动</span>
        </div>
        <h2 class="text-3xl font-bold text-white mb-2">欢迎使用 Nova Mind</h2>
        <p class="text-primary-100 text-base max-w-xl mb-6">
          AI 驱动的产品管理思维导图工具，让你的创意可视化，提升团队协作效率
        </p>
        <button class="inline-flex items-center gap-2 px-6 py-3 rounded-xl
                       bg-white text-primary-600 font-medium
                       shadow-lg hover:shadow-xl
                       transition-all duration-200
                       hover:bg-primary-50">
          创建新思维导图
          <ArrowRight class="w-4 h-4" />
        </button>
      </div>
    </div>

    <div class="grid gap-6 lg:grid-cols-2">
      <section class="rounded-2xl border border-slate-200 dark:border-slate-700
                      bg-white dark:bg-slate-800 p-6 shadow-sm">
        <div class="flex items-center justify-between mb-4">
          <h3 class="text-lg font-semibold text-slate-800 dark:text-slate-100 flex items-center gap-2">
            <Clock class="w-5 h-5 text-primary-500" />
            最近文件
          </h3>
          <button class="text-sm text-primary-500 hover:text-primary-600 font-medium">
            查看全部
          </button>
        </div>
        
        <div class="space-y-3">
          <div
            v-for="file in recentFiles"
            :key="file.id"
            class="group flex items-center gap-4 p-4 rounded-xl
                   border border-slate-100 dark:border-slate-700
                   bg-slate-50 dark:bg-slate-800/50
                   hover:border-primary-200 dark:hover:border-primary-400/50
                   hover:bg-white dark:hover:bg-slate-800
                   cursor-pointer transition-all duration-200"
          >
            <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-primary-100 to-primary-50 dark:from-primary-900/30 dark:to-primary-800/20 flex items-center justify-center">
              <FileText class="w-5 h-5 text-primary-600 dark:text-primary-400" />
            </div>
            <div class="flex-1 min-w-0">
              <h4 class="font-medium text-slate-800 dark:text-slate-200 truncate">{{ file.name }}</h4>
              <p class="text-sm text-slate-500 dark:text-slate-400 truncate">{{ file.path }}</p>
            </div>
            <div class="text-xs text-slate-400 dark:text-slate-500">
              {{ file.updatedAt }}
            </div>
          </div>
        </div>
      </section>

      <section class="rounded-2xl border border-slate-200 dark:border-slate-700
                      bg-white dark:bg-slate-800 p-6 shadow-sm">
        <h3 class="text-lg font-semibold text-slate-800 dark:text-slate-100 mb-4 flex items-center gap-2">
          <Sparkles class="w-5 h-5 text-secondary-500" />
          快捷操作
        </h3>
        
        <div class="grid grid-cols-2 gap-3">
          <button class="flex items-center gap-3 p-4 rounded-xl
                         border border-slate-100 dark:border-slate-700
                         bg-slate-50 dark:bg-slate-800/50
                         hover:border-primary-200 dark:hover:border-primary-400/50
                         hover:bg-white dark:hover:bg-slate-800
                         transition-all duration-200 group">
            <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-blue-100 to-blue-50 dark:from-blue-900/30 dark:to-blue-800/20 flex items-center justify-center">
              <Upload class="w-5 h-5 text-blue-600 dark:text-blue-400" />
            </div>
            <div class="text-left">
              <div class="font-medium text-slate-800 dark:text-slate-200">导入文档</div>
              <div class="text-xs text-slate-500 dark:text-slate-400">支持 Markdown</div>
            </div>
          </button>
          
          <button class="flex items-center gap-3 p-4 rounded-xl
                         border border-slate-100 dark:border-slate-700
                         bg-slate-50 dark:bg-slate-800/50
                         hover:border-primary-200 dark:hover:border-primary-400/50
                         hover:bg-white dark:hover:bg-slate-800
                         transition-all duration-200 group">
            <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-purple-100 to-purple-50 dark:from-purple-900/30 dark:to-purple-800/20 flex items-center justify-center">
              <Sparkles class="w-5 h-5 text-purple-600 dark:text-purple-400" />
            </div>
            <div class="text-left">
              <div class="font-medium text-slate-800 dark:text-slate-200">AI 生成</div>
              <div class="text-xs text-slate-500 dark:text-slate-400">智能创作</div>
            </div>
          </button>
          
          <button class="flex items-center gap-3 p-4 rounded-xl
                         border border-slate-100 dark:border-slate-700
                         bg-slate-50 dark:bg-slate-800/50
                         hover:border-primary-200 dark:hover:border-primary-400/50
                         hover:bg-white dark:hover:bg-slate-800
                         transition-all duration-200 group">
            <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-green-100 to-green-50 dark:from-green-900/30 dark:to-green-800/20 flex items-center justify-center">
              <Download class="w-5 h-5 text-green-600 dark:text-green-400" />
            </div>
            <div class="text-left">
              <div class="font-medium text-slate-800 dark:text-slate-200">导出</div>
              <div class="text-xs text-slate-500 dark:text-slate-400">多格式支持</div>
            </div>
          </button>
          
          <button class="flex items-center gap-3 p-4 rounded-xl
                         border border-slate-100 dark:border-slate-700
                         bg-slate-50 dark:bg-slate-800/50
                         hover:border-primary-200 dark:hover:border-primary-400/50
                         hover:bg-white dark:hover:bg-slate-800
                         transition-all duration-200 group">
            <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-orange-100 to-orange-50 dark:from-orange-900/30 dark:to-orange-800/20 flex items-center justify-center">
              <FileText class="w-5 h-5 text-orange-600 dark:text-orange-400" />
            </div>
            <div class="text-left">
              <div class="font-medium text-slate-800 dark:text-slate-200">模板</div>
              <div class="text-xs text-slate-500 dark:text-slate-400">快速开始</div>
            </div>
          </button>
        </div>
      </section>
    </div>
  </div>
</template>
