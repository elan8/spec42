#!/usr/bin/env node
const fs = require('fs');
const path = require('path');

/**
 * Simple helper that copies the visualization vendor bundles from node_modules
 * into media/vendor so the webview can load them via local URIs.
 */
function ensureDir(dir) {
    if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
    }
}

function copyAsset(sourceModulePath, targetFileName, targetSubdir = 'vendor') {
    const targetDir = path.join(__dirname, '..', 'media', targetSubdir);
    ensureDir(targetDir);

    const targetPath = path.join(targetDir, targetFileName);
    fs.copyFileSync(sourceModulePath, targetPath);
    console.log(`Copied ${sourceModulePath} -> ${targetPath}`);
}

function resolvePackageRoot(packageName) {
    try {
        const entryPoint = require.resolve(packageName);
        let dir = path.dirname(entryPoint);
        const { root } = path.parse(dir);
        while (dir !== root) {
            if (fs.existsSync(path.join(dir, 'package.json'))) {
                return dir;
            }
            dir = path.dirname(dir);
        }
    } catch (error) {
        console.error(`Unable to find entry point for ${packageName}`);
        throw error;
    }
    throw new Error(`Could not locate package.json for ${packageName}`);
}

function resolvePackageFile(packageName, relativePath) {
    const packageDir = resolvePackageRoot(packageName);
    return path.join(packageDir, relativePath);
}

function run() {
    const codiconsDir = path.join(__dirname, '..', 'media', 'codicons');
    ensureDir(codiconsDir);
    const codiconsPkg = path.join(__dirname, '..', 'node_modules', '@vscode', 'codicons');
    const codiconsDist = path.join(codiconsPkg, 'dist');
    if (fs.existsSync(path.join(codiconsDist, 'codicon.css'))) {
        fs.copyFileSync(path.join(codiconsDist, 'codicon.css'), path.join(codiconsDir, 'codicon.css'));
        fs.copyFileSync(path.join(codiconsDist, 'codicon.ttf'), path.join(codiconsDir, 'codicon.ttf'));
        console.log('Copied @vscode/codicons to media/codicons/');
    } else {
        console.warn('@vscode/codicons dist not found - codicons will not be available in webview');
    }

    const assets = [
        { packageName: 'd3', relativePath: 'dist/d3.min.js', filename: 'd3.min.js', subdir: 'vendor' },
        { packageName: 'elkjs', relativePath: 'lib/elk.bundled.js', filename: 'elk.bundled.js', subdir: 'vendor' },
        { packageName: 'elkjs', relativePath: 'lib/elk-worker.min.js', filename: 'elkWorker.js', subdir: 'webview' }
    ];

    assets.forEach(asset => {
        try {
            const resolvedPath = resolvePackageFile(asset.packageName, asset.relativePath);
            copyAsset(resolvedPath, asset.filename, asset.subdir || 'vendor');
        } catch (error) {
            console.error(`Failed to resolve ${asset.packageName}/${asset.relativePath}.`);
            throw error;
        }
    });

    console.log('Webview vendor assets copied successfully.');
}

run();
