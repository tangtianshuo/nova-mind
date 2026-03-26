<script setup lang="ts">
import { ref } from 'vue';

const canvas = ref<HTMLDivElement | null>(null);
const nodes = ref([
  { id: 1, text: '中心主题', x: 400, y: 300, children: [] },
]);

const addNode = (parentId: number) => {
  const parent = nodes.value.find(n => n.id === parentId);
  if (parent) {
    const newId = Date.now();
    parent.children.push({
      id: newId,
      text: '新节点',
      x: parent.x + 150,
      y: parent.y + 50,
      children: [],
    });
  }
};

const deleteNode = (nodeId: number) => {
  const deleteRecursive = (items: any[]) => {
    return items.filter(item => {
      if (item.id === nodeId) return false;
      if (item.children) {
        item.children = deleteRecursive(item.children);
      }
      return true;
    });
  };
  nodes.value = deleteRecursive(nodes.value);
};
</script>

<template>
  <div class="flex h-full flex-col">
    <div class="mb-4 flex items-center justify-between">
      <h2 class="text-xl font-semibold">
        思维导图编辑器
      </h2>
      <div class="flex gap-2">
        <button class="rounded-lg bg-primary-600 px-4 py-2 text-sm hover:bg-primary-700">
          保存
        </button>
        <button class="rounded-lg bg-gray-700 px-4 py-2 text-sm hover:bg-gray-600">
          导出
        </button>
      </div>
    </div>

    <div
      ref="canvas"
      class="flex-1 overflow-hidden rounded-lg border border-gray-700 bg-gray-800"
    >
      <div class="relative h-full w-full">
        <div
          v-for="node in nodes"
          :key="node.id"
          class="absolute flex flex-col items-center"
          :style="{ left: `${node.x}px`, top: `${node.y}px` }"
        >
          <div class="min-w-24 rounded-lg bg-primary-600 px-4 py-2 text-center text-white shadow-lg">
            {{ node.text }}
          </div>
          <div class="mt-2 flex gap-1">
            <button
              class="rounded bg-green-600 px-2 py-1 text-xs hover:bg-green-700"
              @click="addNode(node.id)"
            >
              +
            </button>
            <button
              class="rounded bg-red-600 px-2 py-1 text-xs hover:bg-red-700"
              @click="deleteNode(node.id)"
            >
              -
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
