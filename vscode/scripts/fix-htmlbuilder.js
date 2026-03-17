#!/usr/bin/env node
const fs = require('fs');
const path = require('path');

const filePath = path.join(__dirname, '..', 'src', 'visualization', 'htmlBuilder.ts');
let content = fs.readFileSync(filePath, 'utf8');

// Find the first occurrence of </html>`;
const firstClose = content.indexOf('</html>`;\n');
if (firstClose === -1) {
    console.error('Could not find first template close');
    process.exit(1);
}

// The good content ends at: </html>`;\n (include the newline before next part)
const goodEnd = firstClose + '</html>`;\n'.length;

// Find the orphaned content start - right after goodEnd
// Find the last occurrence of </html>`; which closes the duplicate
const lastClose = content.lastIndexOf('\n</html>`;\n');
if (lastClose === -1) {
    console.error('Could not find last template close');
    process.exit(1);
}

// Find the closing brace of the function (last line)
const lastBrace = content.lastIndexOf('\n}');
if (lastBrace === -1) {
    console.error('Could not find function close');
    process.exit(1);
}

// Keep: start through first good close, then the function's closing brace
content = content.substring(0, goodEnd) + content.substring(lastBrace);
fs.writeFileSync(filePath, content, 'utf8');
console.log('Fixed htmlBuilder.ts - removed orphaned script content');
