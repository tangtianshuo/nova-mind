<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, computed } from 'vue';
import { 
  Save, 
  Download, 
  Plus, 
  Trash2, 
  Edit3, 
  Undo, 
  Redo,
  ZoomIn,
  ZoomOut,
  Maximize2,
} from 'lucide-vue-next';
import { mindmapApi } from '@/api';

interface MindmapNode {
  id: string;
  text: string;
  x: number;
  y: number;
  children: MindmapNode[];
  collapsed?: boolean;
  color?: string;
}

interface HistoryState {
  nodes: MindmapNode[];
  selectedId: string | null;
}

const canvas = ref<HTMLDivElement | null>(null);
const containerRef = ref<HTMLDivElement | null>(null);
const mindmapId = ref<number | null>(null);
const mindmapTitle = ref('未命名思维导图');
const nodes = ref<MindmapNode[]>([]);
const selectedNodeId = ref<string | null>(null);
const editingNodeId = ref<string | null>(null);
const editingText = ref('');
const isSaving = ref(false);
const isDragging = ref(false);
const isPanning = ref(false);
const draggedNodeId = ref<string | null>(null);
const dragOffset = ref({ x: 0, y: 0 });
const panOffset = ref({ x: 0, y: 0 });
const scale = ref(1);
const lastMousePos = ref({ x: 0, y: 0 });
const autoSaveTimer = ref<number | null>(null);

const history = ref<HistoryState[]>([]);
const historyIndex = ref(-1);
const maxHistorySize = 50;

const canUndo = computed(() => historyIndex.value > 0);
const canRedo = computed(() => historyIndex.value < history.value.length - 1);

const selectedNode = computed(() => {
  if (!selectedNodeId.value) return null;
  return findNode(nodes.value, selectedNodeId.value);
});

