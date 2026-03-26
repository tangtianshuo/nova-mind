<script setup lang="ts">
import { onMounted } from 'vue';
import { RouterView } from 'vue-router';
import AppHeader from './components/AppHeader.vue';
import AppSidebar from './components/AppSidebar.vue';
import { useThemeStore } from './stores/themeStore';

const themeStore = useThemeStore();

onMounted(() => {
  themeStore.initTheme();
});
</script>

<template>
  <div
    class="flex h-screen w-screen overflow-hidden"
    :class="themeStore.effectiveTheme === 'dark' ? 'dark' : 'light'"
  >
    <AppSidebar class="w-64 flex-shrink-0" />
    <div class="flex flex-col flex-1 overflow-hidden">
      <AppHeader class="h-14 flex-shrink-0" />
      <main class="flex-1 overflow-auto p-6">
        <RouterView />
      </main>
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
</style>
