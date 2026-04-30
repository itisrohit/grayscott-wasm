// @ts-check

module.exports = async function createConfigAsync() {
  const remarkMath = (await import("remark-math")).default;
  const rehypeKatex = (await import("rehype-katex")).default;

  return {
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
    markdown: {
      mermaid: true
    },

    i18n: {
      defaultLocale: "en",
      locales: ["en"]
    },

    themes: ["@docusaurus/theme-mermaid"],

    stylesheets: [
      {
        href: "/grayscott-wasm/katex/katex.min.css",
        type: "text/css"
      }
    ],

    presets: [
      [
        "classic",
        {
          docs: {
            path: "guide",
            routeBasePath: "guide",
            sidebarPath: require.resolve("./sidebars.js"),
            remarkPlugins: [remarkMath],
            rehypePlugins: [rehypeKatex]
          },
          blog: false,
          theme: {
            customCss: require.resolve("./src/css/custom.css")
          }
        }
      ]
    ],

    plugins: [
      function ignoreKnownWebpackWarning() {
        return {
          name: "ignore-vscode-languageserver-warning",
          configureWebpack() {
            return {
              ignoreWarnings: [
                {
                  module: /vscode-languageserver-types[\\/]lib[\\/]umd[\\/]main\.js$/,
                  message:
                    /Critical dependency: require function is used in a way in which dependencies cannot be statically extracted/
                }
              ]
            };
          }
        };
      }
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
              {
                label: "Paper PDF",
                href: "https://github.com/itisrohit/grayscott-wasm/blob/main/paper/grayscott_wasm_IEEE_Journal_Paper.pdf"
              }
            ]
          },
          {
            title: "Artifact",
            items: [
              {
                label: "Repository",
                href: "https://github.com/itisrohit/grayscott-wasm"
              },
              {
                label: "Experiment Log",
                href: "https://github.com/itisrohit/grayscott-wasm/blob/main/docs/experiment-log.md"
              }
            ]
          }
        ]
      },
      prism: {
        additionalLanguages: ["rust", "python", "bash", "json"]
      },
      mermaid: {
        theme: { light: "neutral", dark: "dark" }
      }
    }
  };
};
