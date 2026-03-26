import { marked } from 'marked';
import hljs from 'highlight.js';

const defaultOptions = {
  gfm: true,
  breaks: true,
};

marked.setOptions({
  gfm: true,
  breaks: true,
});

const renderer = new marked.Renderer();

renderer.code = function(code: string, infostring: string | undefined, escaped: boolean) {
  const lang = infostring || '';
  if (!lang || !hljs.getLanguage(lang)) {
    return `<pre><code>${escapeHtml(text)}</code></pre>`;
  }
  return `<pre class="hljs"><code class="language-${lang}">${highlight(text, lang)}</code></pre>`;
};

renderer.table = function({ header, body }: { header: string; body: string }) {
  return `
    <div class="overflow-x-auto">
      <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
        <thead class="bg-gray-50 dark:bg-gray-800">${header}</thead>
        <tbody class="divide-y divide-gray-200 dark:divide-gray-700">${body}</tbody>
      </table>
    </div>
  `;
};

marked.use({ renderer });

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
