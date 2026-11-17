// Materializes the guides for every published version into the layout @viteplus/versions serves:
//   docs-site/src/                the unreleased guide from the working tree, served at /dev/
//   docs-site/archive/<tag>/      the guide as it was at each stable release tag, served at /<tag>/
//   docs-site/.generated.json     which guides each version lacks, and the latest stable tag
// Guide content comes from its tag; the layout, navigation overview and tooling come from main.
import fs from 'node:fs';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
// `docs-site/` holds the tooling; `docs-site/site/` is the VitePress root the plugin serves.
export const toolDir = path.resolve(here, '..');
export const siteDir = path.join(toolDir, 'site');
const defaultRepo = path.resolve(toolDir, '..');
export const manifestFile = path.join(toolDir, '.generated.json');
export const documentation = JSON.parse(fs.readFileSync(path.join(defaultRepo, 'vscode/src/help/documentation.json'), 'utf8'));

export const groups = [...new Set(Object.values(documentation.pages).map(page => page.group))].map(title => ({
  title,
  pages: Object.values(documentation.pages).filter(page => page.group === title)
}));
export const pages = groups.flatMap(group => group.pages);

const git = (repo, ...args) => execFileSync('git', args, { cwd: repo, encoding: 'utf8', maxBuffer: 32 * 1024 * 1024 });

