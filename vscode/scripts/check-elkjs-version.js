#!/usr/bin/env node
// Guards against the CLI's hand-vendored `crates/server/assets/elkjs/*` silently drifting from
// whatever elkjs version an actual build resolves at runtime (webview via npm, headless via
// shared/diagram-renderer's install). See docs/VIEW-RENDERING-ISSUES.md O-6-adjacent Phase 2 note:
// these were found to have drifted once already (installed 0.8.2 vs lockfile-pinned ^0.11.1).
const fs = require('fs');
const path = require('path');

const repoRoot = path.join(__dirname, '..', '..');
const vendoredVersionFile = path.join(repoRoot, 'crates', 'server', 'assets', 'elkjs', 'VENDORED_VERSION');

function vendoredVersion() {
    return fs.readFileSync(vendoredVersionFile, 'utf8').trim();
}

function installedVersion(elkjsPackageJsonPath) {
    if (!fs.existsSync(elkjsPackageJsonPath)) return null;
    const pkg = JSON.parse(fs.readFileSync(elkjsPackageJsonPath, 'utf8'));
    return pkg.version;
}

/**
 * Fails the build if the elkjs version resolved at `elkjsPackageJsonPath` doesn't match the
 * version `crates/server/assets/elkjs/*` was vendored from — the CLI/headless-export path and the
 * checked build must render with the same layout engine for CLI output to stay representative.
 */
function assertElkjsVersionMatchesVendored(elkjsPackageJsonPath, label) {
    const vendored = vendoredVersion();
    const installed = installedVersion(elkjsPackageJsonPath);
    if (installed === null) {
        console.warn(`[check-elkjs-version] ${label}: elkjs not found at ${elkjsPackageJsonPath}, skipping check.`);
        return;
    }
    if (installed !== vendored) {
        console.error(
            `[check-elkjs-version] elkjs version mismatch for ${label}: installed ${installed}, ` +
                `but crates/server/assets/elkjs/* was vendored from ${vendored}. The CLI/headless SVG ` +
                `output and this build's layout would no longer be guaranteed representative of each ` +
                `other. Fix by reinstalling (npm ci) so this build resolves elkjs@${vendored}, or by ` +
                `re-vendoring crates/server/assets/elkjs/{elk-api.js,elk-worker.min.js} from the newly ` +
                `installed version and updating VENDORED_VERSION.`,
        );
        process.exit(1);
    }
}

module.exports = { assertElkjsVersionMatchesVendored, vendoredVersion };
