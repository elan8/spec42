import * as vscode from 'vscode';
import type { SoftwareAnalysisEntry } from './softwareAnalysisStore';

export type AddonKind = 'builtIn' | 'teaser';
export type WorkspaceSupportStatus = 'supported' | 'unsupported';

export interface AddonDescriptor {
    id: string;
    name: string;
    description: string;
    kind: AddonKind;
    badge?: string;
    canToggle: boolean;
    canOpen: boolean;
    learnMoreUrl?: string;
}

export interface AddonState extends AddonDescriptor {
    enabled: boolean;
    workspaceSupport: WorkspaceSupportStatus;
    statusText: string;
    openEnabled: boolean;
    canRunAnalysis: boolean;
    runAnalysisEnabled: boolean;
}

const SOFTWARE_ARCHITECTURE_ADDON_ID = 'software-architecture';
const SOFTWARE_ARCHITECTURE_SETTING = 'addons.softwareArchitecture.enabled';

const REGISTERED_ADDONS: AddonDescriptor[] = [
    {
        id: SOFTWARE_ARCHITECTURE_ADDON_ID,
        name: 'Software Architecture',
        description: 'Extract and visualize Rust crate/module architecture in a dedicated visualizer.',
        kind: 'builtIn',
        canToggle: true,
        canOpen: true,
    },
    {
        id: 'software-architecture-pro',
        name: 'Babel42',
        description: 'Commercial add-on for conformance, drift, and deeper architecture diagnostics.',
        kind: 'teaser',
        badge: 'Pro',
        canToggle: false,
        canOpen: false,
        learnMoreUrl: 'https://github.com/elan8/spec42',
    },
];

export function getRegisteredAddons(): AddonDescriptor[] {
    return REGISTERED_ADDONS.slice();
}

export function isAddonEnabled(
    addonId: string,
    scope?: vscode.ConfigurationScope
): boolean {
    if (addonId !== SOFTWARE_ARCHITECTURE_ADDON_ID) {
        return false;
    }
    return vscode.workspace
        .getConfiguration('spec42', scope)
        .get<boolean>(SOFTWARE_ARCHITECTURE_SETTING, true);
}

export async function setAddonEnabled(
    addonId: string,
    enabled: boolean,
    target: vscode.ConfigurationTarget = vscode.ConfigurationTarget.Workspace,
): Promise<void> {
    if (addonId !== SOFTWARE_ARCHITECTURE_ADDON_ID) {
        return;
    }
    await vscode.workspace
        .getConfiguration('spec42')
        .update(SOFTWARE_ARCHITECTURE_SETTING, enabled, target);
}

export async function detectRustWorkspace(): Promise<boolean> {
    const cargo = await vscode.workspace.findFiles('**/Cargo.toml', '**/target/**', 1);
    if (cargo.length > 0) {
        return true;
    }
    const rsFiles = await vscode.workspace.findFiles('**/*.rs', '**/target/**', 1);
    return rsFiles.length > 0;
}

function buildSoftwareAddonStatusText(
    rustSupported: boolean,
    enabled: boolean,
    analysisEntry?: SoftwareAnalysisEntry,
): string {
    if (!rustSupported) {
        return 'No supported Rust workspace detected';
    }
    if (!enabled) {
        return 'Disabled';
    }
    switch (analysisEntry?.status) {
        case 'running':
            return 'Analyzing...';
        case 'ready': {
            const summary = analysisEntry.model?.summary;
            if (!summary) {
                return 'Ready';
            }
            return `Ready: ${summary.crateCount} crate(s), ${summary.moduleCount} module(s), ${summary.dependencyCount} dependenc${summary.dependencyCount === 1 ? 'y' : 'ies'}`;
        }
        case 'failed':
            return analysisEntry.errorMessage
                ? `Failed: ${analysisEntry.errorMessage}`
                : 'Failed';
        default:
            return 'Not analyzed yet';
    }
}

export async function getAddonStates(
    softwareAnalysisEntry?: SoftwareAnalysisEntry,
): Promise<AddonState[]> {
    const rustSupported = await detectRustWorkspace();
    return getRegisteredAddons().map((descriptor) => {
        if (descriptor.id === SOFTWARE_ARCHITECTURE_ADDON_ID) {
            const enabled = isAddonEnabled(descriptor.id);
            return {
                ...descriptor,
                enabled,
                workspaceSupport: rustSupported ? 'supported' : 'unsupported',
                statusText: buildSoftwareAddonStatusText(rustSupported, enabled, softwareAnalysisEntry),
                openEnabled: enabled && rustSupported && softwareAnalysisEntry?.status === 'ready',
                canRunAnalysis: true,
                runAnalysisEnabled: enabled && rustSupported && softwareAnalysisEntry?.status !== 'running',
            };
        }
        return {
            ...descriptor,
            enabled: false,
            workspaceSupport: 'unsupported',
            statusText: 'Coming soon',
            openEnabled: false,
            canRunAnalysis: false,
            runAnalysisEnabled: false,
        };
    });
}
