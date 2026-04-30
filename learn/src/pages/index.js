import clsx from "clsx";
import Link from "@docusaurus/Link";
import Layout from "@theme/Layout";
import styles from "./index.module.css";

const chapters = [
  {
    title: "Big Picture",
    text: "What the research is about, why Gray-Scott matters, and what problem the project actually solves.",
    to: "/guide/big-picture"
  },
  {
    title: "Rust, WASM, and CS Ideas",
    text: "Layman explanations of memory layout, SIMD, WebAssembly, workers, benchmarking, and automatic differentiation.",
    to: "/guide/rust-and-wasm"
  },
  {
    title: "Experiments",
    text: "What was measured, why each experiment exists, what the numbers mean, and what the limitations are.",
    to: "/guide/forward-validation"
  },
  {
    title: "Reproduce It",
    text: "The practical path for rerunning the artifact, browser pages, and paper-facing benchmark commands.",
    to: "/guide/reproduce-results"
  }
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
          <div className={styles.actions}>
            <Link className={clsx("button button--primary button--lg", styles.primary)} to="/guide/intro">
              Start Reading
            </Link>
            <Link className={clsx("button button--secondary button--lg", styles.secondary)} to="/guide/glossary">
              Open Glossary
            </Link>
          </div>
        </section>

        <section className={styles.grid}>
          {chapters.map((chapter) => (
            <Link key={chapter.title} className={styles.card} to={chapter.to}>
              <h2>{chapter.title}</h2>
              <p>{chapter.text}</p>
            </Link>
          ))}
        </section>
      </main>
    </Layout>
  );
}
