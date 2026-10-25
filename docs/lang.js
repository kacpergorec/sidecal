// Copyright 2026 Kacper Górec. Licensed under the Apache License, Version 2.0.
// Homepage translations. The HTML stays English (it is what search engines index); this swaps the text at load.
// Language: the visitor's saved pick, else the browser's first supported language, else English. Strings live in i18n.js,
// keyed by the English text, so the markup needs no annotations.
(() => {
  const LANGS = ['en', 'pl', 'de', 'es', 'fr'];
  const root = document.documentElement;
  let saved = null;
  try { saved = localStorage.getItem('lang'); } catch {}
  const guess = () => {
    for (const l of navigator.languages || [navigator.language]) {
      const c = String(l || '').slice(0, 2).toLowerCase();
      if (LANGS.includes(c)) return c;
    }
    return 'en';
  };
  const lang = LANGS.includes(saved) ? saved : guess();
  const dict = (window.SC_I18N || {})[lang];
  window.SC_UI = (dict && dict.ui) || {};
  if (lang === 'en' || !dict) { document.addEventListener('DOMContentLoaded', wire); return; }

  root.lang = lang;
  // Hide the page until the swap is done so English never flashes; the timer is a safety net.
  root.classList.add('i18n-wait');
  const done = () => root.classList.remove('i18n-wait');
  setTimeout(done, 1500);

  const INLINE = new Set(['B', 'A', 'CODE', 'KBD', 'EM', 'STRONG']);
  const SKIP = 'svg, script, style, pre, .names, .swap, .theme-toggle';
  const norm = (s) => s.replace(/\s+/g, ' ').trim();
  const s = dict.s;

  function translate() {
    for (const el of document.querySelectorAll('header.nav *, main *, footer *')) {
      if (el.closest(SKIP)) continue;
      for (const a of ['title', 'aria-label', 'data-t']) {
        const v = el.getAttribute(a);
        if (v && s[v]) el.setAttribute(a, s[v]);
      }
      if ([...el.children].every((c) => INLINE.has(c.tagName)) && el.firstChild) {
        const t = s[norm(el.innerHTML)];
        if (t) { el.innerHTML = t; continue; }
      }
      // Mixed content (icon plus label): translate the bare text nodes.
      for (const n of el.childNodes) {
        if (n.nodeType !== 3) continue;
        const t = s[norm(n.nodeValue)];
        if (t) n.nodeValue = t;
      }
    }
    if (s[document.title]) document.title = s[document.title];
    for (const m of document.querySelectorAll('meta[name="description"], meta[property^="og:"], meta[name^="twitter:"]')) {
      if (s[m.content]) m.content = s[m.content];
    }
    for (const j of document.querySelectorAll('script[type="application/ld+json"]')) {
      const walk = (v) => {
        if (typeof v === 'string') return s[v] || v;
        if (Array.isArray(v)) return v.map(walk);
        if (v && typeof v === 'object') return Object.fromEntries(Object.entries(v).map(([k, x]) => [k, k === '@context' || k === '@type' || k === 'url' ? x : walk(x)]));
        return v;
      };
      try { j.textContent = JSON.stringify(walk(JSON.parse(j.textContent))); } catch {}
    }
  }

  document.addEventListener('DOMContentLoaded', () => { translate(); wire(); done(); });

  function wire() {
    const sel = document.querySelector('.lang');
    if (!sel) return;
    sel.value = lang;
    sel.addEventListener('change', () => {
      try { localStorage.setItem('lang', sel.value); } catch {}
      location.reload();
    });
  }
})();
