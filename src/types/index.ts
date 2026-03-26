export interface CommandResult<T> {
  success: boolean;
  data: T | null;
  error: string | null;
}

export interface HealthStatus {
  status: 'healthy' | 'degraded' | 'unhealthy';
  components: {
    frontend: boolean;
    backend: boolean;
    sandbox: boolean;
    database: boolean;
  };
  version: string;
  timestamp: string;
}

export interface Mindmap {
  id?: number;
  title: string;
  content: string;
  created_at: string;
  updated_at: string;
}

export type VariableType = 'text' | 'textarea' | 'select' | 'number' | 'date' | 'checkbox';

export interface VariableValidation {
  minLength?: number;
  maxLength?: number;
  min?: number;
  max?: number;
  pattern?: string;
  message?: string;
}

export interface SelectOption {
  label: string;
  value: string;
}

export interface SkillVariable {
  name: string;
  type: VariableType;
  label: string;
  required?: boolean;
  placeholder?: string;
  default?: string | number | boolean;
  options?: SelectOption[];
  validation?: VariableValidation;
}

export interface Skill {
  id?: number;
  name: string;
  description?: string;
  content: string;
  category?: string;
  variables?: SkillVariable[];
  version: string;
  created_at: string;
  updated_at: string;
}

export type VariableFormValues = Record<string, string | number | boolean>;

export interface Conversation {
  id?: number;
  mindmap_id?: number;
  title: string;
  created_at: string;
  updated_at: string;
}

export interface Message {
  id?: number;
  conversation_id: number;
  role: 'user' | 'assistant' | 'system';
  content: string;
  created_at: string;
}

export interface CreateSkillRequest {
  name: string;
  description?: string;
  content: string;
  category?: string;
}

export interface UpdateSkillRequest {
  id: number;
  name: string;
  description?: string;
  content: string;
  category?: string;
}

export interface CreateConversationRequest {
  mindmap_id?: number;
  title: string;
}

export interface CreateMessageRequest {
  conversation_id: number;
  role: 'user' | 'assistant' | 'system';
  content: string;
}

export interface SandboxInfo {
  status: 'uninitialized' | 'downloading' | 'ready' | 'running' | 'error';
  version: string;
  path: string;
  port?: number;
}
