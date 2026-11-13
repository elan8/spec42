# Versioned user documentation

The public guide is hosted at <https://elan8.github.io/spec42/>. GitHub Pages must use
**GitHub Actions** as its build/deployment source. `.github/workflows/docs.yml` builds and
publishes the site; configure that repository setting once when adopting this workflow.

## Author and preview

Edit `docs/user/` and the generated references' authoritative inputs. `navigation.json` selects
the user-facing pages and groups them by task. `index.md` is the common navigation overview;
keep it free of release-specific feature claims. Architecture and contributor material stays
on GitHub, linked at the selected release tag. Guides absent from an older tag are omitted from
that version's navigation and search; their version-switch URL explains that the guide is unavailable.

From this directory:

```sh
npm ci --ignore-scripts
npm test
npm run build
npm run preview
```

Open <http://127.0.0.1:4173/spec42/>. Node 24 and a full Git checkout with tags are required.
Output goes to ignored `target/docs-site/`. The local build includes stable tags from 0.60.0
onward; production uses the GitHub API's published, non-draft, non-prerelease release list.
Earlier documentation remains available in the repository's tags.

The build checks local file targets and all links/fragments between hosted guide pages.
External URLs and fragments on GitHub source pages are not fetched. Search is local to each
version and requires JavaScript; the guides, navigation and version permalink remain readable
without JavaScript. The version selector requires JavaScript.

## Release policy

- Every published stable major, minor **and patch** release from 0.60.0 has a permanent
  `/vMAJOR.MINOR.PATCH/` guide, built from that exact tag's Markdown.
- `/MAJOR.MINOR/` redirects to the newest published patch of that line. Every major/minor
  line remains in the version selector, with its exact patch releases grouped underneath.
- `/latest/` and the site root point to the numerically newest stable release. Publishing a
  maintenance patch for an older line does not move Latest backwards.
- `/dev/` uses the main branch, shows an unreleased banner and is excluded from search engines.
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

PR builds only upload a Pages artifact. Deployment runs from main and after successful releases;
it requires the `github-pages` environment, `pages: write` and OIDC permissions. Public Pages
publishing does not need a personal access token.
