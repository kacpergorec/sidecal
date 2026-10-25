// Copyright 2026 Kacper Górec. Licensed under the Apache License, Version 2.0.
// Small helpers for the project site: scroll reveal, GitHub links and the docs table of contents.

// Fade elements in as they enter the viewport (IntersectionObserver, never a scroll listener).
const reveal = [...document.querySelectorAll('.reveal')];
if ('IntersectionObserver' in window && !matchMedia('(prefers-reduced-motion: reduce)').matches) {
  const io = new IntersectionObserver((entries) => {
    for (const e of entries) if (e.isIntersecting) { e.target.classList.add('in'); io.unobserve(e.target); }
  }, { threshold: 0.12 });
  reveal.forEach((el) => io.observe(el));
} else {
  reveal.forEach((el) => el.classList.add('in'));
}

// Links to the repository are derived from the Pages address (https://USER.github.io/REPO/),
// so the site needs no configuration. Opened from disk, those links are hidden.
const m = /^([^.]+)\.github\.io$/.exec(location.hostname);
const repo = m && location.pathname.split('/')[1] ? `https://github.com/${m[1]}/${location.pathname.split('/')[1]}` : '';
document.querySelectorAll('[data-repo]').forEach((a) => {
  if (repo) a.href = repo + a.dataset.repo; else a.hidden = true;
});

// Highlight the current section in the docs table of contents.
const links = [...document.querySelectorAll('.toc a')];
if (links.length && 'IntersectionObserver' in window) {
  const spy = new IntersectionObserver((entries) => {
    for (const e of entries) {
      if (!e.isIntersecting) continue;
      links.forEach((a) => a.classList.toggle('on', a.getAttribute('href') === '#' + e.target.id));
    }
  }, { rootMargin: '-90px 0px -70% 0px' });
  links.forEach((a) => { const s = document.querySelector(a.getAttribute('href')); if (s) spy.observe(s); });
}
