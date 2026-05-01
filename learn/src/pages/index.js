import clsx from "clsx";
import Link from "@docusaurus/Link";
import Layout from "@theme/Layout";
import styles from "./index.module.css";

const chapters = [
  {
    title: "Big Picture",
    chapter: "Chapter 1",
    text: "What the research is about, why Gray-Scott matters, and what problem the project actually solves.",
    detail: "Research question, scope, and physical intuition.",
    to: "/guide/big-picture"
  },
  {
    title: "Rust, WASM, and CS Ideas",
    chapter: "Chapter 2",
    text: "Layman explanations of memory layout, SIMD, WebAssembly, workers, benchmarking, and automatic differentiation.",
    detail: "Execution model, memory model, and code organization.",
    to: "/guide/rust-and-wasm"
  },
  {
    title: "Math and Algorithms",
    chapter: "Chapter 3",
    text: "How the PDE becomes code, how the inverse loss is defined, and how gradients and optimization are computed.",
    detail: "Discrete solver, AD, search methods, and complexity.",
    to: "/guide/math-and-algorithms"
  },
  {
    title: "Experiments",
    chapter: "Chapter 4",
    text: "What was measured, why each experiment exists, what the numbers mean, and what the limitations are.",
    detail: "Validation, performance, browser paths, and inverse recovery.",
    to: "/guide/forward-validation"
  },
  {
    title: "Use It Yourself",
    chapter: "Chapter 5",
    text: "The practical path for rerunning the artifact, browser pages, and paper-facing benchmark commands.",
    detail: "Reproduction steps, glossary, and result interpretation.",
    to: "/guide/reproduce-results"
  }
];

const facts = [
  { label: "Guide chapters", value: "5" },
  { label: "Core experiment families", value: "4" },
  { label: "Primary implementation path", value: "Rust + WASM" },
  { label: "Inverse scope", value: "2 parameters" }
];

export default function Home() {
  return (
    <Layout
      title="Gray-Scott Research Guide"
      description="Student-friendly guide to the grayscott-wasm research artifact"
    >
      <main className={styles.page}>
        <section className={styles.hero}>
          <p className={styles.eyebrow}>For students, first-time readers, and curious developers</p>
          <h1>Learn this research without already knowing PDEs, Rust, or WebAssembly.</h1>
          <p className={styles.lead}>
            This guide explains the project chapter by chapter: the science, the code,
            the experiments, the benchmark numbers, and the browser deployment story.
          </p>
          <div className={styles.heroMeta}>
            <p>
              A focused guide to the <code>grayscott-wasm</code> research artifact:
              validated forward simulation, measured browser delivery, and small-parameter
              inverse recovery.
            </p>
          </div>
          <div className={styles.actions}>
            <Link className={clsx("button button--primary button--lg", styles.primary)} to="/guide/big-picture">
              Start with Big Picture
            </Link>
            <Link className={clsx("button button--secondary button--lg", styles.secondary)} to="/guide/experiment-map">
              View Experiment Map
            </Link>
          </div>
        </section>

        <section className={styles.factStrip} aria-label="Guide facts">
          {facts.map((fact) => (
            <div key={fact.label} className={styles.fact}>
              <span className={styles.factValue}>{fact.value}</span>
              <span className={styles.factLabel}>{fact.label}</span>
            </div>
          ))}
        </section>

        <section className={styles.contentBand}>
          <div className={styles.chapterList}>
            <div className={styles.sectionHeading}>
              <p>Guide map</p>
              <h2>Read it in chapters, not fragments.</h2>
            </div>
            <div className={styles.chapterRows}>
              {chapters.map((chapter) => (
                <Link key={chapter.title} className={styles.chapterRow} to={chapter.to}>
                  <div className={styles.chapterMeta}>
                    <span className={styles.chapterTag}>{chapter.chapter}</span>
                    <h3>{chapter.title}</h3>
                  </div>
                  <div className={styles.chapterCopy}>
                    <p>{chapter.text}</p>
                    <span>{chapter.detail}</span>
                  </div>
                </Link>
              ))}
            </div>
          </div>

          <aside className={styles.sidePanel}>
            <div className={styles.sectionHeading}>
              <p>Why this site exists</p>
              <h2>It is a teaching guide, not a marketing page.</h2>
            </div>
            <ul className={styles.sideList}>
              <li>Explains the solver, browser path, and inverse side in the order they actually matter.</li>
              <li>Uses the real measured tables from the experiment log instead of vague claims.</li>
              <li>Separates correctness, performance, browser delivery, and inverse recovery so the tradeoffs stay inspectable.</li>
              <li>Gives a practical route for rerunning the artifact without treating the repo like a black box.</li>
            </ul>
            <div className={styles.sideCallout}>
              <span>Best starting point for new readers</span>
              <Link to="/guide/big-picture">Begin with Big Picture</Link>
            </div>
          </aside>
        </section>
      </main>
    </Layout>
  );
}
