import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { Conversation, Message } from '@/types';
import { conversationApi, messageApi } from '@/api';

export const useConversationStore = defineStore('conversation', () => {
  const conversations = ref<Conversation[]>([]);
  const currentConversation = ref<Conversation | null>(null);
  const messages = ref<Message[]>([]);
  const isLoading = ref(false);
  const error = ref<string | null>(null);

  const sortedMessages = computed(() => {
    return [...messages.value].sort((a, b) =>
      new Date(a.created_at).getTime() - new Date(b.created_at).getTime()
    );
  });

  async function fetchConversations(mindmapId?: number) {
    isLoading.value = true;
    error.value = null;
    try {
      const result = await conversationApi.getConversations(mindmapId);
      if (result.success && result.data) {
        conversations.value = result.data;
      } else {
        error.value = result.error || '获取会话列表失败';
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : '获取会话列表失败';
    } finally {
      isLoading.value = false;
    }
  }

  async function fetchConversation(id: number) {
    isLoading.value = true;
    error.value = null;
    try {
      const result = await conversationApi.getConversation(id);
      if (result.success && result.data) {
        currentConversation.value = result.data;
        await fetchMessages(id);
      } else {
        error.value = result.error || '获取会话详情失败';
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : '获取会话详情失败';
    } finally {
      isLoading.value = false;
    }
  }

  async function fetchMessages(conversationId: number) {
    try {
      const result = await messageApi.getMessages(conversationId);
      if (result.success && result.data) {
        messages.value = result.data;
      } else {
        error.value = result.error || '获取消息列表失败';
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : '获取消息列表失败';
    }
  }

  async function createConversation(mindmapId: number | null, title: string) {
    isLoading.value = true;
    error.value = null;
    try {
      const result = await conversationApi.createConversation(mindmapId, title);
      if (result.success && result.data !== null) {
        await fetchConversations();
        return result.data;
      } else {
        error.value = result.error || '创建会话失败';
        return null;
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : '创建会话失败';
      return null;
    } finally {
      isLoading.value = false;
    }
  }

  async function updateConversation(id: number, title: string) {
    isLoading.value = true;
    error.value = null;
    try {
      const result = await conversationApi.updateConversation(id, title);
      if (result.success) {
        await fetchConversations();
        if (currentConversation.value?.id === id) {
          currentConversation.value.title = title;
        }
        return true;
      } else {
        error.value = result.error || '更新会话失败';
        return false;
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : '更新会话失败';
      return false;
    } finally {
      isLoading.value = false;
    }
  }

  async function deleteConversation(id: number) {
    isLoading.value = true;
    error.value = null;
    try {
      const result = await conversationApi.deleteConversation(id);
      if (result.success) {
        conversations.value = conversations.value.filter(c => c.id !== id);
        if (currentConversation.value?.id === id) {
          currentConversation.value = null;
          messages.value = [];
        }
        return true;
      } else {
        error.value = result.error || '删除会话失败';
        return false;
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : '删除会话失败';
      return false;
    } finally {
      isLoading.value = false;
    }
  }

  async function sendMessage(conversationId: number, content: string, role: 'user' | 'assistant' | 'system' = 'user') {
    try {
      const result = await messageApi.createMessage(conversationId, role, content);
      if (result.success && result.data !== null) {
        const newMessage: Message = {
          id: result.data,
          conversation_id: conversationId,
          role,
          content,
          created_at: new Date().toISOString(),
        };
        messages.value.push(newMessage);
        return newMessage;
      } else {
        error.value = result.error || '发送消息失败';
        return null;
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : '发送消息失败';
      return null;
    }
  }

  async function updateMessage(id: number, content: string) {
    try {
      const result = await messageApi.updateMessageContent(id, content);
      if (result.success) {
        const msg = messages.value.find(m => m.id === id);
        if (msg) {
          msg.content = content;
        }
        return true;
      } else {
        error.value = result.error || '更新消息失败';
        return false;
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : '更新消息失败';
      return false;
    }
  }

  async function deleteMessage(id: number) {
    try {
      const result = await messageApi.deleteMessage(id);
      if (result.success) {
        messages.value = messages.value.filter(m => m.id !== id);
        return true;
      } else {
        error.value = result.error || '删除消息失败';
        return false;
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : '删除消息失败';
      return false;
    }
  }

  function selectConversation(conversation: Conversation | null) {
    currentConversation.value = conversation;
    if (conversation) {
      messages.value = [];
    }
  }

  function clearError() {
    error.value = null;
  }

  return {
    conversations,
    currentConversation,
    messages,
    isLoading,
    error,
    sortedMessages,
    fetchConversations,
    fetchConversation,
    fetchMessages,
    createConversation,
    updateConversation,
    deleteConversation,
    sendMessage,
    updateMessage,
    deleteMessage,
    selectConversation,
    clearError,
  };
});
