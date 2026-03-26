<script setup lang="ts">
import { ref } from 'vue';
import { useSettings } from '@/composables/useSettings';
import { useKeyboard } from '@/composables/useKeyboard';
import { Settings, Keyboard, Info, Sun, Moon, Monitor } from 'lucide-vue-next';
import ShortcutInput from '@/components/ShortcutInput.vue';

const { settings, updateSetting } = useSettings();
const { activeShortcuts, hasConflict } = useKeyboard();

const activeTab = ref<'general' | 'shortcuts' | 'about'>('general');

const tabs = [
  { key: 'general', label: '通用设置', icon: Settings },
  { key: 'shortcuts', label: '快捷键', icon: Keyboard },
  { key: 'about', label: '关于', icon: Info },
];

function handleThemeChange(theme: 'light' | 'dark' | 'auto') {
  updateSetting('theme', theme);
}

function handleAutoSaveToggle() {
  updateSetting('autoSave', !settings.value.autoSave);
}

const themeOptions = [
  { value: 'light', label: '浅色', icon: Sun },
  { value: 'dark', label: '深色', icon: Moon },
  { value: 'auto', label: '跟随系统', icon: Monitor },
];
</script>

<template>
  <div class="max-w-4xl mx-auto">
    <h1 class="mb-8 text-2xl font-bold text-slate-800 dark:text-slate-100">
      设置
    </h1>

    <div class="flex gap-8">
      <nav class="w-52 flex-shrink-0 space-y-1">
        <button
          v-for="tab in tabs"
          :key="tab.key"
          :class="[
            'w-full flex items-center gap-3 px-4 py-3 rounded-xl text-sm font-medium transition-all duration-200',
            activeTab === tab.key
              ? 'bg-gradient-to-r from-primary-500 to-primary-600 text-white shadow-lg shadow-primary-500/25'
              : 'text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-slate-900 dark:hover:text-slate-200'
          ]"
          @click="activeTab = tab.key as 'general' | 'shortcuts' | 'about'"
        >
          <component
            :is="tab.icon"
            class="w-5 h-5"
          />
          {{ tab.label }}
        </button>
      </nav>

      <div class="flex-1 space-y-6">
        <template v-if="activeTab === 'general'">
          <section
            class="rounded-2xl border border-slate-200 dark:border-slate-700
                         bg-white dark:bg-slate-800 p-6 shadow-sm"
          >
            <h2 class="mb-6 text-lg font-semibold text-slate-800 dark:text-slate-100">
              通用设置
            </h2>

            <div class="space-y-6">
              <div class="flex items-center justify-between">
                <div>
                  <p class="text-sm font-medium text-slate-700 dark:text-slate-200">
                    主题
                  </p>
                  <p class="text-xs text-slate-500 dark:text-slate-400 mt-0.5">
                    选择应用外观
                  </p>
                </div>
                <div class="flex items-center gap-2 p-1.5 rounded-xl bg-slate-100 dark:bg-slate-700">
                  <button
                    v-for="option in themeOptions"
                    :key="option.value"
                    :title="option.label"
                    :class="[
                      'flex items-center justify-center w-8 h-8 rounded-lg transition-all duration-200',
                      settings.theme === option.value
                        ? 'bg-gradient-to-br from-primary-500 to-primary-600 text-white shadow-lg'
                        : 'text-slate-500 hover:text-slate-700 dark:text-slate-400 dark:hover:text-slate-200'
                    ]"
                    @click="handleThemeChange(option.value as 'light' | 'dark' | 'auto')"
                  >
                    <component
                      :is="option.icon"
                      class="w-4 h-4"
                    />
                  </button>
                </div>
              </div>

              <div class="h-px bg-slate-200 dark:bg-slate-700" />

              <div class="flex items-center justify-between">
                <div>
                  <p class="text-sm font-medium text-slate-700 dark:text-slate-200">
                    自动保存
                  </p>
                  <p class="text-xs text-slate-500 dark:text-slate-400 mt-0.5">
                    开启后自动保存草稿
                  </p>
                </div>
                <button
                  :class="[
                    'relative w-11 h-6 rounded-full transition-colors duration-200',
                    settings.autoSave ? 'bg-gradient-to-r from-primary-500 to-primary-600' : 'bg-slate-300 dark:bg-slate-600'
                  ]"
                  role="switch"
                  :aria-checked="settings.autoSave"
                  @click="handleAutoSaveToggle"
                >
                  <span
                    :class="[
                      'absolute top-0.5 w-5 h-5 rounded-full bg-white shadow-sm transition-transform duration-200',
                      settings.autoSave ? 'left-5.5' : 'left-0.5'
                    ]"
                  />
                </button>
              </div>
            </div>
          </section>
        </template>

        <template v-else-if="activeTab === 'shortcuts'">
          <section
            class="rounded-2xl border border-slate-200 dark:border-slate-700
                         bg-white dark:bg-slate-800 p-6 shadow-sm"
          >
            <h2 class="mb-6 text-lg font-semibold text-slate-800 dark:text-slate-100">
              快捷键
            </h2>

            <div class="space-y-4">
              <div
                v-for="shortcut in activeShortcuts"
                :key="shortcut.action"
                class="flex items-center justify-between py-3 px-4 rounded-xl
                       bg-slate-50 dark:bg-slate-800/50
                       hover:bg-slate-100 dark:hover:bg-slate-800
                       transition-colors duration-200"
              >
                <span class="text-sm text-slate-700 dark:text-slate-300">{{ shortcut.description }}</span>
                <ShortcutInput
                  :model-value="{ key: shortcut.key, modifiers: shortcut.modifiers }"
                  :conflict="hasConflict(shortcut.key, shortcut.modifiers)"
                />
              </div>
            </div>
          </section>
        </template>

        <template v-else>
          <section
            class="rounded-2xl border border-slate-200 dark:border-slate-700
                         bg-white dark:bg-slate-800 p-6 shadow-sm"
          >
            <div class="flex items-center gap-4 mb-6">
              <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-primary-500 to-primary-600 flex items-center justify-center shadow-lg shadow-primary-500/25">
                <span class="text-xl font-bold text-white">N</span>
              </div>
              <div>
                <h2 class="text-lg font-bold text-slate-800 dark:text-slate-100">
                  关于 Nova Mind
                </h2>
                <p class="text-sm text-slate-500 dark:text-slate-400">
                  版本 1.0.0
                </p>
              </div>
            </div>
            <div class="space-y-2 text-sm text-slate-600 dark:text-slate-400">
              <p>基于 Tauri + Vue 3 + Rust 构建</p>
              <p>AI 驱动的产品管理思维导图工具</p>
              <p class="text-slate-400 dark:text-slate-500 mt-4">
                © 2024 Nova Mind Team
              </p>
            </div>
          </section>
        </template>
      </div>
    </div>
  </div>
</template>
