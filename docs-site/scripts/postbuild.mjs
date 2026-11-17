// Runs after `vitepress build`: writes the stable aliases and old entry points as redirects, then
// checks every internal link and fragment in the built site. VitePress validates page links
// itself but not anchors, and a bad anchor on a published guide is a silent failure.
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { manifestFile, pages, siteDir, version } from './prepare.mjs';

export const outDir = path.join(siteDir, '.vitepress/dist');
const defaultBase = process.env.DOCS_BASE || '/spec42/';
const escape = text => String(text).replace(/[&<>"']/g, c => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' })[c]);

export function redirectPage(destination) {
  return `<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><meta http-equiv="refresh" content="0;url=${escape(destination)}"><link rel="canonical" href="${escape(destination)}"><meta name="robots" content="noindex"><title>Spec42 documentation</title></head><body><a href="${escape(destination)}">Continue to the Spec42 documentation</a></body></html>\n`;
}

/**
 * `file` (relative to the site root) to destination. `/latest/` and `/MAJOR.MINOR/` are stable
 * aliases for the newest release; `guide/` and `reference/` are the addresses of the former site.
 * `foo.html` and `foo` are both served, so a redirect file covers both spellings.
 */
export function redirects({ base = defaultBase, versions }) {
  const latest = versions[0];
  const newestPatch = new Map();
  for (const tag of versions) {
    const [major, minor] = version(tag);
    const key = `${major}.${minor}`;
    if (!newestPatch.has(key)) newestPatch.set(key, tag);
  }
  const target = (tag, route) => `${base}${tag}/${route === 'index' ? '' : route}`;
  const result = [];
  for (const { route } of pages) {
    result.push([`latest/${route}.html`, target(latest, route)]);
    for (const [minor, tag] of newestPatch) result.push([`${minor}/${route}.html`, target(tag, route)]);
  }
  result.push(
    ['guide/index.html', target(latest, 'getting-started')],
    ['reference/index.html', target(latest, 'included')],
    ['guide/getting-started.html', target(latest, 'getting-started')],
    ['reference/whats-included.html', target(latest, 'included')]
  );
  return result;
}

export function writeRedirects(dist, options) {
  const list = redirects(options);
  for (const [file, destination] of list) {
    const full = path.join(dist, file);
    fs.mkdirSync(path.dirname(full), { recursive: true });
    fs.writeFileSync(full, redirectPage(destination));
  }
  return list.length;
}

function htmlFiles(directory) {
  return fs.readdirSync(directory, { withFileTypes: true }).flatMap(entry => {
    const full = path.join(directory, entry.name);
    if (entry.isDirectory()) return entry.name === 'assets' ? [] : htmlFiles(full);
    return entry.name.endsWith('.html') ? [full] : [];
  });
}

/** Broken internal links and fragments in a built site, as readable strings. */
export function checkLinks(dist, base = defaultBase) {
  const ids = new Map();
  const idsOf = file => {
    if (!ids.has(file)) ids.set(file, new Set([...fs.readFileSync(file, 'utf8').matchAll(/\sid="([^"]+)"/g)].map(match => match[1])));
    return ids.get(file);
  };
  const problems = [];
  for (const file of htmlFiles(dist)) {
    const html = fs.readFileSync(file, 'utf8');
    if (html.includes('http-equiv="refresh"')) continue;
    for (const [, raw] of html.matchAll(/\shref="([^"]*)"/g)) {
      if (!raw.startsWith(base) && !raw.startsWith('#')) continue;
      const [location, fragment = ''] = raw.split('#');
      let target = file;
      if (location) {
        const relative = decodeURIComponent(location.slice(base.length)).split('?')[0];
        if (/\.(?:css|js|svg|json|png|jpe?g|ico|woff2?)$/.test(relative)) continue;
        const candidates = relative === '' || relative.endsWith('/')
          ? [path.join(dist, relative, 'index.html')]
          : [path.join(dist, `${relative}.html`), path.join(dist, relative), path.join(dist, relative, 'index.html')];
        target = candidates.find(candidate => fs.existsSync(candidate) && fs.statSync(candidate).isFile());
        if (!target) { problems.push(`${path.relative(dist, file)}: ${raw} does not exist`); continue; }
      }
      if (fragment && !idsOf(target).has(decodeURIComponent(fragment))) problems.push(`${path.relative(dist, file)}: ${raw} has no matching heading`);
    }
  }
  return problems;
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const manifest = JSON.parse(fs.readFileSync(manifestFile, 'utf8'));
  if (manifest.versions.length) console.log(`Wrote ${writeRedirects(outDir, { versions: manifest.versions })} redirects.`);
  const problems = checkLinks(outDir);
  if (problems.length) {
    console.error(`${problems.length} broken links:\n${problems.slice(0, 40).join('\n')}`);
    process.exit(1);
  }
  console.log('Internal links and fragments are valid.');
}
