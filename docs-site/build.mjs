import fs from 'node:fs';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import MarkdownIt from 'markdown-it';

const here = path.dirname(fileURLToPath(import.meta.url));
const defaultRepo = path.resolve(here, '..');
const documentation = JSON.parse(fs.readFileSync(path.join(here, '../vscode/src/help/documentation.json'), 'utf8'));
const navigation = [...new Set(Object.values(documentation.pages).map(page => page.group))].map(title => ({ title, pages: Object.values(documentation.pages).filter(page => page.group === title).map(page => [page.route, page.title, page.source]) }));
const pages = navigation.flatMap(group => group.pages);
const escape = text => String(text).replace(/[&<>"']/g, c => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' })[c]);
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

function slug(text) {
  return text.toLowerCase().replace(/[^\p{L}\p{N}\s_-]/gu, '').replace(/\s/g, '-');
}

export function buildSite({ repo = defaultRepo, out = path.join(defaultRepo, 'target/docs-site'), tags, base = new URL(documentation.baseUrl).pathname } = {}) {
  if (!/^\/[a-zA-Z0-9/_-]*\/$/.test(base)) throw new Error('Base must be an absolute directory URL ending in /');
  const stable = releases(tags ?? git(repo, 'tag', '--list', 'v*').trim().split(/\s+/));
  if (!stable.length) throw new Error('No published stable releases at or after v0.60.0');
  // Only this disposable build directory may be replaced. Resolve its parent too,
  // so a junction cannot redirect the cleanup outside the selected checkout.
  const expected = path.resolve(repo, 'target/docs-site');
  if (path.resolve(out) !== expected) throw new Error('Output must be the checkout target/docs-site directory');
  fs.mkdirSync(path.dirname(expected), { recursive: true });
  const parent = path.relative(fs.realpathSync(repo), fs.realpathSync(path.dirname(expected)));
  if (parent.startsWith('..') || path.isAbsolute(parent) || (fs.existsSync(expected) && fs.lstatSync(expected).isSymbolicLink())) throw new Error('Unsafe documentation output path');
  fs.rmSync(expected, { recursive: true, force: true });
  fs.mkdirSync(out, { recursive: true });
  const write = (file, content) => {
    const destination = path.join(out, file);
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    fs.writeFileSync(destination, content);
  };
  const editions = [...stable, 'dev'];
  const manifests = editions.map(id => ({ id, label: id === 'dev' ? 'Development' : id, url: `${base}${id}/index.html` }));
  const minorGroups = new Map();
  for (const tag of stable) {
    const [major, minor] = version(tag);
    const key = `${major}.${minor}`;
    if (!minorGroups.has(key)) minorGroups.set(key, []);
    minorGroups.get(key).push(tag);
  }
  const selector = current => `${[...minorGroups].map(([minor, tags]) => `<optgroup label="${minor}">${tags.map(tag => `<option value="${tag}"${tag === current ? ' selected' : ''}>${tag}${tag === stable[0] ? ' · Latest stable' : ''}</option>`).join('')}</optgroup>`).join('')}<optgroup label="Unreleased"><option value="dev"${current === 'dev' ? ' selected' : ''}>Development</option></optgroup>`;
  for (const edition of editions) {
    const isDev = edition === 'dev';
    const ref = isDev ? 'main' : edition;
    const commit = git(repo, 'rev-parse', isDev ? 'HEAD' : `${edition}^{commit}`).trim();
    const cache = new Map();
    const read = file => {
      if (!cache.has(file)) cache.set(file, isDev ? fs.readFileSync(path.join(repo, file), 'utf8') : git(repo, 'show', `${edition}:${file}`));
      return cache.get(file);
    };
    const anchors = new Map();
    const links = [];
    const rendered = new Map();
    const available = new Set(isDev ? pages.filter(p => !p[2] || fs.existsSync(path.join(repo, p[2]))).map(p => p[2]) : git(repo, 'ls-tree', '-r', '--name-only', edition).trim().split('\n'));
    for (const [route, title, source] of pages) {
      const md = new MarkdownIt({ html: false, linkify: true });
      const ids = new Set();
      const headings = [];
      md.core.ruler.push('headings', state => {
        for (let i = 0; i < state.tokens.length; i++) {
          const token = state.tokens[i];
          if (token.type !== 'heading_open') continue;
          const text = state.tokens[i + 1].children.filter(t => ['text', 'code_inline'].includes(t.type)).map(t => t.content).join('');
          const stem = slug(text);
          let id = stem, count = 0;
          while (ids.has(id)) id = `${stem}-${++count}`;
          ids.add(id);
          token.attrSet('id', id);
          if (token.tag === 'h2') headings.push({ id, text });
        }
      });
      const resolve = (href, image = false) => {
        if (/^(https?:|mailto:|tel:)/.test(href)) return href;
        if (href.startsWith('#')) {
          links.push({ from: route, to: route, anchor: decodeURIComponent(href.slice(1)) });
          return href;
        }
        const match = /^([^?#]*)(\?[^#]*)?(#.*)?$/.exec(href);
        if (!match) throw new Error(`Unsupported URL: ${href}`);
        const [, file, query = '', fragment = ''] = match;
        const target = source ? path.posix.normalize(path.posix.join(path.posix.dirname(source), decodeURIComponent(file))) : file;
        const page = source ? pages.find(p => p[2] === target) : pages.find(p => `${p[0]}.html` === file);
        if (page && !image) {
          links.push({ from: route, to: page[0], anchor: decodeURIComponent(fragment.slice(1)) });
          return `${page[0]}.html${query}${fragment}`;
        }
        if (!source || target.startsWith('../') || target.startsWith('/')) throw new Error(`Unresolved link in ${route}: ${href}`);
        // Contributor/reference links stay on the matching source revision on GitHub.
        if (isDev) {
          if (!fs.existsSync(path.join(repo, target))) throw new Error(`Missing link target: ${target}`);
        } else {
          git(repo, 'cat-file', '-e', `${edition}:${target}`);
        }
        return `https://github.com/elan8/spec42/${image ? 'raw' : 'blob'}/${ref}/${target}${query}${fragment}`;
      };
      const defaultLink = md.renderer.rules.link_open ?? ((tokens, idx, options, env, self) => self.renderToken(tokens, idx, options));
      md.renderer.rules.link_open = (tokens, idx, options, env, self) => {
        tokens[idx].attrSet('href', resolve(tokens[idx].attrGet('href')));
        return defaultLink(tokens, idx, options, env, self);
      };
      const defaultImage = md.renderer.rules.image;
      md.renderer.rules.image = (tokens, idx, options, env, self) => {
        tokens[idx].attrSet('src', resolve(tokens[idx].attrGet('src'), true));
        return defaultImage(tokens, idx, options, env, self);
      };
      const missing = source && !available.has(source);
      if (missing && isDev) throw new Error(`Missing current guide: ${source}`);
      // New guides may not exist in old tags. Keep their version-switch URLs explicit,
      // rather than displaying current content under an old release label.
      const markdown = missing ? `# ${title}\n\nThis release does not include this guide. Browse the [overview](index.html) or [features and support](features.html) for this version.` : source ? read(source) : fs.readFileSync(path.join(here, 'index.md'), 'utf8');
      if (missing) md.renderer.rules.link_open = (tokens, idx, options, env, self) => {
        const href = tokens[idx].attrGet('href');
        const target = href.replace('.html', '');
        links.push({ from: route, to: target, anchor: '' });
        return self.renderToken(tokens, idx, options);
      };
      const body = md.render(markdown);
      anchors.set(route, ids);
      rendered.set(route, { title, source: missing ? null : source, missing, body, headings, text: markdown.replace(/```[\s\S]*?```/g, '').replace(/[#*`|]/g, '') });
    }
    for (const link of links) {
      if (!anchors.has(link.to) || (link.anchor && !anchors.get(link.to).has(link.anchor))) {
        throw new Error(`Broken link in ${edition}/${link.from}: ${link.to}#${link.anchor}`);
      }
    }
    const search = [];
    for (const [route, { title, source, missing, body, headings, text }] of rendered) {
      const sidebar = navigation.map(group => `<section><h2>${escape(group.title)}</h2>${group.pages.filter(([id]) => !rendered.get(id).missing).map(([id, name]) => `<a href="${id}.html"${id === route ? ' aria-current="page"' : ''}>${escape(name)}</a>`).join('')}</section>`).join('');
      const notice = isDev ? 'Development documentation describes unreleased changes.' : edition === stable[0] ? `Latest stable release: ${edition}.` : `You are reading ${edition}. <a href="${base}${stable[0]}/${route}.html">Read the latest stable guide</a>.`;
      write(`${edition}/${route}.html`, `<!doctype html>
<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>${escape(title)} · Spec42 ${edition}</title><meta name="description" content="Spec42 ${edition} user guide: ${escape(title)}">${isDev ? '<meta name="robots" content="noindex">' : ''}<link rel="stylesheet" href="${base}assets/site.css"><script src="${base}assets/site.js" defer></script></head>
<body data-version="${edition}" data-page="${route}" data-base="${base}"><a class="skip" href="#content">Skip to content</a><header><a class="brand" href="index.html"><span class="brand-mark">S42</span> Spec42 <span class="wordmark-detail">Documentation</span></a><div class="header-actions"><label for="version">Version</label><select id="version">${selector(edition)}</select><a href="https://github.com/elan8/spec42">GitHub</a></div></header>
<div class="layout"><nav aria-label="Documentation"><details open><summary>Browse documentation</summary><label class="search-label" for="search">Search this version</label><input id="search" type="search" placeholder="Search guides…" autocomplete="off"><ul id="search-results" aria-live="polite"></ul>${sidebar}</details></nav><main id="content"><div class="notice${isDev ? ' development' : ''}">${notice}</div><article>${body}</article><footer>${source ? `<a href="https://github.com/elan8/spec42/blob/${ref}/${source}">View source for this version</a>` : `<a href="getting-started.html">Start with the installation guide →</a>`}<span>${edition} · ${commit.slice(0, 7)} · MIT License</span></footer></main><aside aria-label="On this page"><p>On this page</p>${headings.map(h => `<a href="#${escape(h.id)}">${escape(h.text)}</a>`).join('')}</aside></div></body></html>`);
      if (!missing) search.push({ title, url: `${route}.html`, text });
    }
    write(`${edition}/search.json`, JSON.stringify(search));
  }
  const redirect = (file, destination) => write(file, `<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><meta http-equiv="refresh" content="0;url=${escape(destination)}"><link rel="canonical" href="${escape(destination)}"><title>Spec42 documentation</title></head><body><a href="${escape(destination)}">Continue to Spec42 documentation</a></body></html>`);
  redirect('index.html', `${base}${stable[0]}/index.html`);
  for (const [route] of pages) redirect(`latest/${route}.html`, `${base}${stable[0]}/${route}.html`);
  for (const [minor, tags] of minorGroups) for (const [route] of pages) redirect(`${minor}/${route}.html`, `${base}${tags[0]}/${route}.html`);
  // Preserve the former site's main entry points.
  redirect('guide/index.html', `${base}${stable[0]}/getting-started.html`);
  redirect('reference/index.html', `${base}${stable[0]}/included.html`);
  redirect('guide/getting-started.html', `${base}${stable[0]}/getting-started.html`);
  redirect('reference/whats-included.html', `${base}${stable[0]}/included.html`);
  write('404.html', `<!doctype html><html lang="en"><meta charset="utf-8"><title>Page not found · Spec42</title><h1>Page not found</h1><p><a href="${base}">Choose a guide from the documentation home</a>.</p></html>`);
  write('versions.json', JSON.stringify({ latest: stable[0], versions: manifests }, null, 2));
  write('.nojekyll', '');
  for (const file of ['site.css', 'site.js']) write(`assets/${file}`, fs.readFileSync(path.join(here, file)));
  return { releases: stable, pages: pages.length * editions.length };
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const releaseFile = process.argv.indexOf('--releases-file');
  const tags = releaseFile === -1 ? undefined : fs.readFileSync(process.argv[releaseFile + 1], 'utf8').trim().split(/\s+/);
  console.log(buildSite({ tags }));
}
