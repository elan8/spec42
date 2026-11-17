# Versioned user documentation

The public guide is hosted at <https://elan8.github.io/spec42/>. GitHub Pages must use
**GitHub Actions** as its build/deployment source. `.github/workflows/docs.yml` builds and
publishes the site; configure that repository setting once when adopting this workflow.

The site is a [VitePress](https://vitepress.dev/) project that serves every release with the
[`@viteplus/versions`](https://viteplus.github.io/versions/) plugin. It shares its theme, header,
navigation, version notice and footer with the Doc42 documentation site
(<https://elan8.github.io/doc42-release/>), so the two read as one product family.

## Layout

```text
docs-site/
├── overview.md            common navigation overview (the home page of every version)
├── scripts/prepare.mjs    materializes each release's guides into site/src and site/archive
├── scripts/postbuild.mjs  writes the alias redirects, then checks every link and fragment
├── tests/                 version selection, link rewriting and link validation
└── site/                  the VitePress root
    ├── .vitepress/shared.mjs  shared with Doc42: header, nav, footer, search, canonical links
    ├── .vitepress/theme/      shared with Doc42: brand CSS, layout, version notice, source link
    ├── .vitepress/config.mjs  Spec42's own values: groups, repository, base path
    ├── public/logo.svg
    ├── src/               generated: the unreleased guide, served at /dev/
    └── archive/<tag>/     generated: the guide at each stable tag, served at /<tag>/
```

`shared.mjs` and `theme/` are identical in both repositories. Change them in both, and put
product-specific values in `config.mjs`.

## Author and preview

Edit `docs/user/`, `docs/reference/` and the generated references' authoritative inputs.
`vscode/src/help/documentation.json` owns the public base URL and page routes shared by the
website and extension Help links, and groups the user-facing pages by task (the sidebar groups).
Production extensions link to their exact release; development and prerelease builds use Development.
The embedded quick reference remains offline, with a link to that same version of the guide.
`overview.md` is the common navigation overview; keep it free of release-specific feature claims.
Architecture and contributor material stays on GitHub, linked at the selected release tag. A guide
absent from an older tag gets an explicit "does not include this guide" page and no sidebar entry
in that version.

From this directory:

```sh
npm ci --ignore-scripts
npm test
npm run build      # every release plus Development, then the link and fragment check
npm run preview    # serves the build
npm run dev        # Development only, with hot reload
```

Open <http://127.0.0.1:4173/spec42/>. Node 24 and a full Git checkout with tags are required.
Output goes to the ignored `site/.vitepress/dist/`. The local build includes stable tags from
0.60.0 onward; production uses the GitHub API's published, non-draft, non-prerelease release list.
Earlier documentation remains available in the repository's tags.

The build fails on a dead link between guides (VitePress) and on any internal link or fragment
that does not resolve in the built site (`scripts/postbuild.mjs`). External URLs and fragments
on GitHub source pages are not fetched. Search is local to each version. The guides, navigation
and version permalink remain readable without JavaScript; search and the version selector need it.

## URLs

Pages are served at clean addresses (`/v0.61.1/getting-started`). Each page's `.html` file is
published too, so `/v0.61.1/getting-started.html`, the form the extension's Help links use, keeps
working; every page names the clean address as its canonical URL. `/latest/…` and `/MAJOR.MINOR/…`
redirect to the newest release, and the addresses of the former site (`/guide/`, `/reference/`)
redirect to the matching guide.

## Release policy

- Every published stable major, minor **and patch** release from 0.60.0 has a permanent
  `/vMAJOR.MINOR.PATCH/` guide, built from that exact tag's Markdown.
- `/MAJOR.MINOR/` redirects to the newest published patch of that line. Every release
  stays in the version selector.
- `/latest/` and the site root point to the numerically newest stable release. Publishing a
  maintenance patch for an older line does not move Latest backwards.
- `/dev/` uses the main branch, shows an unreleased notice and is excluded from search engines.
- Guide content comes from its tag; the common layout, navigation overview and build tooling
  come from main. Fixing a released guide requires publishing a patch release.

Documentation is built and link-checked on pull requests. Main-branch documentation changes
publish Development while preserving the tagged guides. After **every successful Release
workflow**, the Documentation workflow rebuilds and deploys all published versions. This uses
`workflow_run` because releases created by `GITHUB_TOKEN` do not trigger a second `release`
event workflow. No manual version-list edit is needed for a major or minor release.

For each release, update affected user guides in the release source **before tagging**, ensure
the generated references are current, then verify both the Release and Documentation workflows
succeed. A green Release alone does not certify the separate Pages deployment. If documentation
deployment fails, rerun Documentation from main using **Run workflow**; it reads the published
release list again and builds every version. It does not create or modify releases.
