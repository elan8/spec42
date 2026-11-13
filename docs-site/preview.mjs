import http from 'node:http';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import documentation from '../vscode/src/help/documentation.json' with { type: 'json' };

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../target/docs-site');
const base = new URL(documentation.baseUrl).pathname;
const types = { '.html': 'text/html; charset=utf-8', '.css': 'text/css', '.js': 'text/javascript', '.json': 'application/json' };
http.createServer((request, response) => {
  const url = new URL(request.url, 'http://localhost');
  if (!url.pathname.startsWith(base)) {
    response.writeHead(302, { Location: base }).end();
    return;
  }
  const relative = decodeURIComponent(url.pathname.slice(base.length));
  const file = path.resolve(root, relative.endsWith('/') || !relative ? `${relative}index.html` : relative);
  if (!file.startsWith(`${root}${path.sep}`)) { response.writeHead(403).end(); return; }
  try {
    response.writeHead(200, { 'Content-Type': types[path.extname(file)] ?? 'application/octet-stream' });
    response.end(fs.readFileSync(file));
  } catch { response.writeHead(404).end('Page not found'); }
}).listen(4173, '127.0.0.1', () => console.log(`Documentation preview: http://127.0.0.1:4173${base}`));
