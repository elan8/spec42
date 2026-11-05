// Generated from the Rust diagram presentation protocol. Do not edit.
export type DiagramRangeChoice = { startLine: number, startCharacter: number, endLine: number, endCharacter: number, };
export type DiagramReferenceChoice = { "kind": "qualified-name", document: string, qualifiedName: string, sourceDomain: string, } | { "kind": "tooling-element-id", elementId: string, sourceDomain: string, } | { "kind": "source-anchor", document: string, ownerQualifiedName: string | null, metaclass: string, sourceDomain: string, range: DiagramRangeChoice, } | { "kind": "relationship", document: string, sourceQualifiedName: string, relationshipKind: string, ordinal: number, sourceDomain: string, };
export type DiagramViewSourceChoice = { uri: string, range: DiagramRangeChoice, };
export type DiagramViewChoice = { handle: string, kind: import('./diagram-product/ViewKind').ViewKind, reference: DiagramReferenceChoice, name: string, namespace: string | null, source: DiagramViewSourceChoice, };
export type DiagramSelection = { kind: import('./diagram-product/ViewKind').ViewKind, reference: DiagramReferenceChoice, };
export type KnownDiagramProduct = { modelDigest: string, selection: DiagramSelection, expanded: Array<string>, };
export type DiagramPresentationParams = {
/**
 * A project context URI; it need not name an admitted or open source document.
 */
projectUri: string, selection: DiagramSelection | null, preferredDocumentUri: string | null, expanded: Array<string>, knownProduct: KnownDiagramProduct | null, };
export type DiagramPresentationResult = { "state": "updating", semanticStateVersion: string, } | { "state": "cancelled", semanticStateVersion: string, } | { "state": "failed", semanticStateVersion: string, message: string, } | { "state": "unavailable", message: string, } | { "state": "superseded" } | { "state": "unchanged", semanticStateVersion: string, } | { "state": "empty", semanticStateVersion: string, modelDigest: string, } | { "state": "ready", semanticStateVersion: string, modelDigest: string, views: Array<DiagramViewChoice>, selectedHandle: string, selection: DiagramSelection, productJson: string, expanded: Array<string>, durationUs: number, };
export const DIAGRAM_PRESENTATION_METHOD = "spec42/diagramPresentation" as const;
