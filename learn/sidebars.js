module.exports = {
  tutorialSidebar: [
    "intro",
    {
      type: "category",
      label: "Chapter 1: Big Picture",
      items: ["big-picture", "gray-scott"]
    },
    {
      type: "category",
      label: "Chapter 2: Computing Ideas",
      items: [
        "rust-and-wasm",
        "cpu-and-gpu",
        "computer-science-concepts",
        "code-organization",
        "trust-and-reproducibility",
        "common-questions"
      ]
    },
    {
      type: "category",
      label: "Chapter 3: Math and Algorithms",
      items: [
        "math-and-algorithms",
        "algorithm-map",
        "forward-solver-math",
        "complexity-and-parallelism",
        "inverse-math-and-search"
      ]
    },
    {
      type: "category",
      label: "Chapter 4: Experiments",
      items: [
        "experiment-map",
        "forward-validation",
        "inverse-recovery",
        "browser-and-simd"
      ]
    },
    {
      type: "category",
      label: "Chapter 5: Use It Yourself",
      items: ["reproduce-results", "glossary"]
    }
  ]
};
