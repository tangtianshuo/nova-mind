import { describe, it, expect, vi, beforeEach } from 'vitest';
import { setActivePinia, createPinia } from 'pinia';
import { useConversationStore } from '@/stores/conversationStore';
import * as api from '@/api';

vi.mock('@/api', () => ({
  conversationApi: {
    getConversations: vi.fn(),
    getConversation: vi.fn(),
    createConversation: vi.fn(),
    updateConversation: vi.fn(),
    deleteConversation: vi.fn(),
  },
  messageApi: {
    getMessages: vi.fn(),
    createMessage: vi.fn(),
    updateMessageContent: vi.fn(),
    deleteMessage: vi.fn(),
  },
}));

describe('ConversationStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.clearAllMocks();
  });

  it('should have empty initial state', () => {
    const store = useConversationStore();
    expect(store.conversations).toEqual([]);
    expect(store.currentConversation).toBeNull();
    expect(store.messages).toEqual([]);
    expect(store.isLoading).toBe(false);
    expect(store.error).toBeNull();
  });

  it('should fetch conversations successfully', async () => {
    const mockConversations = [
      { id: 1, title: 'Conv 1' },
      { id: 2, title: 'Conv 2' },
    ];

    (api.conversationApi.getConversations as any).mockResolvedValue({
      success: true,
      data: mockConversations,
    });

    const store = useConversationStore();
    await store.fetchConversations();

    expect(store.conversations).toEqual(mockConversations);
  });

  it('should create conversation and switch to it', async () => {
    (api.conversationApi.createConversation as any).mockResolvedValue({
      success: true,
      data: 1,
    });
    (api.conversationApi.getConversations as any).mockResolvedValue({
      success: true,
      data: [{ id: 1, title: 'New Conv' }],
    });

    const store = useConversationStore();
    const id = await store.createConversation(null, 'New Conv');

    expect(id).toBe(1);
  });

  it('should delete conversation and clear if current', async () => {
    (api.conversationApi.deleteConversation as any).mockResolvedValue({
      success: true,
    });

    const store = useConversationStore();
    store.conversations = [{ id: 1, title: 'Conv 1' }];
    store.currentConversation = { id: 1, title: 'Conv 1' };

    const success = await store.deleteConversation(1);

    expect(success).toBe(true);
    expect(store.conversations).toHaveLength(0);
    expect(store.currentConversation).toBeNull();
  });

  it('should send message and add to messages', async () => {
    (api.messageApi.createMessage as any).mockResolvedValue({
      success: true,
      data: 1,
    });

    const store = useConversationStore();
    store.currentConversation = { id: 1, title: 'Conv 1' };

    const msg = await store.sendMessage(1, 'Hello', 'user');

    expect(msg).not.toBeNull();
    expect(store.messages).toHaveLength(1);
    expect(store.messages[0].content).toBe('Hello');
  });

  it('should sort messages by created_at', () => {
    const store = useConversationStore();
    store.messages = [
      { id: 2, conversation_id: 1, role: 'user', content: 'Second', created_at: '2024-01-02T00:00:00Z' },
      { id: 1, conversation_id: 1, role: 'user', content: 'First', created_at: '2024-01-01T00:00:00Z' },
    ];

    const sorted = store.sortedMessages;

    expect(sorted[0].id).toBe(1);
    expect(sorted[1].id).toBe(2);
  });
});
