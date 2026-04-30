import React from "react";
import styles from "./GuideVisuals.module.css";

function Panel({ title, text, children }) {
  return (
    <div className={styles.panel}>
      <div className={styles.panelTitle}>{title}</div>
      {text ? <p className={styles.panelText}>{text}</p> : null}
      {children}
    </div>
  );
}

function StepCard({ number, title, text }) {
  return (
    <div className={styles.stepCard}>
      <div className={styles.stepBadge}>{number}</div>
      <div>
        <div className={styles.stepTitle}>{title}</div>
        <div className={styles.stepText}>{text}</div>
      </div>
    </div>
  );
}

export function CellUpdateVisualizer() {
  return (
    <Panel
      title="How one cell update really works"
      text="Focus on the center cell only. The solver checks the center cell and the four direct neighbors, uses them to estimate spreading, applies the local reaction rule, and then stores the next center value."
    >
      <div className={styles.twoColumn}>
        <div className={styles.gridDiagram}>
          <div className={styles.gridRowCenter}>
            <div className={styles.gridSpacer} />
            <div className={styles.gridCell}>up</div>
            <div className={styles.gridSpacer} />
          </div>
          <div className={styles.gridRowCenter}>
            <div className={styles.gridCell}>left</div>
            <div className={`${styles.gridCell} ${styles.gridCellCenter}`}>center</div>
            <div className={styles.gridCell}>right</div>
          </div>
          <div className={styles.gridRowCenter}>
            <div className={styles.gridSpacer} />
            <div className={styles.gridCell}>down</div>
            <div className={styles.gridSpacer} />
          </div>
          <div className={styles.gridCaption}>
            These five cells are the local neighborhood used for the update.
          </div>
        </div>

        <div className={styles.stepList}>
          <StepCard
            number="1"
            title="Read nearby values"
            text="Look at the center cell and the four direct neighbors."
          />
          <StepCard
            number="2"
            title="Compute the next center value"
            text="Combine neighbor spreading, local reaction, and feed/kill effects."
          />
          <StepCard
            number="3"
            title="Store the updated center"
            text="The old center numbers are replaced by the new center numbers."
          />
        </div>
      </div>
    </Panel>
  );
}

export function BrowserLoadVisualizer() {
  return (
    <Panel
      title="How the browser loads the WASM solver"
      text="This is the simplest correct picture. The browser loads the page, JavaScript starts, the WASM module is fetched and instantiated, and then JavaScript can call solver functions."
    >
      <div className={styles.flowRow}>
        <div className={styles.flowNode}>
          <div className={styles.flowNodeTitle}>Browser page</div>
          <div className={styles.flowNodeText}>HTML, CSS, and UI</div>
        </div>
        <div className={styles.flowArrow}>→</div>
        <div className={styles.flowNode}>
          <div className={styles.flowNodeTitle}>JavaScript</div>
          <div className={styles.flowNodeText}>controls and glue</div>
        </div>
        <div className={styles.flowArrow}>→</div>
        <div className={styles.flowNode}>
          <div className={styles.flowNodeTitle}>WASM module</div>
          <div className={styles.flowNodeText}>compiled low-level code</div>
        </div>
        <div className={styles.flowArrow}>→</div>
        <div className={styles.flowNode}>
          <div className={styles.flowNodeTitle}>Rust solver logic</div>
          <div className={styles.flowNodeText}>numerical update rules</div>
        </div>
      </div>

      <div className={styles.stepGrid}>
        <StepCard
          number="1"
          title="Open the page"
          text="The browser loads the page files and starts the JavaScript code."
        />
        <StepCard
          number="2"
          title="Load the WASM package"
          text="JavaScript fetches the generated JS glue and the .wasm binary."
        />
        <StepCard
          number="3"
          title="Instantiate the module"
          text="The browser validates the WASM binary and prepares its exported functions."
        />
        <StepCard
          number="4"
          title="Call solver functions"
          text="JavaScript can now call the Rust-backed exports when the page needs compute."
        />
        <StepCard
          number="5"
          title="Return results"
          text="Results go back to JavaScript, which updates the page, tables, or canvas."
        />
      </div>
    </Panel>
  );
}

export function CpuGpuVisualizer() {
  return (
    <Panel
      title="CPU-style thinking vs GPU-style thinking"
      text="This is not about one being smart and the other being dumb. It is about how the work is organized."
    >
      <div className={styles.compareGrid}>
        <div className={styles.compareCard}>
          <div className={styles.compareTitle}>CPU</div>
          <div className={styles.compareSubtitle}>fewer stronger lanes</div>
          <div className={styles.lanes}>
            <div className={styles.laneLong} />
            <div className={styles.laneLong} />
            <div className={styles.laneLong} />
            <div className={styles.laneLong} />
          </div>
          <div className={styles.compareText}>Better when the program has mixed tasks, control flow, and coordination.</div>
        </div>

        <div className={styles.compareCard}>
          <div className={styles.compareTitle}>GPU</div>
          <div className={styles.compareSubtitle}>many parallel lanes</div>
          <div className={styles.lanesShort}>
            <div className={styles.laneShort} />
            <div className={styles.laneShort} />
            <div className={styles.laneShort} />
            <div className={styles.laneShort} />
            <div className={styles.laneShort} />
            <div className={styles.laneShort} />
            <div className={styles.laneShort} />
            <div className={styles.laneShort} />
          </div>
          <div className={styles.compareText}>Better when the same math can be repeated over lots of data in parallel.</div>
        </div>
      </div>
    </Panel>
  );
}

export function CpuPipelineVisualizer() {
  return (
    <Panel
      title="Low-level CPU-side work flow"
      text="This is the repeated solver path. Each update works through these stages in order."
    >
      <div className={styles.pipelineList}>
        <div className={styles.pipelineItem}>1. Grid values live in arrays in memory.</div>
        <div className={styles.pipelineArrow}>↓</div>
        <div className={styles.pipelineItem}>2. Read the nearby cells needed for the stencil update.</div>
        <div className={styles.pipelineArrow}>↓</div>
        <div className={styles.pipelineItem}>3. Compute spreading and local reaction.</div>
        <div className={styles.pipelineArrow}>↓</div>
        <div className={styles.pipelineItem}>4. Write the updated values back.</div>
        <div className={styles.pipelineArrow}>↓</div>
        <div className={styles.pipelineItem}>5. Repeat across many cells and many time steps.</div>
        <div className={styles.pipelineArrow}>↓</div>
        <div className={styles.pipelineItem}>6. Optional SIMD means one instruction can handle several floats together.</div>
        <div className={styles.pipelineArrow}>↓</div>
        <div className={styles.pipelineItem}>7. Results are exposed to JavaScript or the browser canvas path.</div>
      </div>
    </Panel>
  );
}
