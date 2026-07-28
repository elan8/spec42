import { defineConfig } from "vitepress";
import meta from "./generated-meta.json";

export default defineConfig({
  title: "Spec42",
  description: "SysML v2 tooling for VS Code",
  base: "/spec42/",
  themeConfig: {
    logo: "/logo.svg",
    nav: [
      { text: "Guide", link: "/guide/getting-started" },
      { text: "Reference", link: "/reference/whats-included" },
      {
        text: "SysML v2 Spec",
        link: "https://www.omg.org/spec/SysML/2.0/",
      },
      {
        text: `v${meta.spec42Version}`,
        link: "/reference/whats-included",
      },
    ],
    sidebar: [
      {
        text: "Guide",
        items: [
          { text: "Getting Started", link: "/guide/getting-started" },
          { text: "Examples", link: "/guide/examples" },
          { text: "Model Explorer", link: "/guide/model-explorer" },
          { text: "Diagram Visualizer", link: "/guide/visualizer" },
          { text: "Feature Inspector", link: "/guide/feature-inspector" },
          { text: "Library & Dependencies", link: "/guide/libraries" },
        ],
      },
      {
        text: "Reference",
        items: [
          { text: "What's Included", link: "/reference/whats-included" },
          { text: "Domain Libraries", link: "/reference/domain-libraries" },
          { text: "Method Libraries", link: "/reference/method-libraries" },
          { text: "SysML v2 Quick Reference", link: "/reference/sysml-quick-reference" },
        ],
      },
    ],
    socialLinks: [{ icon: "github", link: "https://github.com/elan8/spec42" }],
    footer: {
      message: `Spec42 v${meta.spec42Version} · Released under the MIT License.`,
      copyright: "Copyright © 2024-present Elan8",
    },
  },
});
