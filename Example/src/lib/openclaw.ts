/**
 * OpenClaw 沙箱管理器 TypeScript API
 */

import { invoke } from '@tauri-apps/api/core';
import { listen, Event } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/plugin-shell';

export interface SandboxStatus {
  installed: boolean;
  version: string | null;
  path: string | null;
  running: boolean;
  pid: number | null;
  port: number;
}

export interface OpenClawConfig {
  channels?: {
    whatsapp?: {
      allowFrom?: string[];
      groups?: Record<string, { requireMention: boolean }>;
    };
    telegram?: {
      token?: string;
      allowedUsers?: string[];
    };
  };
  messages?: {
    groupChat?: {
      mentionPatterns?: string[];
    };
  };
  plugins?: string[];
}

export interface LogEntry {
  timestamp: number;
  level: 'INFO' | 'ERROR' | 'WARN';
  message: string;
}

export class OpenClawManager {
  private static instance: OpenClawManager;
  private statusListeners = new Set<(status: SandboxStatus) => void>();
  private logListeners = new Set<(log: LogEntry) => void>();
  private unlisteners: (() => void)[] = [];

  static getInstance(): OpenClawManager {
    if (!OpenClawManager.instance) {
      OpenClawManager.instance = new OpenClawManager();
    }
    return OpenClawManager.instance;
  }

  async initialize(): Promise<SandboxStatus> {
    return await invoke('initialize_sandbox');
  }

  async start(port?: number, config?: OpenClawConfig): Promise<SandboxStatus> {
    return await invoke('start_openclaw', { port, config });
  }

  async stop(): Promise<void> {
    return await invoke('stop_openclaw');
  }

  async getStatus(): Promise<SandboxStatus> {
    return await invoke('get_sandbox_status');
  }

  async checkUpdate(): Promise<string | null> {
    return await invoke('check_for_updates');
  }

  async openDashboard(port: number = 18789): Promise<void> {
    await open(`http://127.0.0.1:${port}`);
  }

  onStatusChange(callback: (status: SandboxStatus) => void): () => void {
    this.statusListeners.add(callback);
    return () => this.statusListeners.delete(callback);
  }

  onLog(callback: (log: LogEntry) => void): () => void {
    this.logListeners.add(callback);
    return () => this.logListeners.delete(callback);
  }

  dispose(): void {
    this.unlisteners.forEach(u => u());
    this.unlisteners = [];
    this.statusListeners.clear();
    this.logListeners.clear();
  }
}
