import * as path from 'path';
import * as vscode from 'vscode';
import { getOutputChannel, isVerboseLoggingEnabled, logPerfEvent } from '../logger';
import { LspModelProvider, toVscodeRange } from '../providers/lspModelProvider';
import type { SysMLElement } from '../types/sysmlTypes';

export interface MessageHandlerContext {
    panel: vscode.WebviewPanel;
    document: vscode.TextDocument;
    workspaceRootUri: string;
    lspModelProvider: LspModelProvider;
    updateVisualization: (force: boolean, triggerSource?: string) => void;
    setNavigating: (value: boolean) => void;
    setCurrentView: (view: string) => void;
    setSelectedView: (value?: string) => void;
    setLastContentHash: (hash: string) => void;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type WebviewMessage = { command: string; [key: string]: any };

export function createMessageDispatcher(ctx: MessageHandlerContext): (msg: WebviewMessage) => void {
    const handlers = createMessageHandlers(ctx);
    const { panel, document, setCurrentView, setSelectedView, setLastContentHash, updateVisualization } = ctx;

    return (message: WebviewMessage) => {
        switch (message.command) {
            case 'webviewLog':
                if (message.level === 'error' || message.level === 'warn' || isVerboseLoggingEnabled()) {
                    handlers.logWebviewMessage(message.level, message.args ?? []);
                }
                break;
            case 'webviewPerf':
                if (typeof message.event === 'string') {
                    logPerfEvent(message.event, {
                        source: 'webview',
                        ...(message.data ?? {}),
                    });
                }
                break;
            case 'jumpToElement':
                handlers.jumpToElement(message.elementName, message.skipCentering, message.parentContext, message.elementQualifiedName, message.elementUri);
                break;
            case 'renameElement':
                handlers.renameElement(message.oldName, message.newName);
                break;
            case 'export':
                handlers.handleExport(message.format, message.data);
                break;
            case 'executeCommand':
                if (message.args && message.args.length > 0) {
                    const cmd = message.args[0];
                    const allowedCommands: string[] = [];
                    if (!allowedCommands.includes(cmd)) {
                        // eslint-disable-next-line no-console
                        console.warn(`[SysML Visualizer] Blocked disallowed command: ${cmd}`);
                        break;
                    }
                    if (cmd === 'sysml.showModelDashboard') {
                        const dashboardUri = document.uri;
                        setTimeout(() => vscode.commands.executeCommand(cmd, dashboardUri), 100);
                    } else {
                        const cmdArgs = message.args.slice(1);
                        setTimeout(() => vscode.commands.executeCommand(cmd, ...cmdArgs), 100);
                    }
                }
                break;
            case 'viewChanged':
                setCurrentView(message.view ?? '');
                setLastContentHash('');
                updateVisualization(true, 'viewChanged');
                break;
            case 'viewSelectionChanged':
                if (typeof message.rendererView === 'string' && message.rendererView.length > 0) {
                    setCurrentView(message.rendererView);
                }
                setSelectedView(message.viewId || undefined);
                setLastContentHash('');
                updateVisualization(true, 'viewSelectionChanged');
                break;
            case 'clearSelectedView':
                setSelectedView(undefined);
                setLastContentHash('');
                updateVisualization(true, 'clearSelectedView');
                break;
            case 'openExternal':
                if (message.url) {
                    vscode.env.openExternal(vscode.Uri.parse(message.url));
                }
                break;
            case 'currentViewResponse':
                setCurrentView(message.view ?? '');
                break;
            case 'webviewReady':
                setLastContentHash('');
                updateVisualization(true, 'webviewReady');
                break;
            case 'testDiagramExported':
                handleTestDiagramExported(message.viewId, message.svgString).catch(err =>
                    // eslint-disable-next-line no-console
                    console.error('[SysML Visualizer] Failed to write test diagram:', err)
                );
                break;
        }
    };
}

async function handleTestDiagramExported(viewId: string, svgString: string): Promise<void> {
    const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
    if (!workspaceFolder) return;

    const outputDir = getTestDiagramOutputDir(workspaceFolder);
    await vscode.workspace.fs.createDirectory(outputDir);

    const safeViewId = (viewId || 'unknown').replace(/[^a-zA-Z0-9_-]/g, '_');
    const fileUri = vscode.Uri.joinPath(outputDir, `${safeViewId}.svg`);
    await vscode.workspace.fs.writeFile(fileUri, Buffer.from(svgString, 'utf8'));
}

function getTestDiagramOutputDir(workspaceFolder: vscode.WorkspaceFolder): vscode.Uri {
    const configuredDir = (process.env.SPEC42_TEST_EXPORT_DIR || '').trim();
    if (!configuredDir) {
        return vscode.Uri.joinPath(workspaceFolder.uri, 'test-output', 'diagrams');
    }

    const safeWorkspaceName = workspaceFolder.name.replace(/[^a-zA-Z0-9_-]/g, '_');
    return vscode.Uri.file(path.join(configuredDir, safeWorkspaceName));
}

export function createMessageHandlers(context: MessageHandlerContext) {
    const { panel, document, workspaceRootUri, lspModelProvider, updateVisualization, setNavigating } = context;
    let activeHighlightDecoration: vscode.TextEditorDecorationType | undefined;
    let activeHighlightTimeout: ReturnType<typeof setTimeout> | undefined;
    let activeHighlightEditor: vscode.TextEditor | undefined;

    function clearActiveSourceHighlight(): void {
        const trackedUris = new Set([document.uri.toString()]);

        if (activeHighlightTimeout) {
            clearTimeout(activeHighlightTimeout);
            activeHighlightTimeout = undefined;
        }
        vscode.window.visibleTextEditors.forEach((editor) => {
            if (!trackedUris.has(editor.document.uri.toString())) {
                return;
            }
            const caret = editor.selection.active;
            editor.selection = new vscode.Selection(caret, caret);
        });
        if (activeHighlightDecoration && activeHighlightEditor) {
            activeHighlightEditor.setDecorations(activeHighlightDecoration, []);
        }
        if (activeHighlightEditor) {
            const anchor = activeHighlightEditor.selection.active;
            activeHighlightEditor.selection = new vscode.Selection(anchor, anchor);
        }
        if (activeHighlightDecoration) {
            activeHighlightDecoration.dispose();
            activeHighlightDecoration = undefined;
        }
        activeHighlightEditor = undefined;
    }

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    function logWebviewMessage(level: string, args: any[]) {
        try {
            const outputChannel = getOutputChannel();
            if (!outputChannel) {
                return;
            }

            const formattedArgs = args.map(arg => {
                if (typeof arg === 'object') {
                    try {
                        return JSON.stringify(arg, null, 2);
                    } catch {
                        return String(arg);
                    }
                }
                return String(arg);
            }).join(' ');

            const prefix = level === 'error' ? '❌' : level === 'warn' ? '⚠️' : 'ℹ️';
            outputChannel.appendLine(`[Webview ${level.toUpperCase()}] ${prefix} ${formattedArgs}`);
        } catch (error) {
            // eslint-disable-next-line no-console
            console.error('Failed to log webview message:', error);
        }
    }

    async function jumpToElement(elementName: string, skipCentering: boolean = false, parentContext?: string, elementQualifiedName?: string, elementUri?: string) {
        setNavigating(true);

        let element: SysMLElement | undefined;
        let resolvedUri = elementUri || document.uri.toString();

        const dto = await lspModelProvider.findElement(
            resolvedUri || workspaceRootUri,
            elementName,
            parentContext,
            elementQualifiedName,
        );
        if (dto) {
            resolvedUri = dto.uri || resolvedUri;
            element = {
                type: dto.type,
                name: dto.name,
                range: toVscodeRange(dto.range),
                children: [],
                attributes: new Map(),
                relationships: [],
            };
        }

        if (element) {
            const visualizerColumn = panel.viewColumn || vscode.ViewColumn.Two;
            const targetColumn = visualizerColumn === vscode.ViewColumn.One
                ? vscode.ViewColumn.Two
                : vscode.ViewColumn.One;

            const targetDocument = await vscode.workspace.openTextDocument(vscode.Uri.parse(resolvedUri));
            vscode.window.showTextDocument(targetDocument, {
                viewColumn: targetColumn,
                preserveFocus: true,
                preview: false,
            }).then(editor => {
                clearActiveSourceHighlight();

                editor.selection = new vscode.Selection(element!.range.start, element!.range.start);
                editor.revealRange(element!.range, vscode.TextEditorRevealType.InCenter);

                activeHighlightDecoration = vscode.window.createTextEditorDecorationType({
                    backgroundColor: 'rgba(255, 215, 0, 0.4)',
                    border: '2px solid #FFD700',
                    borderRadius: '3px',
                    isWholeLine: false,
                    rangeBehavior: vscode.DecorationRangeBehavior.ClosedClosed,
                });

                activeHighlightEditor = editor;
                editor.setDecorations(activeHighlightDecoration, [element!.range]);

                activeHighlightTimeout = setTimeout(() => {
                    clearActiveSourceHighlight();
                }, 3000);

                if (!skipCentering) {
                    panel.webview.postMessage({
                        command: 'highlightElement',
                        elementName: elementName,
                        skipCentering: skipCentering,
                    });
                }

                setTimeout(() => setNavigating(false), 500);
            });
        } else {
            vscode.window.showInformationMessage(`Element "${elementName}" not found in the current document.`);
            setNavigating(false);
        }
    }

    function escapeRegex(string: string): string {
        return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    }

    async function renameElement(oldName: string, newName: string) {
        if (!newName || newName === oldName) {
            return;
        }

        if (!/^[a-zA-Z_][a-zA-Z0-9_]*$/.test(newName)) {
            vscode.window.showErrorMessage(`Invalid element name: "${newName}". Names must start with a letter or underscore and contain only alphanumeric characters and underscores.`);
            updateVisualization(true, 'renameElement:invalidName');
            return;
        }

        let element: SysMLElement | undefined;
        const dto = await lspModelProvider.findElement(document.uri.toString(), oldName);
        if (dto) {
            element = {
                type: dto.type,
                name: dto.name,
                range: toVscodeRange(dto.range),
                children: [],
                attributes: new Map(),
                relationships: [],
            };
        }

        if (!element || !element.range) {
            vscode.window.showErrorMessage(`Could not find element "${oldName}" to rename.`);
            updateVisualization(true, 'renameElement:notFound');
            return;
        }

        const text = document.getText();
        const elementStartOffset = document.offsetAt(element.range.start);
        const elementEndOffset = document.offsetAt(element.range.end);
        const elementText = text.substring(elementStartOffset, elementEndOffset);

        const namePattern = new RegExp(`\\b${escapeRegex(oldName)}\\b`);
        const nameMatch = elementText.match(namePattern);

        if (!nameMatch || nameMatch.index === undefined) {
            vscode.window.showErrorMessage(`Could not locate name "${oldName}" in the element definition.`);
            updateVisualization(true, 'renameElement:matchMissing');
            return;
        }

        const nameStartOffset = elementStartOffset + nameMatch.index;
        const nameEndOffset = nameStartOffset + oldName.length;
        const nameRange = new vscode.Range(
            document.positionAt(nameStartOffset),
            document.positionAt(nameEndOffset),
        );

        const edit = new vscode.WorkspaceEdit();
        edit.replace(document.uri, nameRange, newName);

        const success = await vscode.workspace.applyEdit(edit);

        if (success) {
            await document.save();
            vscode.window.showInformationMessage(`Renamed "${oldName}" to "${newName}"`);
        } else {
            vscode.window.showErrorMessage(`Failed to rename "${oldName}"`);
            updateVisualization(true, 'renameElement:applyFailed');
        }
    }

    async function handleExport(format: string, data: string) {
        const filters: { [key: string]: string[] } = {
            'PNG Images': ['png'],
            'SVG Images': ['svg'],
            'JSON Files': ['json'],
        };

        let defaultFolder: vscode.Uri | undefined;
        if (document?.uri?.scheme === 'file' && document.uri.fsPath) {
            defaultFolder = vscode.Uri.file(path.dirname(document.uri.fsPath));
        }
        if (!defaultFolder) {
            const activeEditor = vscode.window.activeTextEditor;
            if (activeEditor?.document.uri.scheme === 'file') {
                defaultFolder = vscode.Uri.file(path.dirname(activeEditor.document.uri.fsPath));
            }
        }
        if (!defaultFolder && vscode.workspace.workspaceFolders?.length) {
            defaultFolder = vscode.workspace.workspaceFolders[0].uri;
        }

        const defaultUri = defaultFolder
            ? vscode.Uri.joinPath(defaultFolder, `sysml-model.${format}`)
            : vscode.Uri.file(`sysml-model.${format}`);

        const uri = await vscode.window.showSaveDialog({
            filters: filters,
            defaultUri: defaultUri,
        });

        if (uri) {
            let buffer: Buffer;

            if (format === 'json') {
                if (data.startsWith('data:')) {
                    buffer = Buffer.from(data.split(',')[1], 'base64');
                } else {
                    buffer = Buffer.from(data, 'utf8');
                }
            } else {
                if (data.startsWith('data:')) {
                    buffer = Buffer.from(data.split(',')[1], 'base64');
                } else {
                    buffer = Buffer.from(data, 'utf8');
                }
            }

            await vscode.workspace.fs.writeFile(uri, buffer);
            vscode.window.showInformationMessage(`Visualization exported to ${uri.fsPath}`);
        }
    }

    return {
        logWebviewMessage,
        jumpToElement,
        renameElement,
        handleExport,
    };
}
