// Copyright 2026 Kacper Górec. Licensed under the Apache License, Version 2.0.
// Small helpers for the project site: scroll reveal and the docs table of contents.

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

// Copy button on the terminal block.
document.querySelectorAll('.copy').forEach((b) => b.addEventListener('click', async () => {
  try { await navigator.clipboard.writeText(b.dataset.copy); b.textContent = 'Copied'; } catch { b.textContent = 'Press Ctrl+C'; }
  setTimeout(() => (b.textContent = 'Copy'), 1600);
}));

// Pinned story: scrolling drives the mockup (clock, menu bar, new event, auto-hide, island, dark).
(() => {
  const story = document.querySelector('.story');
  if (!story) return;
  const stage = story.querySelector('.stage'), scene = story.querySelector('.scene'), side = story.querySelector('.side');
  const evs = [...story.querySelectorAll('.ev[data-s]')], ghost = story.querySelector('.ghost');  // ghost is the entry "added" in step 3
  const now = story.querySelector('.now'), title = story.querySelector('.mbar b'), clk = story.querySelector('.clk');
  const caps = [...story.querySelectorAll('.cap')], dots = [...story.querySelectorAll('.dots i')];
  const N = caps.length, T0 = 8 * 60 + 40, T1 = 20 * 60 + 10;
  const dark = matchMedia('(prefers-color-scheme: dark)').matches;
  const dur = (m) => (m >= 60 ? `${Math.floor(m / 60)}h${m % 60 ? ' ' + (m % 60) + 'm' : ''}` : `${m}m`);
  const clamp = (v) => Math.min(1, Math.max(0, v));
  let step = -1, lastT = -1;

  // Same tiers as the app: two lines when tall enough, one line otherwise.
  const tiers = () => evs.forEach((e) => e.classList.toggle('l2', e.offsetHeight >= 40));

  function draw() {
    const rect = story.getBoundingClientRect();
    const p = clamp((52 - rect.top) / (story.offsetHeight - stage.offsetHeight));
    const t = Math.round(T0 + (T1 - T0) * p);
    const i = Math.min(N - 1, Math.floor(p * N));

    if (i !== step) {
      step = i;
      caps.forEach((c, k) => c.classList.toggle('on', k === i));
      dots.forEach((d, k) => d.classList.toggle('on', k === i));
      ghost.classList.toggle('on', i >= 2);
      side.classList.toggle('collapsed', i === 3);
      side.classList.toggle('island', i >= 4);
      scene.dataset.mode = (i >= 5) === dark ? 'light' : 'dark';
      lastT = -1;  // the ghost entry joins (or leaves) the clock logic
      setTimeout(tiers, 0);
    }
    if (t !== lastT) {
      lastT = t;
      now.style.top = (t / 14.4) + '%';
      let cur = null, next = null;
      for (const e of evs) {
        if (e === ghost && !ghost.classList.contains('on')) { e.classList.remove('active'); continue; }
        const on = +e.dataset.s <= t && +e.dataset.e > t;
        e.classList.toggle('active', on);
        if (on) cur = e;
        else if (+e.dataset.s > t && (!next || +e.dataset.s < +next.dataset.s)) next = e;
      }
      side.style.setProperty('--glow', cur ? cur.style.getPropertyValue('--ev-c') : 'transparent');
      title.textContent = cur ? `${cur.dataset.t} (${dur(+cur.dataset.e - t)} left)` : next ? `${next.dataset.t} in ${dur(+next.dataset.s - t)}` : 'Calendar';
      clk.textContent = `${Math.floor(t / 60)}:${String(t % 60).padStart(2, '0')}`;
    }
  }
  let queued = false;
  const tick = () => { if (queued) return; queued = true; requestAnimationFrame(() => { queued = false; draw(); }); };
  addEventListener('scroll', tick, { passive: true });
  addEventListener('resize', () => { tiers(); tick(); });
  tiers(); draw();
})();

// Hero chip: roll through the assistant names.
(() => {
  const words = [...document.querySelectorAll('.chip .rot b')];
  if (words.length < 2 || matchMedia('(prefers-reduced-motion: reduce)').matches) return;
  let i = 0;
  setInterval(() => {
    const n = (i + 1) % words.length, prev = i;
    words[prev].className = 'off';
    words[n].className = 'on';
    setTimeout(() => (words[prev].className = ''), 600);
    i = n;
  }, 2000);
})();