function generateId(): string {
  return `node_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
}

function findNode(nodeList: MindmapNode[], id: string): MindmapNode | null {
  for (const node of nodeList) {
    if (node.id === id) return node;
    const found = findNode(node.children, id);
    if (found) return found;
  }
  return null;
}

function saveToHistory() {
  const state: HistoryState = {
    nodes: JSON.parse(JSON.stringify(nodes.value)),
    selectedId: selectedNodeId.value
  };
  
  if (historyIndex.value < history.value.length - 1) {
    history.value = history.value.slice(0, historyIndex.value + 1);
  }
  
  history.value.push(state);
  
  if (history.value.length > maxHistorySize) {
    history.value.shift();
  } else {
    historyIndex.value++;
  }
}

function undo() {
  if (!canUndo.value) return;
  historyIndex.value--;
  const state = history.value[historyIndex.value];
  nodes.value = JSON.parse(JSON.stringify(state.nodes));
  selectedNodeId.value = state.selectedId;
}

function redo() {
  if (!canRedo.value) return;
  historyIndex.value++;
  const state = history.value[historyIndex.value];
  nodes.value = JSON.parse(JSON.stringify(state.nodes));
  selectedNodeId.value = state.selectedId;
}

function addNode(parentId: string) {
  saveToHistory();
  const parent = findNode(nodes.value, parentId);
  if (parent) {
    const newId = generateId();
    const offset = parent.children.length * 30;
    parent.children.push({
      id: newId,
      text: '新节点',
      x: parent.x + 180,
      y: parent.y + offset,
      children: [],
      collapsed: false
    });
    selectedNodeId.value = newId;
    startEditing(newId, '新节点');
    scheduleAutoSave();
  }
}

function deleteNode(nodeId: string) {
  saveToHistory();
  const deleteRecursive = (items: MindmapNode[]): MindmapNode[] => {
    return items.filter(item => {
      if (item.id === nodeId) return false;
      if (item.children.length > 0) {
        item.children = deleteRecursive(item.children);
      }
      return true;
    });
  };
  nodes.value = deleteRecursive(nodes.value);
  if (selectedNodeId.value === nodeId) {
    selectedNodeId.value = null;
  }
  scheduleAutoSave();
}

function startEditing(nodeId: string, currentText: string) {
  editingNodeId.value = nodeId;
  editingText.value = currentText;
}

function finishEditing() {
  if (editingNodeId.value) {
    saveToHistory();
    const node = findNode(nodes.value, editingNodeId.value);
    if (node) {
      node.text = editingText.value.trim() || '未命名节点';
    }
    scheduleAutoSave();
  }
  editingNodeId.value = null;
  editingText.value = '';
}

function cancelEditing() {
  editingNodeId.value = null;
  editingText.value = '';
}

function handleNodeMouseDown(event: MouseEvent, nodeId: string) {
  if (event.button !== 0) return;
  event.stopPropagation();
  
  const node = findNode(nodes.value, nodeId);
  if (!node) return;
  
  selectedNodeId.value = nodeId;
  isDragging.value = true;
  draggedNodeId.value = nodeId;
  
  const rect = (event.target as HTMLElement).closest('.mind-node')?.getBoundingClientRect();
  if (rect) {
    dragOffset.value = {
      x: event.clientX - rect.left,
      y: event.clientY - rect.top
    };
  }
  
  lastMousePos.value = { x: event.clientX, y: event.clientY };
}

function handleCanvasMouseDown(event: MouseEvent) {
  if (event.button !== 0) return;
  if ((event.target as HTMLElement).closest('.mind-node')) return;
  
  selectedNodeId.value = null;
  isPanning.value = true;
  lastMousePos.value = { x: event.clientX, y: event.clientY };
}

function handleMouseMove(event: MouseEvent) {
  if (isDragging.value && draggedNodeId.value) {
    const container = containerRef.value;
    if (!container) return;
    
    const rect = container.getBoundingClientRect();
    const x = (event.clientX - rect.left - panOffset.value.x) / scale.value - dragOffset.value.x;
    const y = (event.clientY - rect.top - panOffset.value.y) / scale.value - dragOffset.value.y;
    
    const node = findNode(nodes.value, draggedNodeId.value);
    if (node) {
      node.x = x;
      node.y = y;
    }
  }
  
  if (isPanning.value) {
    const dx = event.clientX - lastMousePos.value.x;
    const dy = event.clientY - lastMousePos.value.y;
    panOffset.value = {
      x: panOffset.value.x + dx,
      y: panOffset.value.y + dy
    };
    lastMousePos.value = { x: event.clientX, y: event.clientY };
  }
}

function handleMouseUp() {
  if (isDragging.value) {
    isDragging.value = false;
    if (draggedNodeId.value) {
      saveToHistory();
      scheduleAutoSave();
    }
    draggedNodeId.value = null;
  }
  isPanning.value = false;
}

function handleWheel(event: WheelEvent) {
  event.preventDefault();
  const delta = event.deltaY > 0 ? 0.9 : 1.1;
  const newScale = Math.min(Math.max(scale.value * delta, 0.25), 4);
  scale.value = newScale;
}

function handleKeyDown(event: KeyboardEvent) {
  if (editingNodeId.value) {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      finishEditing();
    } else if (event.key === 'Escape') {
      event.preventDefault();
      cancelEditing();
    }
    return;
  }
  
  if (event.ctrlKey || event.metaKey) {
    switch (event.key.toLowerCase()) {
      case 'z':
        event.preventDefault();
        if (event.shiftKey) {
          redo();
        } else {
          undo();
        }
        break;
      case 'y':
        event.preventDefault();
        redo();
        break;
      case 's':
        event.preventDefault();
        saveMindmap();
        break;
    }
    return;
  }
  
  switch (event.key) {
    case 'Delete':
    case 'Backspace':
      if (selectedNodeId.value) {
        event.preventDefault();
        deleteNode(selectedNodeId.value);
      }
      break;
    case 'Enter':
      if (selectedNodeId.value) {
        event.preventDefault();
        const node = findNode(nodes.value, selectedNodeId.value);
        if (node) {
          startEditing(selectedNodeId.value, node.text);
        }
      }
      break;
    case 'Tab':
      if (selectedNodeId.value) {
        event.preventDefault();
        addNode(selectedNodeId.value);
      }
      break;
    case 'Escape':
      selectedNodeId.value = null;
      break;
  }
}

function zoomIn() {
  scale.value = Math.min(scale.value * 1.2, 4);
}

function zoomOut() {
  scale.value = Math.max(scale.value * 0.8, 0.25);
}

function resetView() {
  scale.value = 1;
  panOffset.value = { x: 0, y: 0 };
}

function toggleCollapse(nodeId: string) {
  const node = findNode(nodes.value, nodeId);
  if (node) {
    node.collapsed = !node.collapsed;
  }
}

function scheduleAutoSave() {
  if (autoSaveTimer.value) {
    clearTimeout(autoSaveTimer.value);
  }
  autoSaveTimer.value = window.setTimeout(() => {
    saveMindmap();
  }, 30000);
}

async function saveMindmap() {
  if (isSaving.value) return;
  isSaving.value = true;
  
  try {
    const content = JSON.stringify(nodes.value);
    if (mindmapId.value) {
      await mindmapApi.updateMindmap(mindmapId.value, mindmapTitle.value, content);
    } else {
      const result = await mindmapApi.createMindmap(mindmapTitle.value);
      if (result.success && result.data !== null) {
        mindmapId.value = result.data;
        await mindmapApi.updateMindmap(mindmapId.value, mindmapTitle.value, content);
      }
    }
  } catch (error) {
    console.error('保存失败:', error);
  } finally {
    isSaving.value = false;
  }
}

async function exportMarkdown() {
  function nodesToMarkdown(nodeList: MindmapNode[], level: number = 0): string {
    let md = '';
    for (const node of nodeList) {
      const prefix = '#'.repeat(Math.min(level + 1, 6));
      md += `${prefix} ${node.text}\n\n`;
      if (node.children.length > 0 && !node.collapsed) {
        md += nodesToMarkdown(node.children, level + 1);
      }
    }
    return md;
  }
  
  const content = nodesToMarkdown(nodes.value);
  const blob = new Blob([content], { type: 'text/markdown' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = `${mindmapTitle.value}.md`;
  a.click();
  URL.revokeObjectURL(url);
}

function handleTitleChange(event: Event) {
  const target = event.target as HTMLInputElement;
  mindmapTitle.value = target.value || '未命名思维导图';
  scheduleAutoSave();
}

onMounted(async () => {
  document.addEventListener('keydown', handleKeyDown);
  document.addEventListener('mousemove', handleMouseMove);
  document.addEventListener('mouseup', handleMouseUp);
  
  if (nodes.value.length === 0) {
    saveToHistory();
  }
});

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeyDown);
  document.removeEventListener('mousemove', handleMouseMove);
  document.removeEventListener('mouseup', handleMouseUp);
  
  if (autoSaveTimer.value) {
    clearTimeout(autoSaveTimer.value);
  }
  
  if (nodes.value.length > 0 && mindmapId.value) {
    saveMindmap();
  }
});

watch(scale, (newVal) => {
  console.log('缩放比例:', newVal);
});
</script>

<template>
  <div class="flex h-full flex-col">
    <div class="mb-4 flex items-center justify-between">
      <div class="flex items-center gap-4">
        <input
          :value="mindmapTitle"
          class="text-xl font-semibold bg-transparent border-none outline-none text-slate-800 dark:text-slate-100 focus:ring-2 focus:ring-primary-500 rounded px-2 py-1"
          @change="handleTitleChange"
        />
      </div>
      <div class="flex items-center gap-2">
        <button
          class="p-2 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors disabled:opacity-50"
          :disabled="!canUndo"
          title="撤销 (Ctrl+Z)"
          @click="undo"
        >
          <Undo class="w-5 h-5 text-slate-600 dark:text-slate-300" />
        </button>
        <button
          class="p-2 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors disabled:opacity-50"
          :disabled="!canRedo"
          title="重做 (Ctrl+Y)"
          @click="redo"
        >
          <Redo class="w-5 h-5 text-slate-600 dark:text-slate-300" />
        </button>
        <div class="w-px h-6 bg-slate-300 dark:bg-slate-600 mx-1" />
        <button
          class="p-2 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
          title="缩小"
          @click="zoomOut"
        >
          <ZoomOut class="w-5 h-5 text-slate-600 dark:text-slate-300" />
        </button>
        <span class="text-sm text-slate-500 dark:text-slate-400 w-12 text-center">
          {{ Math.round(scale * 100) }}%
        </span>
        <button
          class="p-2 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
          title="放大"
          @click="zoomIn"
        >
          <ZoomIn class="w-5 h-5 text-slate-600 dark:text-slate-300" />
        </button>
        <button
          class="p-2 rounded-lg hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
          title="适应屏幕"
          @click="resetView"
        >
          <Maximize2 class="w-5 h-5 text-slate-600 dark:text-slate-300" />
        </button>
        <div class="w-px h-6 bg-slate-300 dark:bg-slate-600 mx-1" />
        <button
          class="inline-flex items-center gap-2 rounded-lg bg-primary-600 px-4 py-2 text-sm text-white hover:bg-primary-700 transition-colors disabled:opacity-50"
          :disabled="isSaving"
          @click="saveMindmap"
        >
          <Save class="w-4 h-4" />
          {{ isSaving ? '保存中...' : '保存' }}
        </button>
        <button
          class="inline-flex items-center gap-2 rounded-lg bg-gray-700 px-4 py-2 text-sm text-white hover:bg-gray-600 dark:bg-slate-600 dark:hover:bg-slate-500 transition-colors"
          @click="exportMarkdown"
        >
          <Download class="w-4 h-4" />
          导出
        </button>
      </div>
    </div>

    <div
      ref="containerRef"
      class="flex-1 overflow-hidden rounded-lg border border-gray-700 bg-gray-800 dark:border-slate-700 dark:bg-slate-900 relative"
      @mousedown="handleCanvasMouseDown"
      @wheel="handleWheel"
    >
      <div
        ref="canvas"
        class="absolute inset-0"
        :style="{
          transform: `translate(${panOffset.x}px, ${panOffset.y}px) scale(${scale})`,
          transformOrigin: '0 0'
        }"
      >
        <svg class="absolute inset-0 w-full h-full pointer-events-none" style="min-width: 3000px; min-height: 3000px;">
          <g v-for="node in nodes" :key="`lines-${node.id}`">
            <template v-if="node.children.length > 0 && !node.collapsed">
              <line
                v-for="child in node.children"
                :key="`line-${node.id}-${child.id}`"
                :x1="node.x + 96"
                :y1="node.y + 20"
                :x2="child.x"
                :y2="child.y + 20"
                stroke="rgba(148, 163, 184, 0.3)"
                stroke-width="2"
              />
              <g v-for="child in node.children" :key="`child-lines-${child.id}`">
                <template v-if="child.children.length > 0 && !child.collapsed">
                  <line
                    v-for="grandchild in child.children"
                    :key="`line-${child.id}-${grandchild.id}`"
                    :x1="child.x + 96"
                    :y1="child.y + 20"
                    :x2="grandchild.x"
                    :y2="grandchild.y + 20"
                    stroke="rgba(148, 163, 184, 0.3)"
                    stroke-width="2"
                  />
                </template>
              </g>
            </template>
          </g>
        </svg>

        <div
          v-for="node in nodes"
          :key="node.id"
          class="absolute"
          :style="{ left: `${node.x}px`, top: `${node.y}px` }"
        >
          <div
            class="mind-node flex flex-col items-center cursor-move select-none"
            :class="{
              'ring-2 ring-primary-500': selectedNodeId === node.id,
              'ring-2 ring-primary-300': draggedNodeId === node.id
            }"
            @mousedown="handleNodeMouseDown($event, node.id)"
            @dblclick="startEditing(node.id, node.text)"
          >
            <div
              class="min-w-24 max-w-48 rounded-lg px-4 py-2 text-center shadow-lg transition-all"
              :class="{
                'bg-gradient-to-br from-primary-500 to-primary-600 text-white': selectedNodeId === node.id,
                'bg-slate-700 text-slate-200 dark:bg-slate-800 dark:text-slate-100': selectedNodeId !== node.id
              }"
            >
              <input
                v-if="editingNodeId === node.id"
                v-model="editingText"
                class="w-full bg-transparent border-none outline-none text-center text-inherit"
                @blur="finishEditing"
                @keydown.stop
                autofocus
              />
              <span v-else class="block truncate">
                {{ node.text }}
              </span>
            </div>
            <div class="mt-2 flex gap-1">
              <button
                v-if="node.children.length > 0"
                class="rounded bg-slate-600 px-2 py-1 text-xs text-white hover:bg-slate-500 transition-colors"
                :title="node.collapsed ? '展开' : '折叠'"
                @click.stop="toggleCollapse(node.id)"
              >
                {{ node.collapsed ? '▼' : '▲' }}
              </button>
              <button
                class="rounded bg-green-600 px-2 py-1 text-xs text-white hover:bg-green-700 transition-colors"
                title="添加子节点 (Tab)"
                @click.stop="addNode(node.id)"
              >
                <Plus class="w-3 h-3" />
              </button>
              <button
                class="rounded bg-blue-600 px-2 py-1 text-xs text-white hover:bg-blue-700 transition-colors"
                title="编辑 (Enter)"
                @click.stop="startEditing(node.id, node.text)"
              >
                <Edit3 class="w-3 h-3" />
              </button>
              <button
                class="rounded bg-red-600 px-2 py-1 text-xs text-white hover:bg-red-700 transition-colors"
                title="删除 (Delete)"
                @click.stop="deleteNode(node.id)"
              >
                <Trash2 class="w-3 h-3" />
              </button>
            </div>
          </div>
        </div>
      </div>

      <div class="absolute bottom-4 right-4 flex flex-col gap-2">
        <div class="text-xs text-slate-500 dark:text-slate-400 text-right">
          {{ selectedNode ? `已选择: ${selectedNode.text}` : '点击节点选择' }}
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.mind-node {
  user-select: none;
}

.mind-node:active {
  cursor: grabbing;
}

input:focus {
  outline: none;
}
</style>
