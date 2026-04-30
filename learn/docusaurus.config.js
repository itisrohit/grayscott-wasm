// @ts-check

const config = {
  title: "Gray-Scott Research Guide",
  tagline: "A chapter-by-chapter explainer for students and first-time readers",

  url: "https://itisrohit.github.io",
  baseUrl: "/grayscott-wasm/",

  organizationName: "itisrohit",
  projectName: "grayscott-wasm",
  deploymentBranch: "gh-pages",
  trailingSlash: false,

  onBrokenLinks: "throw",
  onBrokenMarkdownLinks: "warn",

  i18n: {
    defaultLocale: "en",
    locales: ["en"]
  },

  presets: [
    [
      "classic",
      {
        docs: {
          path: "guide",
          routeBasePath: "guide",
          sidebarPath: require.resolve("./sidebars.js")
        },
        blog: false,
        theme: {
          customCss: require.resolve("./src/css/custom.css")
        }
      }
    ]
  ],

  themeConfig: {
    navbar: {
      title: "Gray-Scott Guide",
      items: [
        { to: "/guide/intro", label: "Guide", position: "left" },
        {
          href: "https://github.com/itisrohit/grayscott-wasm",
          label: "GitHub",
          position: "right"
        }
      ]
    },
    footer: {
      style: "dark",
      links: [
        {
          title: "Read",
          items: [
            { label: "Guide", to: "/guide/intro" },
            { label: "Paper PDF", href: "https://github.com/itisrohit/grayscott-wasm/blob/main/paper/grayscott_wasm_IEEE_Journal_Paper.pdf" }
          ]
        },
        {
          title: "Artifact",
          items: [
            { label: "Repository", href: "https://github.com/itisrohit/grayscott-wasm" },
            { label: "Experiment Log", href: "https://github.com/itisrohit/grayscott-wasm/blob/main/docs/experiment-log.md" }
          ]
        }
      ]
    },
    prism: {
      additionalLanguages: ["rust", "python", "bash", "json"]
    }
  }
};

module.exports = config;
