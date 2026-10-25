// Copyright 2026 Kacper Górec. Licensed under the Apache License, Version 2.0.
// Theme: the visitor's saved choice, else the system setting. Runs in <head> to avoid a flash.
(() => {
  const root = document.documentElement;
  const dark = matchMedia('(prefers-color-scheme: dark)');
  let saved = null;
  try { saved = localStorage.getItem('theme'); } catch {}
  const apply = (t) => {
    root.dataset.theme = t;
    const meta = document.querySelector('meta[name="theme-color"]');
    if (meta) meta.content = t === 'dark' ? '#000000' : '#ffffff';
    document.dispatchEvent(new CustomEvent('themechange', { detail: t }));
  };
  root.dataset.theme = saved === 'light' || saved === 'dark' ? saved : dark.matches ? 'dark' : 'light';
  // Follow the system while the visitor has not chosen.
  dark.addEventListener('change', (e) => { if (!saved) apply(e.matches ? 'dark' : 'light'); });
  document.addEventListener('DOMContentLoaded', () => {
    const btn = document.querySelector('.theme-toggle');
    if (!btn) return;
    const UI = window.SC_UI || {};
    const label = () => btn.setAttribute('aria-label', root.dataset.theme === 'dark' ? (UI.toLight || 'Switch to light mode') : (UI.toDark || 'Switch to dark mode'));
    label();
    document.addEventListener('themechange', label);
    btn.addEventListener('click', () => {
      saved = root.dataset.theme === 'dark' ? 'light' : 'dark';
      try { localStorage.setItem('theme', saved); } catch {}
      apply(saved);
    });
    const meta = document.querySelector('meta[name="theme-color"]');
    if (meta) meta.content = root.dataset.theme === 'dark' ? '#000000' : '#ffffff';
  });
})();