export function version(tag) {
  const match = /^v(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$/.exec(tag);
  return match ? match.slice(1).map(Number) : null;
}

export function releases(tags) {
  // Start the hosted archive at 0.60. Earlier guides remain available on GitHub.
  return [...new Set(tags)].filter(tag => {
    const v = version(tag);
    return v && (v[0] > 0 || v[1] >= 60);
  }).sort((a, b) => {
    const av = version(a), bv = version(b);
    return bv[0] - av[0] || bv[1] - av[1] || bv[2] - av[2];
  });
}

// Generated guides begin with an HTML comment marking them as generated. Raw HTML stays disabled,
// so the comment would otherwise print as visible text. Comments inside code fences are examples
// and stay.
export function withoutHtmlComments(markdown) {
  let fenced = false;
  return markdown.split('\n').filter(line => {
    if (/^\s*(```|~~~)/.test(line)) fenced = !fenced;
    return fenced || !/^\s*<!--.*-->\s*$/.test(line);
  }).join('\n');
}

/**
 * Rewrites the text outside code fences and spans. `transform` receives each prose segment.
 * Code stays byte-for-byte what the author wrote.
 */
function mapLines(markdown, transform) {
  let fenced = false;
  return markdown.split('\n').map(line => {
    if (/^\s*(```|~~~)/.test(line)) { fenced = !fenced; return line; }
    return fenced ? line : transform(line);
  }).join('\n');
}
const mapProse = (markdown, transform) => mapLines(markdown, line =>
  line.split(/(`[^`]*`)/).map((part, index) => (index % 2 ? part : transform(part))).join(''));

// Vue would read `{{ x }}` in prose as an interpolation.
const escapeInterpolation = markdown => mapProse(markdown, text => text.replaceAll('{{', '&#123;&#123;'));

/**
 * Resolves the links and images of one guide. A link to another guide becomes a page link that
 * VitePress resolves (and fails the build on if the target is missing); any other repository file
 * links to the same revision on GitHub.
 */
export function rewriteLinks(markdown, { route, source, ref, exists, repository = 'https://github.com/elan8/spec42' }) {
  const resolve = (href, image) => {
    if (/^(https?:|mailto:|tel:)/.test(href) || href.startsWith('#')) return href;
    const match = /^([^?#]*)(\?[^#]*)?(#.*)?$/.exec(href);
    if (!match) throw new Error(`Unsupported URL: ${href}`);
    const [, file, query = '', fragment = ''] = match;
    const target = source ? path.posix.normalize(path.posix.join(path.posix.dirname(source), decodeURIComponent(file))) : file;
    const page = source ? pages.find(p => p.source === target) : pages.find(p => file === `${p.route}.md` || file === `${p.route}.html`);
    if (page && !image) return `${page.route}.md${query}${fragment}`;
    if (!source || target.startsWith('../') || target.startsWith('/')) throw new Error(`Unresolved link in ${route}: ${href}`);
    // Contributor and reference material stays on the matching source revision on GitHub.
    if (!exists(target)) throw new Error(`Missing link target: ${target}`);
    return `${repository}/${image ? 'raw' : 'blob'}/${ref}/${target}${query}${fragment}`;
  };
  // A link label may itself contain code (`[`.github/x.md`](...)`), so a link and a code span are
  // alternatives scanned left to right: code that is not a label is left exactly as written.
  const token = /(!?)\[((?:[^\]`]|`[^`]*`)*)\]\(([^)\s]+)((?:\s+"[^"]*")?)\)|`[^`]*`/g;
  return mapLines(markdown, line => line.replace(token,
    (whole, bang, label, href, title) => (href === undefined ? whole : `${bang}[${label}](${resolve(href, bang === '!')}${title})`)));
}

/**
 * The site logo is the repository's own mark (`vscode/design/logo.svg`, a `currentColor` glyph) on the
 * Elan8 cyan tile that the extension icon uses. It is derived at build time so it cannot drift.
 */
export function writeLogo({ siteRoot = siteDir } = {}) {
  const glyph = fs.readFileSync(path.join(defaultRepo, 'vscode/design/logo.svg'), 'utf8');
  if (!glyph.includes('currentColor')) throw new Error('vscode/design/logo.svg must draw with currentColor');
  const inner = glyph.replace(/^[\s\S]*?<svg[^>]*>/, '').replace(/<\/svg>\s*$/, '').replaceAll('currentColor', '#ecfeff');
  fs.mkdirSync(path.join(siteRoot, 'public'), { recursive: true });
  fs.writeFileSync(path.join(siteRoot, 'public/logo.svg'),
    `<svg viewBox="0 0 64 64" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="Spec42">\n  <rect width="64" height="64" rx="14" fill="#0891b2"/>${inner}</svg>\n`);
}

const stub = title => `# ${title}\n\nThis release does not include this guide. Browse the [overview](index.md) or [features and support](features.md) for this version.`;

export function prepare({ repo = defaultRepo, tags, siteRoot = siteDir, manifestPath = manifestFile, quick = false } = {}) {
  const stable = quick ? [] : releases(tags ?? git(repo, 'tag', '--list', 'v*').trim().split(/\s+/));
  if (!quick && !stable.length) throw new Error('No published stable releases at or after v0.60.0');
  // Only the generated folders may be replaced.
  for (const generated of ['src', 'archive']) fs.rmSync(path.join(siteRoot, generated), { recursive: true, force: true });
  fs.rmSync(manifestPath, { force: true });
  writeLogo({ siteRoot });

  const missing = {};
  for (const edition of [...stable, 'dev']) {
    const isDev = edition === 'dev';
    const ref = isDev ? 'main' : edition;
    const folder = path.join(siteRoot, isDev ? 'src' : `archive/${edition}`);
    const cache = new Map();
    const read = file => {
      if (!cache.has(file)) cache.set(file, isDev ? fs.readFileSync(path.join(repo, file), 'utf8') : git(repo, 'show', `${edition}:${file}`));
      return cache.get(file);
    };
    const available = new Set(isDev
      ? pages.filter(p => p.source && fs.existsSync(path.join(repo, p.source))).map(p => p.source)
      : git(repo, 'ls-tree', '-r', '--name-only', edition).trim().split('\n'));
    const exists = target => {
      if (isDev) return fs.existsSync(path.join(repo, target));
      try { git(repo, 'cat-file', '-e', `${edition}:${target}`); return true; } catch { return false; }
    };
    fs.mkdirSync(folder, { recursive: true });
    missing[edition] = [];
    for (const { route, title, source } of pages) {
      const absent = source && !available.has(source);
      if (absent && isDev) throw new Error(`Missing current guide: ${source}`);
      // A guide new since an old tag keeps an explicit page, rather than showing current content
      // under an old release label; the sidebar for that version leaves it out.
      if (absent) missing[edition].push(route);
      const markdown = absent ? stub(title) : source ? read(source) : fs.readFileSync(path.join(toolDir, 'overview.md'), 'utf8');
      const body = escapeInterpolation(rewriteLinks(withoutHtmlComments(markdown), { route, source: absent ? null : source, ref, exists }));
      const front = ['---', `title: ${JSON.stringify(title)}`, ...(source && !absent ? [`source: ${source}`, `ref: ${ref}`] : []), '---'];
      fs.writeFileSync(path.join(folder, `${route}.md`), `${front.join('\n')}\n\n${body}\n`);
    }
  }
  const manifest = { latest: stable[0] ?? null, versions: stable, missing };
  fs.writeFileSync(manifestPath, JSON.stringify(manifest, null, 2));
  return manifest;
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const releaseFile = process.argv.indexOf('--releases-file');
  const tags = releaseFile === -1 ? undefined : fs.readFileSync(process.argv[releaseFile + 1], 'utf8').trim().split(/\s+/);
  const manifest = prepare({ tags, quick: process.argv.includes('--quick') });
  console.log(`Prepared ${manifest.versions.length} releases plus dev; latest ${manifest.latest}.`);
}
