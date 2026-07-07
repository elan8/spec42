#!/usr/bin/env node
const esbuild = require('esbuild');
const path = require('path');
const { assertElkjsVersionMatchesVendored } = require('./check-elkjs-version');

const rootDir = path.join(__dirname, '..');
const entryPoint = path.join(rootDir, 'src', 'visualization', 'webview', 'index.ts');
const outFile = path.join(rootDir, 'media', 'webview', 'visualizer.js');

async function build() {
    try {
        assertElkjsVersionMatchesVendored(
            path.join(rootDir, 'node_modules', 'elkjs', 'package.json'),
            'vscode webview build',
        );
        await esbuild.build({
            entryPoints: [entryPoint],
            bundle: true,
            outfile: outFile,
            format: 'iife',
            platform: 'browser',
            target: 'es2020',
            sourcemap: true,
            minify: false,
            define: {
                'process.env.NODE_ENV': '"production"',
            },
            alias: {
                '@spec42/diagram-renderer/prepare': path.join(
                    rootDir,
                    '..',
                    'shared',
                    'diagram-renderer',
                    'src',
                    'prepare.ts',
                ),
                '@spec42/diagram-renderer/renderer': path.join(
                    rootDir,
                    '..',
                    'shared',
                    'diagram-renderer',
                    'src',
                    'renderer.ts',
                ),
                '@spec42/diagram-renderer/behavior-interaction': path.join(
                    rootDir,
                    '..',
                    'shared',
                    'diagram-renderer',
                    'src',
                    'views',
                    'behavior-interaction.ts',
                ),
                // Use the same d3 instance as orchestrator (script tag), not a second bundled copy.
                d3: path.join(rootDir, 'src', 'visualization', 'webview', 'd3-global.ts'),
                // diagram-renderer imports elkjs; resolve via vscode's dependency (CI only runs npm ci here).
                'elkjs/lib/elk.bundled.js': path.join(
                    rootDir,
                    'node_modules',
                    'elkjs',
                    'lib',
                    'elk.bundled.js',
                ),
            },
        });
        console.log(`Webview bundle written to ${outFile}`);
    } catch (err) {
        console.error('Webview build failed:', err);
        process.exit(1);
    }
}

build();
