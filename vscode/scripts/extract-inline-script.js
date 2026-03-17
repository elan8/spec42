#!/usr/bin/env node
/**
 * Extracts the inline script from htmlBuilder.ts into webview/legacyBundle.ts.
 * Run when htmlBuilder's inline script changes.
 * The bundle will use this; elkWorkerUrl is read from window.__VIZ_INIT (set by a tiny inline script in HTML).
 */
const fs = require('fs');
const path = require('path');

const htmlBuilderPath = path.join(__dirname, '..', 'src', 'visualization', 'htmlBuilder.ts');
const content = fs.readFileSync(htmlBuilderPath, 'utf8');

const scriptStart = content.indexOf('<script nonce="${nonce}">\n');
const scriptEnd = content.indexOf('\n</script>', scriptStart);
if (scriptStart === -1 || scriptEnd === -1) {
    console.error('Could not find inline script block');
    process.exit(1);
}

const scriptContent = content.substring(scriptStart + '<script nonce="${nonce}">\n'.length, scriptEnd);

// Replace template variable with runtime config
const fixed = scriptContent.replace(
    "const elkWorkerUrl = '${elkWorkerUri}';",
    "const elkWorkerUrl = (typeof window !== 'undefined' && (window).__VIZ_INIT?.elkWorkerUrl) ?? '';"
);

const header = `/* eslint-disable @typescript-eslint/no-explicit-any */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */
// Legacy bundle: full visualizer logic extracted from htmlBuilder inline script.
// Config (elkWorkerUrl) is set by a minimal inline script in HTML before this bundle loads.

declare const acquireVsCodeApi: () => { postMessage: (msg: unknown) => void };

`;

const outPath = path.join(__dirname, '..', 'src', 'visualization', 'webview', 'legacyBundle.ts');
fs.writeFileSync(outPath, header + fixed, 'utf8');
console.log('Wrote legacyBundle.ts, size:', (header.length + fixed.length), 'chars');
