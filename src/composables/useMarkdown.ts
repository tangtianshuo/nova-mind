import { marked } from 'marked';

marked.setOptions({
  gfm: true,
  breaks: true,
});

export function useMarkdown() {
  function renderMarkdown(content: string): string {
    try {
      return marked.parse(content) as string;
    } catch (error) {
      console.error('Markdown 解析错误:', error);
      return content;
    }
  }

  function extractCodeBlocks(content: string): string[] {
    const codeBlockRegex = /```(\w+)?\n([\s\S]*?)```/g;
    const matches: string[] = [];
    let match;

    while ((match = codeBlockRegex.exec(content)) !== null) {
      matches.push(match[2].trim());
    }

    return matches;
  }

  function extractHeadings(content: string): { level: number; text: string }[] {
    const headingRegex = /^(#{1,6})\s+(.+)$/gm;
    const headings: { level: number; text: string }[] = [];
    let match;

    while ((match = headingRegex.exec(content)) !== null) {
      headings.push({
        level: match[1].length,
        text: match[2].trim(),
      });
    }

    return headings;
  }

  function setupCopyButtons(container: HTMLElement) {
    const copyButtons = container.querySelectorAll('.copy-btn');

    copyButtons.forEach((btn) => {
      btn.addEventListener('click', async () => {
        const code = decodeURIComponent(btn.getAttribute('data-code') || '');

        try {
          await navigator.clipboard.writeText(code);
          const originalText = btn.textContent;
          btn.textContent = '已复制!';
          setTimeout(() => {
            btn.textContent = originalText;
          }, 2000);
        } catch (err) {
          console.error('复制失败:', err);
        }
      });
    });
  }

  return {
    renderMarkdown,
    extractCodeBlocks,
    extractHeadings,
    setupCopyButtons,
  };
}
