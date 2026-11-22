/**
 * Tessera Progressive Enhancement Micro-Script (~1.5KB)
 * Pure vanilla JavaScript - Zero dependencies
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

    // Close drawers on outside click
    document.addEventListener('click', (e) => {
      document.querySelectorAll('.ts-open').forEach(openElem => {
        if (!openElem.contains(e.target)) {
          openElem.classList.remove('ts-open');
          const trigger = document.querySelector(`[data-ts-toggle="${openElem.id}"]`);
          if (trigger) trigger.setAttribute('aria-expanded', 'false');
        }
      });
    });

    // Close drawers on Escape key
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

  // Initialize all interactive behaviors once DOM is ready
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
      initTheme();
      initToggles();
      initDismiss();
      initModals();
    });
  } else {
    initTheme();
    initToggles();
    initDismiss();
    initModals();
  }
})();
