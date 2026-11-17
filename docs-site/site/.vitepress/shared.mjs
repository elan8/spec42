// Shared by the Spec42 and Doc42 documentation sites. Both repositories carry an identical copy of
// `shared.mjs` and `theme/`, so the sites keep one header, navigation, notice, footer and style.
// Change both together; product-specific values belong in each site's `config.mjs`.

const SITE = 'https://elan8.github.io';

/** A page route (`index`, `getting-started`) as a sidebar link. */
export const pageLink = route => (route === 'index' ? '/' : `/${route}`);

/**
 * @param {object} options
 * @param {string} options.product      Display name, e.g. `Spec42`.
 * @param {string} options.slug         Elan8 product page and GitHub repository name segment.
 * @param {string} options.repo         `owner/name` of the repository that publishes releases.
 * @param {string} options.base         Site base path, e.g. `/spec42/`.
 * @param {string} options.latest       Latest stable version label, e.g. `v0.61.1`.
 * @param {string} options.description  Meta description.
 * @param {string} options.footer       Footer message.
 * @param {string} options.license      Footer license text.
 * @param {{ title: string, pages: { route: string, title: string }[] }[]} options.groups
 * @param {string | null} [options.development]  Label of the unreleased version, if one is published.
 * @param {Record<string, typeof groups>} [options.sidebarByVersion]  Groups for versions that lack some guides.
 * @param {object} [options.markdown]  VitePress markdown options merged over the shared ones.
 */
export function sharedConfig({ product, slug, repo, base, latest, description, footer, license, groups, development = null, sidebarByVersion = {}, markdown = {} }) {
  const sidebarFor = list => list.map(group => ({
    text: group.title,
    items: group.pages.map(page => ({ text: page.title, link: pageLink(page.route) }))
  }));
  return {
    title: product,
    description,
    base,
    cleanUrls: true,
    lastUpdated: false,
    head: [['link', { rel: 'icon', type: 'image/svg+xml', href: `${base}logo.svg` }]],
    // Both products author SysML in fenced blocks that no highlighter knows; show them as plain text.
    markdown: { languageAlias: { sysml: 'text', kerml: 'text', typ: 'text', typst: 'text' }, ...markdown },
    // `foo.html` is still served next to the clean `foo` address, so older links keep working;
    // the canonical link tells search engines which one to index.
    transformHead({ pageData }) {
      const clean = pageData.relativePath.replace(/(^|\/)index\.md$/, '$1').replace(/\.md$/, '');
      const head = [['link', { rel: 'canonical', href: `${SITE}${base}${clean}` }]];
      if (development && pageData.relativePath.startsWith(`${development}/`)) head.push(['meta', { name: 'robots', content: 'noindex' }]);
      return head;
    },
    themeConfig: {
      logo: '/logo.svg',
      siteTitle: `${product} docs`,
      nav: [
        { component: 'VersionSwitcher' },
        { text: 'Elan8', link: 'https://www.elan8.com' },
        { text: 'Product', link: `https://www.elan8.com/products/${slug}` },
        { text: `Download ${latest.slice(1)}`, link: `https://github.com/${repo}/releases/tag/${latest}` }
      ],
      // A version may lack guides that later releases added; it then lists only its own.
      sidebar: Object.keys(sidebarByVersion).length
        ? { root: sidebarFor(groups), ...Object.fromEntries(Object.entries(sidebarByVersion).map(([version, list]) => [version, sidebarFor(list)])) }
        : sidebarFor(groups),
      search: { provider: 'local' },
      outline: [2, 3],
      socialLinks: [{ icon: 'github', link: `https://github.com/${repo}` }],
      footer: { message: footer, copyright: `${license} · © Elan8` },
      // Read by the version notice and the source link in `theme/`. (`versioning` is the plugin's own key.)
      elan8Docs: { latest, development, repo }
    }
  };
}
