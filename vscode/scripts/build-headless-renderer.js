#!/usr/bin/env node
const esbuild = require('esbuild');
const fs = require('fs');
const path = require('path');

const rootDir = path.join(__dirname, '..');
const repoRoot = path.join(rootDir, '..');
const entryPoint = path.join(repoRoot, 'shared', 'diagram-renderer', 'src', 'headless-export.ts');
const outFile = path.join(repoRoot, 'crates', 'server', 'assets', 'diagram-renderer', 'headless-renderer.js');

async function build() {
    try {
        fs.mkdirSync(path.dirname(outFile), { recursive: true });
        await esbuild.build({
            entryPoints: [entryPoint],
            bundle: true,
            outfile: outFile,
            format: 'iife',
            platform: 'browser',
            target: 'es2020',
            sourcemap: false,
            minify: false,
            globalName: 'Spec42HeadlessRendererBundle',
            define: {
                'process.env.NODE_ENV': '"production"',
            },
            alias: {
                'elkjs/lib/elk.bundled.js': path.join(
                    repoRoot,
                    'shared',
                    'diagram-renderer',
                    'src',
                    'headless-elk-shim.ts',
                ),
            },
        });
        console.log(`Headless renderer bundle written to ${outFile}`);
    } catch (err) {
        console.error('Headless renderer build failed:', err);
        process.exit(1);
    }
}

build();
