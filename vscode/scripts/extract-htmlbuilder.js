#!/usr/bin/env node
const fs = require('fs');
const path = require('path');

const srcPath = path.join(__dirname, '..', 'src', 'visualization', 'visualizationPanel.ts');
const content = fs.readFileSync(srcPath, 'utf8');

// Find the full _getHtmlForWebview method - from "private _getHtmlForWebview" to the closing }; of the class method
// The method ends with }; before the next "    private" or "    public" or "}"
const methodStart = content.indexOf('    private _getHtmlForWebview(webview: vscode.Webview, extensionUri: vscode.Uri): string {');
if (methodStart === -1) {
    console.error('Could not find _getHtmlForWebview');
    process.exit(1);
}

// Find the end of the method - it ends with `};  (template + closing)
// Actually the method is:   private _getHtmlForWebview(...) { ... return `...`; }
// So we need to find the closing brace. The template ends with </html>`; 
// Then we have just } to close the method. Let me find </html>`; and take everything up to and including the };
const templateEnd = content.indexOf('</html>`;', methodStart);
if (templateEnd === -1) {
    console.error('Could not find template end');
    process.exit(1);
}
const closingBrace = content.indexOf('\n    }', templateEnd);
if (closingBrace === -1) {
    console.error('Could not find method closing brace');
    process.exit(1);
}
const methodEnd = closingBrace + 5; // include '\n    }'
const methodBody = content.substring(methodStart, methodEnd);

// Convert to standalone function: replace "private _getHtmlForWebview(...)" with "export function getWebviewHtml(...)"
// and "_getNonce()" with "getNonce()"
let converted = methodBody
    .replace('    private _getHtmlForWebview(webview: vscode.Webview, extensionUri: vscode.Uri): string {', 'export function getWebviewHtml(webview: vscode.Webview, extensionUri: vscode.Uri): string {')
    .replace(/const nonce = _getNonce\(\);/g, 'const nonce = getNonce();');

// Add getNonce and fix indentation (remove leading 4 spaces from the class method to get function body)
converted = converted.replace(/^    /gm, '');

const header = `import * as vscode from 'vscode';

function getNonce(): string {
    const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    let nonce = '';
    for (let i = 0; i < 32; i++) {
        nonce += chars.charAt(Math.floor(Math.random() * chars.length));
    }
    return nonce;
}

`;

const fullOutput = header + converted;
const outPath = path.join(__dirname, '..', 'src', 'visualization', 'htmlBuilder.ts');
fs.writeFileSync(outPath, fullOutput, 'utf8');
console.log('Wrote htmlBuilder.ts, size:', fullOutput.length);

// Remove _getHtmlForWebview and _getNonce from visualizationPanel
const vpPath = path.join(__dirname, '..', 'src', 'visualization', 'visualizationPanel.ts');
let vpContent = fs.readFileSync(vpPath, 'utf8');
const removeStart = vpContent.indexOf('\n\n    private _getHtmlForWebview(webview: vscode.Webview, extensionUri: vscode.Uri): string {');
const removeEnd = vpContent.indexOf('\n/** Generate a random nonce for Content Security Policy. */');
if (removeStart !== -1 && removeEnd !== -1) {
    vpContent = vpContent.substring(0, removeStart) + vpContent.substring(removeEnd);
    // Also remove the _getNonce function
    const nonceStart = vpContent.indexOf('function _getNonce(): string {');
    const nonceEnd = vpContent.indexOf('\n}', nonceStart) + 2;
    if (nonceStart !== -1 && nonceEnd > nonceStart) {
        vpContent = vpContent.substring(0, nonceStart).trimEnd() + '\n' + vpContent.substring(nonceEnd);
    }
    fs.writeFileSync(vpPath, vpContent, 'utf8');
    console.log('Removed _getHtmlForWebview and _getNonce from visualizationPanel.ts');
}
