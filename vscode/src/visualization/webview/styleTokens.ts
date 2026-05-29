/**
 * Shared visual style tokens for all diagram views.
 * Notation-neutral: one ink color; markers and dashes convey edge kind.
 */

export const DIAGRAM_STYLE = {
    canvasBackground: 'var(--vscode-editor-background)',
    panelBackground: 'var(--vscode-button-secondaryBackground)',
    nodeFill: 'var(--vscode-editor-background)',
    nodeBorder: 'var(--vscode-editor-foreground)',
    textPrimary: 'var(--vscode-editor-foreground)',
    textSecondary: 'var(--vscode-descriptionForeground)',
    edgePrimary: 'var(--vscode-editor-foreground)',
    edgeSecondary: 'var(--vscode-editor-foreground)',
    edgeSuccess: 'var(--vscode-editor-foreground)',
    edgeWarning: 'var(--vscode-editor-foreground)',
    edgeDanger: 'var(--vscode-editor-foreground)',
    highlight: 'var(--vscode-focusBorder, #d97706)',
} as const;
