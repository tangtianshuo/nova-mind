import { describe, it, expect } from 'vitest';

describe('MindMap Component Logic', () => {
  describe('Node Structure', () => {
    interface MindMapNode {
      id: number;
      text: string;
      x: number;
      y: number;
      children: MindMapNode[];
    }

    const createNode = (text: string, x: number, y: number): MindMapNode => ({
      id: Date.now(),
      text,
      x,
      y,
      children: [],
    });

    it('should create a node with correct properties', () => {
      const node = createNode('测试节点', 100, 200);
      expect(node).toHaveProperty('id');
      expect(node).toHaveProperty('text', '测试节点');
      expect(node).toHaveProperty('x', 100);
      expect(node).toHaveProperty('y', 200);
      expect(node).toHaveProperty('children');
      expect(Array.isArray(node.children)).toBe(true);
    });

    it('should add child node to parent', () => {
      const parent = createNode('父节点', 100, 100);
      const child = createNode('子节点', 250, 150);

      parent.children.push(child);

      expect(parent.children).toHaveLength(1);
      expect(parent.children[0].text).toBe('子节点');
    });
  });

  describe('Node Operations', () => {
    interface MindMapNode {
      id: number;
      text: string;
      x: number;
      y: number;
      children: MindMapNode[];
    }

    const deleteNodeById = (nodes: MindMapNode[], id: number): MindMapNode[] => {
      const deleteRecursive = (items: MindMapNode[]): MindMapNode[] => {
        return items.filter(item => {
          if (item.id === id) return false;
          if (item.children) {
            item.children = deleteRecursive(item.children);
          }
          return true;
        });
      };
      return deleteRecursive(nodes);
    };

    it('should delete node by id', () => {
      const nodes: MindMapNode[] = [
        {
          id: 1,
          text: '节点1',
          x: 100,
          y: 100,
          children: [
            {
              id: 2,
              text: '节点2',
              x: 200,
              y: 100,
              children: [],
            },
          ],
        },
      ];

      const result = deleteNodeById(nodes, 2);
      expect(result[0].children).toHaveLength(0);
    });

    it('should not affect other nodes when deleting', () => {
      const nodes: MindMapNode[] = [
        {
          id: 1,
          text: '节点1',
          x: 100,
          y: 100,
          children: [],
        },
        {
          id: 2,
          text: '节点2',
          x: 200,
          y: 100,
          children: [],
        },
      ];

      const result = deleteNodeById(nodes, 1);
      expect(result).toHaveLength(1);
      expect(result[0].id).toBe(2);
    });
  });
});
