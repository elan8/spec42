// The workspace server admits filesystem models. Chat edit histories, code blocks and
// revision previews are separate documents, not additional workspace source revisions.
export const MODEL_DOCUMENT_SELECTOR = [
  { language: "sysml", scheme: "file" },
  { language: "kerml", scheme: "file" },
];
