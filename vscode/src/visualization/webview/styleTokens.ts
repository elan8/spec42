/**
 * Shared visual style tokens for all diagram views.
 * Keep all renderer colors in one place for consistent theming.
 */

export const DIAGRAM_STYLE = {
    canvasBackground: 'var(--vscode-editor-background)',
    panelBackground: 'var(--vscode-button-secondaryBackground)',
    nodeFill: 'var(--vscode-editor-background)',
    nodeBorder: '#E5E7EB',
    textPrimary: 'var(--vscode-editor-foreground)',
    textSecondary: 'var(--vscode-descriptionForeground)',
    edgePrimary: '#2F6FDD',
    edgeSecondary: '#2F6FDD',
    edgeSuccess: '#2F6FDD',
    edgeWarning: '#2F6FDD',
    edgeDanger: '#2F6FDD',
    highlight: '#FFD700',
} as const;

