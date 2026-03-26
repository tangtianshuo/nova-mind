import { invoke } from '@tauri-apps/api/core';
import type {
  CommandResult,
  HealthStatus,
  Mindmap,
  Skill,
  Conversation,
  Message,
  SandboxInfo,
} from '@/types';

export const mindmapApi = {
  async getMindmaps(): Promise<CommandResult<Mindmap[]>> {
    return invoke<CommandResult<Mindmap[]>>('get_mindmaps');
  },

  async createMindmap(title: string): Promise<CommandResult<number>> {
    return invoke<CommandResult<number>>('create_mindmap', { title });
  },

  async getMindmap(id: number): Promise<CommandResult<Mindmap>> {
    return invoke<CommandResult<Mindmap>>('get_mindmap', { id });
  },

  async updateMindmap(id: number, title: string, content: string): Promise<CommandResult<void>> {
    return invoke<CommandResult<void>>('update_mindmap', { id, title, content });
  },

  async deleteMindmap(id: number): Promise<CommandResult<void>> {
    return invoke<CommandResult<void>>('delete_mindmap', { id });
  },
};

export const skillApi = {
  async getSkills(category?: string): Promise<CommandResult<Skill[]>> {
    return invoke<CommandResult<Skill[]>>('get_skills', { category });
  },

  async getSkill(id: number): Promise<CommandResult<Skill>> {
    return invoke<CommandResult<Skill>>('get_skill', { id });
  },

  async createSkill(
    name: string,
    content: string,
    description?: string,
    category?: string
  ): Promise<CommandResult<number>> {
    return invoke<CommandResult<number>>('create_skill', {
      name,
      content,
      description,
      category,
    });
  },

  async updateSkill(
    id: number,
    name: string,
    content: string,
    description?: string,
    category?: string
  ): Promise<CommandResult<void>> {
    return invoke<CommandResult<void>>('update_skill', {
      id,
      name,
      content,
      description,
      category,
    });
  },

  async deleteSkill(id: number): Promise<CommandResult<void>> {
    return invoke<CommandResult<void>>('delete_skill', { id });
  },
};

export const conversationApi = {
  async getConversations(mindmapId?: number): Promise<CommandResult<Conversation[]>> {
    return invoke<CommandResult<Conversation[]>>('get_conversations', { mindmapId });
  },

  async getConversation(id: number): Promise<CommandResult<Conversation>> {
    return invoke<CommandResult<Conversation>>('get_conversation', { id });
  },

  async createConversation(mindmapId: number | null, title: string): Promise<CommandResult<number>> {
    return invoke<CommandResult<number>>('create_conversation', {
      mindmapId,
      title,
    });
  },

  async updateConversation(id: number, title: string): Promise<CommandResult<void>> {
    return invoke<CommandResult<void>>('update_conversation', { id, title });
  },

  async deleteConversation(id: number): Promise<CommandResult<void>> {
    return invoke<CommandResult<void>>('delete_conversation', { id });
  },
};

export const messageApi = {
  async getMessages(conversationId: number): Promise<CommandResult<Message[]>> {
    return invoke<CommandResult<Message[]>>('get_messages', { conversationId });
  },

  async createMessage(
    conversationId: number,
    role: 'user' | 'assistant' | 'system',
    content: string
  ): Promise<CommandResult<number>> {
    return invoke<CommandResult<number>>('create_message', {
      conversationId,
      role,
      content,
    });
  },

  async updateMessageContent(id: number, content: string): Promise<CommandResult<void>> {
    return invoke<CommandResult<void>>('update_message_content', { id, content });
  },

  async deleteMessage(id: number): Promise<CommandResult<void>> {
    return invoke<CommandResult<void>>('delete_message', { id });
  },
};

export const sandboxApi = {
  async initializeSandbox(downloadUrl?: string): Promise<CommandResult<void>> {
    return invoke<CommandResult<void>>('initialize_sandbox', { downloadUrl });
  },

  async startOpenclaw(port?: number): Promise<CommandResult<void>> {
    return invoke<CommandResult<void>>('start_openclaw', { port });
  },

  async stopOpenclaw(): Promise<CommandResult<void>> {
    return invoke<CommandResult<void>>('stop_openclaw');
  },

  async getSandboxStatus(): Promise<SandboxInfo> {
    return invoke<SandboxInfo>('get_sandbox_status');
  },

  async checkForUpdates(): Promise<CommandResult<string>> {
    return invoke<CommandResult<string>>('check_for_updates');
  },
};

export const systemApi = {
  async healthCheck(): Promise<HealthStatus> {
    return invoke<HealthStatus>('health_check');
  },
};
