<script setup lang="ts">
import { onMounted } from 'vue';
import { RouterView } from 'vue-router';
import CustomTitleBar from './components/CustomTitleBar.vue';
import AppSidebar from './components/AppSidebar.vue';
import AppHeader from './components/AppHeader.vue';
import { useThemeStore } from './stores/themeStore';

const themeStore = useThemeStore();

onMounted(() => {
  themeStore.initTheme();
});
</script>

<template>
  <div class="flex flex-col h-screen w-screen overflow-hidden bg-slate-100 dark:bg-slate-900">
    <CustomTitleBar />
    
    <div class="flex flex-1 overflow-hidden">
      <AppSidebar class="w-64 flex-shrink-0" />
      <div class="flex flex-col flex-1 overflow-hidden">
        <AppHeader class="flex-shrink-0" />
        <main class="flex-1 overflow-auto">
          <RouterView />
        </main>
      </div>
    </div>
  </div>
</template>

<style>
html, body, #app {
  height: 100%;
  width: 100%;
  margin: 0;
  padding: 0;
}

html.dark {
  color-scheme: dark;
}

html.light {
  color-scheme: light;
}
</style>
