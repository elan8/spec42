import documentation from "./documentation.json";

export type DocumentationPage = keyof typeof documentation.pages;

/** Link to the installed product version, or explicitly unreleased documentation. */
export function documentationUrl(version: string, page: DocumentationPage, development: boolean): string {
  const match = /^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$/.exec(version);
  if (!match) throw new Error(`Invalid extension version: ${version}`);
  const edition = development || match[4] ? "dev" : `v${match[1]}.${match[2]}.${match[3]}`;
  return `${documentation.baseUrl}${edition}/${documentation.pages[page].route}.html`;
}
