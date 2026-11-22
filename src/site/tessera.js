/**
 * Tessera Progressive Enhancement Micro-Script (~4.5KB)
 * Pure vanilla JavaScript - Zero dependencies
 */
(function () {
  'use strict';

  // Global namespace for programmatic APIs
  window.tessera = window.tessera || {};

  // 1. Theme Management (Dark / Light Mode)
  function initTheme() {
    const savedTheme = localStorage.getItem('ts-theme');
    const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    const initialTheme = savedTheme || (systemPrefersDark ? 'dark' : 'light');
    document.documentElement.setAttribute('data-theme', initialTheme);

    document.querySelectorAll('[data-ts-theme-toggle]').forEach(button => {
      button.addEventListener('click', () => {
        toggleTheme();
      });
    });
  }

  function toggleTheme() {
    const current = document.documentElement.getAttribute('data-theme') || 'light';
    const next = current === 'dark' ? 'light' : 'dark';
    document.documentElement.setAttribute('data-theme', next);
    localStorage.setItem('ts-theme', next);
    if (window.tessera.toast) {
      window.tessera.toast(`Switched to ${next} theme`, 'info');
    }
  }
  window.tessera.toggleTheme = toggleTheme;

  // 2. Toast Notification System
  function initToasts() {
    let container = document.getElementById('ts-toast-container');
    if (!container) {
      container = document.createElement('div');
      container.id = 'ts-toast-container';
      container.className = 'ts-toast-container';
      document.body.appendChild(container);
    }

    window.tessera.toast = function (message, type = 'info', duration = 3000) {
      const toast = document.createElement('div');
      toast.className = `ts-toast ts-toast--${type}`;
      const icon = type === 'success' ? '✓' : type === 'error' ? '✗' : 'ℹ';
      toast.innerHTML = `<span class="ts-toast__icon">${icon}</span><span class="ts-toast__msg">${message}</span>`;
      container.appendChild(toast);

      // Trigger enter animation
      requestAnimationFrame(() => toast.classList.add('ts-toast--visible'));

      setTimeout(() => {
        toast.classList.remove('ts-toast--visible');
        setTimeout(() => toast.remove(), 250);
      }, duration);
    };
  }

  // 3. Client-Side Syntax Highlighting (Zero-dependency tokenization)
  function initSyntaxHighlighting() {
    const KEYWORDS = new Set([
      'fn', 'let', 'mut', 'pub', 'struct', 'enum', 'impl', 'use', 'mod', 'match', 'if', 'else',
      'for', 'in', 'while', 'loop', 'return', 'break', 'continue', 'as', 'const', 'static', 'async',
      'await', 'trait', 'type', 'where', 'ref', 'move', 'true', 'false', 'function', 'var', 'const',
      'import', 'export', 'default', 'class', 'extends', 'new', 'this', 'super', 'null', 'undefined',
      'def', 'from', 'elif', 'self', 'None'
    ]);

    document.querySelectorAll('pre code').forEach(codeBlock => {
      if (codeBlock.dataset.highlighted) return;
      codeBlock.dataset.highlighted = 'true';
      const raw = codeBlock.innerText;

      // Tokenizer regex
      const tokenRegex = /(\/\/[^\n]*|\/\*[\s\S]*?\*\/|#[^\n]*)|("(?:\\.|[^"\\])*"|'[^']*')|(\b\d+(?:\.\d+)?\b)|(\b[a-zA-Z_]\w*\b)|([{}()[\],;])/g;

      let highlighted = '';
      let lastIdx = 0;
      let match;

      while ((match = tokenRegex.exec(raw)) !== null) {
        // Append text between matches (escaped)
        highlighted += escapeHtml(raw.slice(lastIdx, match.index));
        lastIdx = match.index + match[0].length;

        const [full, comment, string, number, word, punct] = match;
        if (comment) {
          highlighted += `<span class="ts-hl-comment">${escapeHtml(comment)}</span>`;
        } else if (string) {
          highlighted += `<span class="ts-hl-string">${escapeHtml(string)}</span>`;
        } else if (number) {
          highlighted += `<span class="ts-hl-number">${escapeHtml(number)}</span>`;
        } else if (word) {
          if (KEYWORDS.has(word)) {
            highlighted += `<span class="ts-hl-keyword">${escapeHtml(word)}</span>`;
          } else {
            // Check if followed by open parenthesis (function call)
            const rest = raw.slice(lastIdx).trimStart();
            if (rest.startsWith('(')) {
              highlighted += `<span class="ts-hl-fn">${escapeHtml(word)}</span>`;
            } else {
              highlighted += escapeHtml(word);
            }
          }
        } else if (punct) {
          highlighted += `<span class="ts-hl-punct">${escapeHtml(punct)}</span>`;
        }
      }
      highlighted += escapeHtml(raw.slice(lastIdx));
      codeBlock.innerHTML = highlighted;
    });
  }

  function escapeHtml(str) {
    return str
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;');
  }

  // 4. Interactive Table Sorting
  function initTableSorting() {
    document.querySelectorAll('table').forEach(table => {
      const headers = table.querySelectorAll('th');
      headers.forEach((th, colIdx) => {
        th.classList.add('ts-sortable');
        th.setAttribute('title', 'Click to sort');
        let asc = true;

        th.addEventListener('click', () => {
          const tbody = table.querySelector('tbody') || table;
          const rows = Array.from(tbody.querySelectorAll('tr')).filter(r => r.querySelector('td'));

          rows.sort((rowA, rowB) => {
            const cellA = (rowA.children[colIdx]?.innerText || '').trim();
            const cellB = (rowB.children[colIdx]?.innerText || '').trim();

            const numA = parseFloat(cellA.replace(/[^0-9.-]+/g, ''));
            const numB = parseFloat(cellB.replace(/[^0-9.-]+/g, ''));

            if (!isNaN(numA) && !isNaN(numB)) {
              return asc ? numA - numB : numB - numA;
            }
            return asc ? cellA.localeCompare(cellB) : cellB.localeCompare(cellA);
          });

          // Re-append rows in sorted order
          rows.forEach(r => tbody.appendChild(r));

          // Update header indicators
          headers.forEach(h => h.classList.remove('ts-sort-asc', 'ts-sort-desc'));
          th.classList.add(asc ? 'ts-sort-asc' : 'ts-sort-desc');
          asc = !asc;
        });
      });
    });
  }

  // 5. Heading Anchor Links with One-Click Copy
  function initHeadingAnchors() {
    document.querySelectorAll('main h2, main h3, main h4').forEach(heading => {
      if (!heading.id) {
        heading.id = heading.innerText
          .toLowerCase()
          .replace(/[^\w\s-]/g, '')
          .trim()
          .replace(/\s+/g, '-');
      }

      const anchor = document.createElement('a');
      anchor.className = 'ts-heading-anchor';
      anchor.href = `#${heading.id}`;
      anchor.setAttribute('aria-label', `Link to ${heading.innerText}`);
      anchor.innerText = '#';

      anchor.addEventListener('click', (e) => {
        e.preventDefault();
        history.pushState(null, null, `#${heading.id}`);
        navigator.clipboard.writeText(window.location.href).then(() => {
          if (window.tessera.toast) {
            window.tessera.toast('Copied direct section link to clipboard! 🔗', 'success');
          }
        });
        heading.scrollIntoView({ behavior: 'smooth' });
      });

      heading.appendChild(anchor);
    });
  }

  // 6. Reading Time Estimation (data-ts-reading-time)
  function initReadingTime() {
    document.querySelectorAll('[data-ts-reading-time]').forEach(target => {
      const main = document.querySelector('main');
      if (!main) return;
      const text = main.innerText || '';
      const wordCount = text.trim().split(/\s+/).filter(Boolean).length;
      const readingMinutes = Math.max(1, Math.ceil(wordCount / 200));
      target.innerText = `⏱️ ${readingMinutes} min read (${wordCount} words)`;
    });
  }

  // 7. Instant Page Prefetching on Link Hover
  function initPrefetch() {
    const prefetched = new Set();
    document.querySelectorAll('a[href^="/"]').forEach(link => {
      link.addEventListener('mouseenter', () => {
        const href = link.getAttribute('href');
        if (!href || href.startsWith('/#') || prefetched.has(href)) return;
        prefetched.add(href);

        const linkEl = document.createElement('link');
        linkEl.rel = 'prefetch';
        linkEl.href = href;
        document.head.appendChild(linkEl);
      });
    });
  }

  // 8. Command Palette & Search (Ctrl+K)
  function initCommandPalette() {
    let searchIndex = null;
    let dialog = document.getElementById('ts-search-dialog');

    if (!dialog) {
      dialog = document.createElement('dialog');
      dialog.id = 'ts-search-dialog';
      dialog.className = 'ts-search-dialog';
      dialog.innerHTML = `
        <div class="ts-search-box">
          <input type="text" class="ts-search-input" placeholder="Search pages or type > for actions... (Esc to close)" autocomplete="off">
          <div class="ts-search-results"></div>
        </div>
      `;
      document.body.appendChild(dialog);
    }

    const input = dialog.querySelector('.ts-search-input');
    const resultsContainer = dialog.querySelector('.ts-search-results');

    async function loadIndex() {
      if (searchIndex !== null) return searchIndex;
      try {
        const res = await fetch('/search-index.json');
        searchIndex = await res.json();
      } catch (e) {
        searchIndex = [];
      }
      return searchIndex;
    }

    const QUICK_ACTIONS = [
      { title: '🌓 Toggle Dark / Light Theme', action: () => { toggleTheme(); dialog.close(); } },
      { title: '📋 Copy Page URL to Clipboard', action: () => { navigator.clipboard.writeText(window.location.href); if (window.tessera.toast) window.tessera.toast('Copied URL! 🔗', 'success'); dialog.close(); } },
      { title: '⬆️ Scroll to Top of Page', action: () => { window.scrollTo({ top: 0, behavior: 'smooth' }); dialog.close(); } },
      { title: '🐙 View Project on GitHub', action: () => { window.open('https://github.com/cybercoretech/tessera', '_blank'); dialog.close(); } },
    ];

    function openSearch() {
      dialog.showModal();
      input.value = '';
      renderInitial();
      input.focus();
      loadIndex();
    }

    function renderInitial() {
      let html = '<div class="ts-search-section-title">⚡ Quick Actions</div>';
      QUICK_ACTIONS.forEach((act, i) => {
        html += `<div class="ts-search-item" data-action-idx="${i}"><div class="ts-search-item__title">${act.title}</div></div>`;
      });
      html += '<div class="ts-search-hint">Or type to search documentation & pages...</div>';
      resultsContainer.innerHTML = html;

      resultsContainer.querySelectorAll('[data-action-idx]').forEach(item => {
        item.addEventListener('click', () => {
          const idx = parseInt(item.dataset.actionIdx, 10);
          QUICK_ACTIONS[idx].action();
        });
      });
    }

    document.querySelectorAll('[data-ts-search-trigger]').forEach(btn => btn.addEventListener('click', openSearch));

    document.addEventListener('keydown', (e) => {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'k') {
        e.preventDefault();
        if (dialog.open) dialog.close();
        else openSearch();
      }
    });

    dialog.addEventListener('click', (e) => {
      if (e.target === dialog) dialog.close();
    });

    input.addEventListener('input', async () => {
      const q = input.value.trim().toLowerCase();
      if (!q) {
        renderInitial();
        return;
      }

      const data = await loadIndex();
      const matches = data.filter(item =>
        item.title.toLowerCase().includes(q) || item.description.toLowerCase().includes(q)
      );

      if (matches.length === 0) {
        resultsContainer.innerHTML = '<div class="ts-search-empty">No matching pages found</div>';
        return;
      }

      resultsContainer.innerHTML = matches.map(item => `
        <a class="ts-search-item" href="${item.url}">
          <div class="ts-search-item__title">📄 ${item.title}</div>
          ${item.description ? `<div class="ts-search-item__desc">${item.description}</div>` : ''}
        </a>
      `).join('');
    });
  }

  // 9. Code Copy Button
  function initCodeCopy() {
    document.querySelectorAll('pre').forEach(pre => {
      if (pre.querySelector('.ts-code-copy')) return;
      const copyBtn = document.createElement('button');
      copyBtn.className = 'ts-code-copy';
      copyBtn.setAttribute('aria-label', 'Copy code snippet');
      copyBtn.innerText = 'Copy';

      copyBtn.addEventListener('click', () => {
        const code = pre.querySelector('code') || pre;
        navigator.clipboard.writeText(code.innerText).then(() => {
          copyBtn.innerText = 'Copied! ✓';
          copyBtn.classList.add('ts-copied');
          setTimeout(() => {
            copyBtn.innerText = 'Copy';
            copyBtn.classList.remove('ts-copied');
          }, 2000);
        }).catch(() => {
          copyBtn.innerText = 'Error';
        });
      });

      pre.appendChild(copyBtn);
    });
  }

  // 10. Accessible Tabs
  function initTabs() {
    document.querySelectorAll('[data-ts-tabs]').forEach(tabsContainer => {
      const tabButtons = Array.from(tabsContainer.querySelectorAll('[data-ts-tab]'));
      const tabPanels = Array.from(tabsContainer.querySelectorAll('[data-ts-tabpanel]'));

      tabButtons.forEach((button, index) => {
        button.setAttribute('role', 'tab');
        button.setAttribute('tabindex', index === 0 ? '0' : '-1');
        button.setAttribute('aria-selected', index === 0 ? 'true' : 'false');

        button.addEventListener('click', () => selectTab(index));
        button.addEventListener('keydown', (e) => {
          let nextIndex = null;
          if (e.key === 'ArrowRight') nextIndex = (index + 1) % tabButtons.length;
          else if (e.key === 'ArrowLeft') nextIndex = (index - 1 + tabButtons.length) % tabButtons.length;
          else if (e.key === 'Home') nextIndex = 0;
          else if (e.key === 'End') nextIndex = tabButtons.length - 1;

          if (nextIndex !== null) {
            e.preventDefault();
            selectTab(nextIndex);
            tabButtons[nextIndex].focus();
          }
        });
      });

      function selectTab(activeIndex) {
        tabButtons.forEach((btn, idx) => {
          const isSelected = idx === activeIndex;
          btn.setAttribute('aria-selected', String(isSelected));
          btn.setAttribute('tabindex', isSelected ? '0' : '-1');
          btn.classList.toggle('ts-tab--active', isSelected);
        });

        tabPanels.forEach((panel, idx) => {
          const isSelected = idx === activeIndex;
          panel.hidden = !isSelected;
          panel.classList.toggle('ts-tabpanel--active', isSelected);
        });
      }
    });
  }

  // 11. Reading Progress Bar
  function initReadingProgress() {
    const bar = document.querySelector('[data-ts-progress]');
    if (!bar) return;

    let ticking = false;
    window.addEventListener('scroll', () => {
      if (!ticking) {
        window.requestAnimationFrame(() => {
          const total = document.documentElement.scrollHeight - window.innerHeight;
          const current = window.scrollY;
          const progress = total > 0 ? (current / total) * 100 : 0;
          bar.style.width = `${Math.min(100, Math.max(0, progress))}%`;
          ticking = false;
        });
        ticking = true;
      }
    });
  }

  // 12. Image Lightbox
  function initLightbox() {
    let overlay = document.getElementById('ts-lightbox');
    if (!overlay) {
      overlay = document.createElement('div');
      overlay.id = 'ts-lightbox';
      overlay.className = 'ts-lightbox';
      overlay.innerHTML = '<img class="ts-lightbox__img" alt="Zoomed view"><button class="ts-lightbox__close" aria-label="Close">&times;</button>';
      document.body.appendChild(overlay);
    }

    const img = overlay.querySelector('.ts-lightbox__img');
    const closeBtn = overlay.querySelector('.ts-lightbox__close');

    function closeLightbox() {
      overlay.classList.remove('ts-open');
    }

    closeBtn.addEventListener('click', closeLightbox);
    overlay.addEventListener('click', (e) => {
      if (e.target === overlay) closeLightbox();
    });
    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape' && overlay.classList.contains('ts-open')) {
        closeLightbox();
      }
    });

    document.querySelectorAll('main img, [data-ts-lightbox]').forEach(thumb => {
      thumb.style.cursor = 'zoom-in';
      thumb.addEventListener('click', () => {
        img.src = thumb.src;
        img.alt = thumb.alt || 'Zoomed view';
        overlay.classList.add('ts-open');
      });
    });
  }

  // 13. Mobile Drawer & Alerts
  function initDrawersAndAlerts() {
    document.querySelectorAll('[data-ts-toggle]').forEach(trigger => {
      const targetId = trigger.getAttribute('data-ts-toggle');
      const target = document.getElementById(targetId);
      if (!target) return;

      trigger.addEventListener('click', (e) => {
        e.stopPropagation();
        const isOpen = target.classList.contains('ts-open');
        target.classList.toggle('ts-open', !isOpen);
        trigger.setAttribute('aria-expanded', String(!isOpen));
      });
    });

    document.addEventListener('click', (e) => {
      document.querySelectorAll('.ts-open').forEach(openElem => {
        if (!openElem.contains(e.target)) {
          openElem.classList.remove('ts-open');
          const trigger = document.querySelector(`[data-ts-toggle="${openElem.id}"]`);
          if (trigger) trigger.setAttribute('aria-expanded', 'false');
        }
      });
    });

    document.querySelectorAll('[data-ts-dismiss]').forEach(button => {
      button.addEventListener('click', () => {
        const targetType = button.getAttribute('data-ts-dismiss');
        const alertEl = button.closest(`.ts-${targetType}`) || button.parentElement;
        if (alertEl) {
          alertEl.style.transition = 'opacity 0.25s ease, transform 0.25s ease';
          alertEl.style.opacity = '0';
          alertEl.style.transform = 'scale(0.97)';
          setTimeout(() => alertEl.remove(), 250);
        }
      });
    });
  }

  // Initialize all interactive components
  function initAll() {
    initTheme();
    initToasts();
    initSyntaxHighlighting();
    initTableSorting();
    initHeadingAnchors();
    initReadingTime();
    initPrefetch();
    initCommandPalette();
    initCodeCopy();
    initTabs();
    initReadingProgress();
    initLightbox();
    initDrawersAndAlerts();
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initAll);
  } else {
    initAll();
  }
})();
