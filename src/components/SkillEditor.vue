<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useSkillStore } from '@/stores/skillStore';
import type { Skill } from '@/types';

const props = defineProps<{ skill: Skill | null }>();
const emit = defineEmits<{ (e: 'close'): void; (e: 'save'): void }>();

const skillStore = useSkillStore();
const name = ref('');
const description = ref('');
const content = ref('');
const category = ref('');

const isEditing = computed(() => !!props.skill);
const title = computed(() => isEditing.value ? '编辑技能' : '新建技能');
const isValid = computed(() => name.value.trim() !== '' && content.value.trim() !== '');

onMounted(() => {
  if (props.skill) {
    name.value = props.skill.name;
    description.value = props.skill.description || '';
    content.value = props.skill.content;
    category.value = props.skill.category || '';
  }
});

async function handleSave() {
  if (!isValid.value) return;

  if (isEditing.value) {
    const success = await skillStore.updateSkill(
      props.skill!.id!,
      name.value.trim(),
      content.value.trim(),
      description.value.trim() || undefined,
      category.value.trim() || undefined
    );
    if (success) emit('save');
  } else {
    const id = await skillStore.createSkill(
      name.value.trim(),
      content.value.trim(),
      description.value.trim() || undefined,
      category.value.trim() || undefined
    );
    if (id) emit('save');
  }
}
</script>

<template>
  <Teleport to="body">
    <div
      class="fixed inset-0 z-50 flex items-center justify-center backdrop-blur-md"
      style="background: rgba(0,0,0,0.8)"
    >
      <div
        class="w-full max-w-2xl rounded-2xl border border-gray-700 p-6 shadow-glow"
        style="background: #18181B"
      >
        <div class="mb-6 flex items-center justify-between">
          <h3 class="text-lg font-medium text-white">
            {{ title }}
          </h3>
          <button
            class="text-gray-400 hover:text-white transition-colors"
            @click="emit('close')"
          >
            ✕
          </button>
        </div>
        <div class="space-y-4">
          <div>
            <label class="mb-2 block text-sm text-gray-400">技能名称 *</label>
            <input
              v-model="name"
              type="text"
              placeholder="例如：PRD生成器"
              class="input w-full"
            >
          </div>
          <div>
            <label class="mb-2 block text-sm text-gray-400">描述</label>
            <input
              v-model="description"
              type="text"
              placeholder="简要描述"
              class="input w-full"
            >
          </div>
          <div>
            <label class="mb-2 block text-sm text-gray-400">分类</label>
            <input
              v-model="category"
              type="text"
              placeholder="文档、需求、竞品分析"
              class="input w-full"
            >
          </div>
          <div>
            <label class="mb-2 block text-sm text-gray-400">技能内容 *</label>
            <textarea
              v-model="content"
              rows="8"
              placeholder="输入技能内容..."
              class="input w-full font-mono"
            />
          </div>
        </div>
        <div
          v-if="skillStore.error"
          class="mt-4 rounded-lg p-3 text-sm"
          style="background: rgba(239,68,68,0.1); color: #FCA5A5"
        >
          错误: {{ skillStore.error }}
        </div>
        <div class="mt-6 flex justify-end gap-3">
          <button
            class="btn btn-secondary"
            @click="emit('close')"
          >
            取消
          </button>
          <button
            :disabled="!isValid || skillStore.isLoading"
            class="btn btn-primary"
            @click="handleSave"
          >
            {{ skillStore.isLoading ? '保存中...' : '保存' }}
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>
