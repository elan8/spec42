#!/usr/bin/env node
const esbuild = require('esbuild');
const path = require('path');

const rootDir = path.join(__dirname, '..');
const entryPoint = path.join(rootDir, 'src', 'visualization', 'webview', 'index.ts');
const outFile = path.join(rootDir, 'media', 'webview', 'visualizer.js');

async function build() {
    try {
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
            // d3, elk are loaded via <script> tags before this bundle; webview code uses them as globals
        });
        console.log(`Webview bundle written to ${outFile}`);
    } catch (err) {
        console.error('Webview build failed:', err);
        process.exit(1);
    }
}

build();
