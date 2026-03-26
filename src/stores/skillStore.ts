import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { Skill } from '@/types';
import { skillApi } from '@/api';

export const useSkillStore = defineStore('skill', () => {
  const skills = ref<Skill[]>([]);
  const currentSkill = ref<Skill | null>(null);
  const isLoading = ref(false);
  const error = ref<string | null>(null);
  const selectedCategory = ref<string | null>(null);

  const filteredSkills = computed(() => {
    if (!selectedCategory.value) {
      return skills.value;
    }
    return skills.value.filter(skill => skill.category === selectedCategory.value);
  });

  const categories = computed(() => {
    const cats = new Set<string>();
    skills.value.forEach(skill => {
      if (skill.category) {
        cats.add(skill.category);
      }
    });
    return Array.from(cats);
  });

  async function fetchSkills(category?: string) {
    isLoading.value = true;
    error.value = null;
    try {
      const result = await skillApi.getSkills(category);
      if (result.success && result.data) {
        skills.value = result.data;
      } else {
        error.value = result.error || '获取技能列表失败';
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : '获取技能列表失败';
    } finally {
      isLoading.value = false;
    }
  }

  async function fetchSkill(id: number) {
    isLoading.value = true;
    error.value = null;
    try {
      const result = await skillApi.getSkill(id);
      if (result.success && result.data) {
        currentSkill.value = result.data;
      } else {
        error.value = result.error || '获取技能详情失败';
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : '获取技能详情失败';
    } finally {
      isLoading.value = false;
    }
  }

  async function createSkill(
    name: string,
    content: string,
    description?: string,
    category?: string
  ) {
    isLoading.value = true;
    error.value = null;
    try {
      const result = await skillApi.createSkill(name, content, description, category);
      if (result.success && result.data !== null) {
        await fetchSkills();
        return result.data;
      } else {
        error.value = result.error || '创建技能失败';
        return null;
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : '创建技能失败';
      return null;
    } finally {
      isLoading.value = false;
    }
  }

  async function updateSkill(
    id: number,
    name: string,
    content: string,
    description?: string,
    category?: string
  ) {
    isLoading.value = true;
    error.value = null;
    try {
      const result = await skillApi.updateSkill(id, name, content, description, category);
      if (result.success) {
        await fetchSkills();
        return true;
      } else {
        error.value = result.error || '更新技能失败';
        return false;
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : '更新技能失败';
      return false;
    } finally {
      isLoading.value = false;
    }
  }

  async function deleteSkill(id: number) {
    isLoading.value = true;
    error.value = null;
    try {
      const result = await skillApi.deleteSkill(id);
      if (result.success) {
        skills.value = skills.value.filter(s => s.id !== id);
        if (currentSkill.value?.id === id) {
          currentSkill.value = null;
        }
        return true;
      } else {
        error.value = result.error || '删除技能失败';
        return false;
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : '删除技能失败';
      return false;
    } finally {
      isLoading.value = false;
    }
  }

  function setCategory(category: string | null) {
    selectedCategory.value = category;
  }

  function clearError() {
    error.value = null;
  }

  return {
    skills,
    currentSkill,
    isLoading,
    error,
    selectedCategory,
    filteredSkills,
    categories,
    fetchSkills,
    fetchSkill,
    createSkill,
    updateSkill,
    deleteSkill,
    setCategory,
    clearError,
  };
});
