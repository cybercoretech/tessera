/**
 * Tessera Progressive Enhancement Micro-Script
 * Pure vanilla JavaScript - Zero dependencies (~3KB)
 */
(function () {
  'use strict';

  // 1. Theme Management (Dark / Light Mode)
  function initTheme() {
    const savedTheme = localStorage.getItem('ts-theme');
    const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    const initialTheme = savedTheme || (systemPrefersDark ? 'dark' : 'light');
    document.documentElement.setAttribute('data-theme', initialTheme);

    document.querySelectorAll('[data-ts-theme-toggle]').forEach(button => {
      button.addEventListener('click', () => {
        const current = document.documentElement.getAttribute('data-theme') || 'light';
        const next = current === 'dark' ? 'light' : 'dark';
        document.documentElement.setAttribute('data-theme', next);
        localStorage.setItem('ts-theme', next);
      });
    });
  }

  // 2. Drawer / Mobile Menu Toggles (data-ts-toggle="element-id")
  function initToggles() {
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

    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape') {
        document.querySelectorAll('.ts-open').forEach(openElem => {
          openElem.classList.remove('ts-open');
          const trigger = document.querySelector(`[data-ts-toggle="${openElem.id}"]`);
          if (trigger) {
            trigger.setAttribute('aria-expanded', 'false');
            trigger.focus();
          }
        });
      }
    });
  }

  // 3. Dismissible Alerts & Banners (data-ts-dismiss="alert")
  function initDismiss() {
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

  // 4. Native Dialog / Modal Triggers (data-ts-modal="dialog-id")
  function initModals() {
    document.querySelectorAll('[data-ts-modal]').forEach(button => {
      const modalId = button.getAttribute('data-ts-modal');
      const dialog = document.getElementById(modalId);
      if (!dialog || typeof dialog.showModal !== 'function') return;

      button.addEventListener('click', () => {
        dialog.showModal();
      });

      dialog.querySelectorAll('[data-ts-close]').forEach(closeBtn => {
        closeBtn.addEventListener('click', () => dialog.close());
      });
    });
  }

  // 5. Code Copy-to-Clipboard Button
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

  // 6. Accessible Tabs (data-ts-tabs)
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

  // 7. Instant Client-Side Search (Ctrl+K or data-ts-search-trigger)
  function initSearch() {
    let searchIndex = null;
    let dialog = document.getElementById('ts-search-dialog');

    if (!dialog) {
      dialog = document.createElement('dialog');
      dialog.id = 'ts-search-dialog';
      dialog.className = 'ts-search-dialog';
      dialog.innerHTML = `
        <div class="ts-search-box">
          <input type="text" class="ts-search-input" placeholder="Search pages... (Esc to close)" autocomplete="off">
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

    function openSearch() {
      dialog.showModal();
      input.value = '';
      resultsContainer.innerHTML = '<div class="ts-search-hint">Type to search pages...</div>';
      input.focus();
      loadIndex();
    }

    document.querySelectorAll('[data-ts-search-trigger]').forEach(btn => {
      btn.addEventListener('click', openSearch);
    });

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
        resultsContainer.innerHTML = '<div class="ts-search-hint">Type to search pages...</div>';
        return;
      }
      const data = await loadIndex();
      const matches = data.filter(item =>
        item.title.toLowerCase().includes(q) || item.description.toLowerCase().includes(q)
      );

      if (matches.length === 0) {
        resultsContainer.innerHTML = '<div class="ts-search-empty">No results found</div>';
        return;
      }

      resultsContainer.innerHTML = matches.map(item => `
        <a class="ts-search-item" href="${item.url}">
          <div class="ts-search-item__title">${item.title}</div>
          ${item.description ? `<div class="ts-search-item__desc">${item.description}</div>` : ''}
        </a>
      `).join('');
    });
  }

  // 8. Reading Progress Bar (data-ts-progress)
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

  // 9. Image Lightbox
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

  // Initialize all interactive components once DOM is loaded
  function initAll() {
    initTheme();
    initToggles();
    initDismiss();
    initModals();
    initCodeCopy();
    initTabs();
    initSearch();
    initReadingProgress();
    initLightbox();
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initAll);
  } else {
    initAll();
  }
})();
