// Copyright 2026 Kacper Górec. Licensed under the Apache License, Version 2.0 (see LICENSE and NOTICE).
// Shared by the sidebar and the settings window: the 12 entry/calendar colours and colour helpers.
// Every colour has a light-mode and a dark-mode variant; only the palette id is stored.
const PALETTE = [
  { id: 'red',    light: '#ff3b30', dark: '#ff453a' }, { id: 'coral',  light: '#ff7a5c', dark: '#ff8a6b' },
  { id: 'orange', light: '#ff9500', dark: '#ff9f0a' }, { id: 'yellow', light: '#ffcc00', dark: '#ffd60a' },
  { id: 'lime',   light: '#7ac943', dark: '#a3e05a' }, { id: 'green',  light: '#34c759', dark: '#30d158' },
  { id: 'teal',   light: '#12b886', dark: '#38d9a9' }, { id: 'blue',   light: '#007aff', dark: '#0a84ff' },
  { id: 'purple', light: '#af52de', dark: '#bf5af2' }, { id: 'pink',   light: '#ff2d55', dark: '#ff375f' },
  { id: 'brown',  light: '#a2845e', dark: '#ac8e68' }, { id: 'gray',   light: '#8e8e93', dark: '#98989d' },
];
// Colours dropped from the palette (too many blues) map onto the nearest remaining one.
const PALETTE_ALIASES = { sky: 'blue', indigo: 'purple' };
const scFind = (id) => PALETTE.find((x) => x.id === (PALETTE_ALIASES[id] || id));

const scIsDark = (theme) =>
  theme === 'dark' || (theme !== 'light' && matchMedia('(prefers-color-scheme: dark)').matches);
const scTone = (p, theme) => (scIsDark(theme) ? p.dark : p.light);

// Calendar colours: keep hue, lift saturation and pull lightness into a lively mid-range.
function scSoften(hex, theme) {
  const m = /^#?([0-9a-f]{6})$/i.exec(hex || '');
  if (!m) return hex;
  const [r, g, b] = [0, 2, 4].map((i) => parseInt(m[1].slice(i, i + 2), 16) / 255);
  const mx = Math.max(r, g, b), mn = Math.min(r, g, b), d = mx - mn;
  let l = (mx + mn) / 2, s = d === 0 ? 0 : d / (1 - Math.abs(2 * l - 1)), hue = 0;
  if (d) {
    hue = mx === r ? ((g - b) / d) % 6 : mx === g ? (b - r) / d + 2 : (r - g) / d + 4;
    hue = (hue * 60 + 360) % 360;
  }
  if (s < 0.12) return hex;                       // greys stay grey
  s = Math.min(0.95, Math.max(s * 1.15, 0.7));
  l = scIsDark(theme) ? Math.min(0.62, Math.max(l, 0.52)) : Math.min(0.56, Math.max(l, 0.46));
  const c = (1 - Math.abs(2 * l - 1)) * s, x = c * (1 - Math.abs(((hue / 60) % 2) - 1)), mm = l - c / 2;
  const [r1, g1, b1] = hue < 60 ? [c, x, 0] : hue < 120 ? [x, c, 0] : hue < 180 ? [0, c, x] : hue < 240 ? [0, x, c] : hue < 300 ? [x, 0, c] : [c, 0, x];
  return '#' + [r1, g1, b1].map((v) => Math.round((v + mm) * 255).toString(16).padStart(2, '0')).join('');
}
