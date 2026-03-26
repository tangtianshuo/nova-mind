import { describe, it, expect, vi, beforeEach } from 'vitest';
import { setActivePinia, createPinia } from 'pinia';
import { useSkillStore } from '@/stores/skillStore';
import * as api from '@/api';

vi.mock('@/api', () => ({
  skillApi: {
    getSkills: vi.fn(),
    getSkill: vi.fn(),
    createSkill: vi.fn(),
    updateSkill: vi.fn(),
    deleteSkill: vi.fn(),
  },
}));

describe('SkillStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.clearAllMocks();
  });

  it('should have empty initial state', () => {
    const store = useSkillStore();
    expect(store.skills).toEqual([]);
    expect(store.currentSkill).toBeNull();
    expect(store.isLoading).toBe(false);
    expect(store.error).toBeNull();
  });

  it('should fetch skills successfully', async () => {
    const mockSkills = [
      { id: 1, name: 'Skill 1', content: 'Content 1', version: '1.0.0' },
      { id: 2, name: 'Skill 2', content: 'Content 2', version: '1.0.0' },
    ];

    (api.skillApi.getSkills as any).mockResolvedValue({
      success: true,
      data: mockSkills,
    });

    const store = useSkillStore();
    await store.fetchSkills();

    expect(store.skills).toEqual(mockSkills);
    expect(store.isLoading).toBe(false);
    expect(store.error).toBeNull();
  });

  it('should handle fetch skills error', async () => {
    (api.skillApi.getSkills as any).mockResolvedValue({
      success: false,
      error: '获取失败',
    });

    const store = useSkillStore();
    await store.fetchSkills();

    expect(store.skills).toEqual([]);
    expect(store.error).toBe('获取失败');
  });

  it('should create skill successfully', async () => {
    (api.skillApi.createSkill as any).mockResolvedValue({
      success: true,
      data: 1,
    });
    (api.skillApi.getSkills as any).mockResolvedValue({
      success: true,
      data: [{ id: 1, name: 'New Skill', content: 'Content' }],
    });

    const store = useSkillStore();
    const id = await store.createSkill('New Skill', 'Content');

    expect(id).toBe(1);
  });

  it('should update skill successfully', async () => {
    (api.skillApi.updateSkill as any).mockResolvedValue({
      success: true,
    });
    (api.skillApi.getSkills as any).mockResolvedValue({
      success: true,
      data: [{ id: 1, name: 'Updated', content: 'Content' }],
    });

    const store = useSkillStore();
    const success = await store.updateSkill(1, 'Updated', 'Content');

    expect(success).toBe(true);
  });

  it('should delete skill successfully', async () => {
    (api.skillApi.deleteSkill as any).mockResolvedValue({
      success: true,
    });

    const store = useSkillStore();
    store.skills = [{ id: 1, name: 'Skill', content: 'Content' }];
    const success = await store.deleteSkill(1);

    expect(success).toBe(true);
    expect(store.skills).toHaveLength(0);
  });

  it('should filter skills by category', () => {
    const store = useSkillStore();
    store.skills = [
      { id: 1, name: 'Skill 1', category: 'A', content: '' },
      { id: 2, name: 'Skill 2', category: 'B', content: '' },
      { id: 3, name: 'Skill 3', category: 'A', content: '' },
    ];

    store.setCategory('A');
    expect(store.filteredSkills).toHaveLength(2);
    expect(store.filteredSkills.every(s => s.category === 'A')).toBe(true);
  });

  it('should extract categories from skills', () => {
    const store = useSkillStore();
    store.skills = [
      { id: 1, name: 'Skill 1', category: 'A', content: '' },
      { id: 2, name: 'Skill 2', category: 'B', content: '' },
      { id: 3, name: 'Skill 3', category: 'A', content: '' },
    ];

    expect(store.categories).toContain('A');
    expect(store.categories).toContain('B');
    expect(store.categories).toHaveLength(2);
  });
});
