<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useSkillStore } from '@/stores/skillStore';
import SkillCard from '@/components/SkillCard.vue';
import SkillEditor from '@/components/SkillEditor.vue';
import { Plus, Layers, Filter } from 'lucide-vue-next';

const skillStore = useSkillStore();
const showEditor = ref(false);
const editingSkill = ref<any>(null);

onMounted(() => {
  skillStore.fetchSkills();
});

function handleNewSkill() {
  editingSkill.value = null;
  showEditor.value = true;
}

function handleEditSkill(skill: any) {
  editingSkill.value = skill;
  showEditor.value = true;
}

async function handleDeleteSkill(id: number) {
  if (confirm('确定要删除这个技能吗？')) {
    await skillStore.deleteSkill(id);
  }
}

function handleEditorClose() {
  showEditor.value = false;
  editingSkill.value = null;
}

async function handleEditorSave() {
  await skillStore.fetchSkills();
  handleEditorClose();
}
</script>

<template>
  <div class="flex h-full flex-col max-w-6xl mx-auto">
    <div class="mb-6 flex items-center justify-between">
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-primary-500 to-primary-600 flex items-center justify-center shadow-lg shadow-primary-500/25">
          <Layers class="w-5 h-5 text-white" />
        </div>
        <div>
          <h2 class="text-xl font-bold text-slate-800 dark:text-slate-100">
            技能管理
          </h2>
          <p class="text-sm text-slate-500 dark:text-slate-400">
            {{ skillStore.filteredSkills.length }} 个技能
          </p>
        </div>
      </div>
      
      <button
        class="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl
               bg-gradient-to-r from-primary-500 to-primary-600
               text-sm font-medium text-white
               shadow-lg shadow-primary-500/25
               transition-all duration-200
               hover:from-primary-400 hover:to-primary-500
               hover:shadow-xl hover:shadow-primary-500/30
               active:from-primary-600 active:to-primary-700"
        @click="handleNewSkill"
      >
        <Plus class="w-4 h-4" />
        新建技能
      </button>
    </div>

    <div
      v-if="skillStore.selectedCategory || skillStore.categories.length > 0"
      class="mb-6 flex flex-wrap gap-2"
    >
      <button
        :class="[
          'inline-flex items-center gap-1.5 rounded-full px-4 py-1.5 text-sm font-medium transition-all duration-200',
          !skillStore.selectedCategory
            ? 'bg-gradient-to-r from-primary-500 to-primary-600 text-white shadow-md'
            : 'bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400 hover:bg-slate-200 dark:hover:bg-slate-700'
        ]"
        @click="skillStore.setCategory(null)"
      >
        全部
      </button>
      <button
        v-for="cat in skillStore.categories"
        :key="cat"
        :class="[
          'inline-flex items-center gap-1.5 rounded-full px-4 py-1.5 text-sm font-medium transition-all duration-200',
          skillStore.selectedCategory === cat
            ? 'bg-gradient-to-r from-primary-500 to-primary-600 text-white shadow-md'
            : 'bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-400 hover:bg-slate-200 dark:hover:bg-slate-700'
        ]"
        @click="skillStore.setCategory(cat)"
      >
        {{ cat }}
      </button>
    </div>

    <div
      v-if="skillStore.isLoading"
      class="flex flex-1 items-center justify-center"
    >
      <div class="h-8 w-8 animate-spin rounded-full border-2 border-primary-500 border-t-transparent" />
    </div>

    <div
      v-else-if="skillStore.error"
      class="flex flex-1 items-center justify-center"
    >
      <div class="text-center">
        <div class="w-16 h-16 rounded-2xl bg-red-100 dark:bg-red-900/20 flex items-center justify-center mx-auto mb-4">
          <Filter class="w-8 h-8 text-red-500" />
        </div>
        <p class="text-red-500 mb-4">
          {{ skillStore.error }}
        </p>
        <button
          class="text-primary-500 hover:text-primary-600 font-medium"
          @click="skillStore.fetchSkills()"
        >
          重试
        </button>
      </div>
    </div>

    <div
      v-else-if="skillStore.filteredSkills.length === 0"
      class="flex flex-1 items-center justify-center"
    >
      <div class="text-center">
        <div class="w-16 h-16 rounded-2xl bg-slate-100 dark:bg-slate-800 flex items-center justify-center mx-auto mb-4">
          <Layers class="w-8 h-8 text-slate-400" />
        </div>
        <p class="text-slate-500 dark:text-slate-400 mb-4">
          暂无技能
        </p>
        <button
          class="text-primary-500 hover:text-primary-600 font-medium"
          @click="handleNewSkill"
        >
          创建第一个技能
        </button>
      </div>
    </div>

    <div
      v-else
      class="flex-1 overflow-auto"
    >
      <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        <SkillCard
          v-for="skill in skillStore.filteredSkills"
          :key="skill.id"
          :skill="skill"
          @edit="handleEditSkill"
          @delete="handleDeleteSkill"
        />
      </div>
    </div>

    <SkillEditor
      v-if="showEditor"
      :skill="editingSkill"
      @close="handleEditorClose"
      @save="handleEditorSave"
    />
  </div>
</template>
