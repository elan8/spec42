import { defineConfig } from "vitepress";

export default defineConfig({
  title: "Spec42",
  description: "SysML v2 tooling for VS Code",
  base: "/spec42/",
  themeConfig: {
    logo: "/logo.svg",
    nav: [
      { text: "Guide", link: "/guide/getting-started" },
      { text: "Reference", link: "/reference/sysml-quick-reference" },
      {
        text: "SysML v2 Spec",
        link: "https://www.omg.org/spec/SysML/2.0/Language/",
      },
    ],
    sidebar: [
      {
        text: "Guide",
        items: [
          { text: "Getting Started", link: "/guide/getting-started" },
          { text: "Examples", link: "/guide/examples" },
          { text: "Diagram Visualizer", link: "/guide/visualizer" },
          { text: "Library & Dependencies", link: "/guide/libraries" },
        ],
      },
      {
        text: "Reference",
        items: [
          { text: "SysML v2 Quick Reference", link: "/reference/sysml-quick-reference" },
        ],
      },
    ],
    socialLinks: [{ icon: "github", link: "https://github.com/elan8/spec42" }],
    footer: {
      message: "Released under the MIT License.",
      copyright: "Copyright © 2024-present Elan8",
    },
  },
});
