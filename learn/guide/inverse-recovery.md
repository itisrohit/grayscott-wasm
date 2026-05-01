---
sidebar_position: 8
title: Inverse Recovery
---

import { InverseRecoveryVisualizer } from "../src/components/GuideVisuals";

# Inverse Recovery

If you want the algorithm details first, read
[Inverse Math and Search](./inverse-math-and-search) before this page. This
page is about what the inverse experiments showed and how to interpret them.

## The question on this page

This page answers:

> Can this artifact recover `F` and `k` from a final pattern, and what does
> each inverse experiment prove or fail to prove?

This is the most layered experiment family in the repo. It is not one method.
It is a staircase:

1. build a brute-force baseline,
2. define a gradient baseline,
3. check AD against that baseline,
4. compare gradient cost,
5. compare optimization methods,
6. test robustness under regime changes and noise.

## Start with the simplest version

If you are new to inverse problems, use this mental model:

- a **forward** problem means: choose inputs, run the system, observe the
  output;
- an **inverse** problem means: observe the output, then work backward and ask
  what inputs probably created it.

In this project, the visible output is a final Gray-Scott pattern, and the
hidden inputs are the two parameters `F` and `k`.

<InverseRecoveryVisualizer />

## What is the inverse task here?

The project generates a target pattern using known parameters and then tries to
recover the feed and kill parameters from the resulting final field.

Those parameters are:

- `F`
- `k`

The model keeps the initial condition and several other simulation settings
fixed.

That means this project is **not** asking:

- can we recover every detail of the simulation,
- can we recover the whole starting field,
- can we solve every inverse PDE problem.

It is asking a smaller and cleaner question:

> If the pattern came from some unknown `F` and `k`, can we estimate those two
> values from the final observed field?

## Why generate the target ourselves first?

Because then we know the correct answer.

The project first creates a target pattern using known parameters. After that,
the inverse methods try to recover those parameters without being told them.

That is a fair test because:

- we know what the truth is,
- we can measure how close the recovered answer is,
- we can compare different inverse methods on the same target.

## What does “match the pattern” mean here?

The solver needs some way to score a candidate pair of parameters.

The basic idea is:

1. guess a candidate `F` and `k`,
2. run the simulation with that guess,
3. compare the guessed final pattern to the target final pattern,
4. assign a loss value,
5. prefer guesses with lower loss.

In plain language:

> lower loss means "this guessed pattern looks more like the target."

For this project, that loss is based on field differences across the final `u`
and `v` values. You can read it as a structured "how far apart are these two
final patterns?" score.

Terms used repeatedly on this page:

- **target**:
  the known final field generated first, which the inverse method tries to
  match.
- **guess**:
  the current candidate `F, k` pair being tested.
- **clean loss**:
  loss against the unnoised target field.
- **noisy loss**:
  loss against the noise-perturbed target field used in the robustness runs.
- **parameter error**:
  absolute difference between recovered `F/k` and the generating `F/k`.
- **evaluated**:
  count of solver-backed objective evaluations. This is a better algorithm-cost
  signal than only reporting wall-clock time.

## Experiment 1: dense grid search

### Why this exists

Grid search is the baseline that does not require gradients or optimizer
judgment.

It answers:

> If we evaluate many candidate `(F, k)` pairs directly, what is the best
> field-match we can find on a fixed parameter grid?

### What the method is

- choose a rectangular parameter range,
- sample a fixed number of `F` values and `k` values,
- run the forward solver at every candidate pair,
- keep the one with the lowest final-field loss.

### What the metric means

- **loss**: field mismatch score,
- **evaluated**: number of candidate points tested.

Protocol:

- same fixed initial condition,
- same grid and step count as the target generation,
- rectangular `(F, k)` search range,
- test every candidate point directly.

### What happened

- when the true target was on the grid, the method recovered it exactly;
- when the true target was off-grid, the method recovered the nearest good
  candidate with low loss.

Grid-search baseline:

| Case | Target F | Target k | Best F | Best k | F abs err | k abs err | Loss | Evaluated |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| On-grid | 0.060000 | 0.062000 | 0.060000 | 0.062000 | 0.000000 | 0.000000 | 0.000e0 | 25 |
| Off-grid | 0.060550 | 0.062450 | 0.060500 | 0.062500 | 0.000050 | 0.000050 | 2.997e-7 | 121 |

### Why this matters

Grid search is expensive, but it is easy to trust.

That makes it a good baseline for everything that comes later.

### Tradeoff

- **good**: simple, defensible, no derivative machinery.
- **bad**: scales poorly because it spends work everywhere, even in bad regions.

## Experiment 2: finite-difference gradients

### Why this exists

Before trusting automatic differentiation, the repo needs a numerical gradient
baseline.

### What the method is

- perturb `F` slightly up and down,
- perturb `k` slightly up and down,
- see how the loss changes,
- estimate `dL/dF` and `dL/dk` from those changes.

### What the metric means

- **gradient value**: direction and sensitivity of the loss,
- **evaluated = 5**: one primal loss plus four perturbed losses for two
  parameters.

Protocol:

- off-target guess at `F = 0.060000`, `k = 0.063000`,
- target at `F = 0.060550`, `k = 0.062450`,
- epsilon `= 1e-4`.

### What happened

The finite-difference gradient was finite, nonzero, and usable as a baseline.

Representative finite-difference gradient check:

| Guess F | Guess k | Epsilon | Loss | dLoss/dF | dLoss/dk | Evaluated |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 0.060000 | 0.063000 | 1.0e-4 | 3.761e-5 | 6.972e-2 | 2.088e-1 | 5 |

### Why this matters

Without this step, an AD gradient would be harder to defend.

### Tradeoff

- **good**: conceptually simple and general.
- **bad**: cost grows with parameter count, and the epsilon choice itself
  becomes part of the method.

## Experiment 3: fixed-step finite-difference optimization

### Why this exists

This is the first full optimization baseline.

It asks:

> If we already have finite-difference gradients, can a simple gradient-descent
> loop reduce the loss?

### What happened

Yes. The fixed-step optimizer reduced the loss substantially from the off-target
starting guess.

Fixed-step FD optimizer summary:

| Initial loss | Final F | Final k | F abs err | k abs err | Final loss | Evaluated |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 3.761e-5 | 0.059956 | 0.062867 | 0.000594 | 0.000417 | 1.317e-5 | 45 |

### What it means

This proves the inverse objective is at least locally navigable. It does **not**
yet prove that the method is efficient or best.

### Tradeoff

- **good**: simple and easy to explain.
- **bad**: uses a hand-chosen learning rate and can waste work or stall if that
  step size is poor.

## Experiment 4: forward-mode AD gradient check

### Why this exists

This is the first trust check for the differentiable solver.

It asks:

> Does the AD gradient agree with the finite-difference gradient on the same
> test case?

### What happened

It did. The relative differences stayed below about `4e-4` in the logged
comparison point.

AD vs FD gradient agreement:

| Loss | AD dLoss/dF | FD dLoss/dF | F rel delta | AD dLoss/dk | FD dLoss/dk | k rel delta |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 3.761e-5 | 6.971e-2 | 6.972e-2 | 2.418e-4 | 2.087e-1 | 2.088e-1 | 3.598e-4 |

### What it means

That does not make AD "proven forever," but it is strong evidence that the
forward-mode implementation is carrying useful derivative information through
the solver correctly.

### Tradeoff

- **good**: AD removes the need to choose a finite-difference epsilon for every
  future run.
- **bad**: the AD implementation is more specialized and uses more memory.

## Experiment 5: AD vs finite-difference overhead

### Why this exists

Correct gradients are not enough. They also have to be worth their cost.

This experiment asks:

> For the same two-parameter inverse query, how much more expensive is AD than
> one primal loss, and how does that compare with finite differences?

### What the metric means

- **primal loss**: one forward-solve-based loss evaluation,
- **overhead vs primal**: how much more expensive the gradient method is than a
  single loss,
- **AD vs finite difference**: the direct cost comparison between the two
  gradient approaches.

Protocol:

- grids: `64`, `128`, `256`,
- steps: `100`,
- trials: `7` for the direct timing table,
- Criterion benchmark used for tighter `128` and `256` statistical intervals.

### What happened

At `128 x 128` and `256 x 256`:

- finite differences cost about `5x` one primal loss,
- forward-mode AD cost about `2.6x`,
- AD was therefore roughly `1.9x` cheaper than finite differences for this
  two-parameter query.

Criterion-derived overhead:

| Grid | Finite difference vs primal | Forward-mode AD vs primal | AD vs finite difference |
| --- | ---: | ---: | ---: |
| 128x128 | ~5.0x | ~2.65x | ~1.9x cheaper |
| 256x256 | ~5.0x | ~2.65x | ~1.9x cheaper |

### Why the `64 x 64` result is not emphasized

Because the timings are too small and noisy there.

This is a good example of a research judgment call:

- do not report the noisiest small-grid ratio as if it were equally stable.

### Why AD still costs more than "ideal arithmetic intuition"

Because the dual-number fields carry:

- the primal value,
- derivative with respect to `F`,
- derivative with respect to `k`.

That increases memory traffic and cache pressure.

### Tradeoff

- **finite difference**:
  simpler concept, but cost grows linearly with parameter count.
- **forward-mode AD**:
  better here for `p = 2`, but memory overhead is real.

## Experiment 6: multi-regime grid-search recovery

### Why this exists

One hand-picked target is not enough.

This experiment asks:

> Does the grid-search baseline still recover plausible parameters across
> several target regimes?

### What happened

It recovered close parameters in all tested regimes, but not uniformly exact
ones.

The lower-feed regime was especially useful because it showed an important
lesson:

- low field loss does not always mean tiny parameter error.

Multi-regime recovery:

| Regime | Target F | Target k | Best F | Best k | F abs err | k abs err | Loss |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| default-off-grid | 0.060550 | 0.062450 | 0.060500 | 0.062500 | 0.000050 | 0.000050 | 2.997e-7 |
| lower-feed | 0.050250 | 0.060250 | 0.051000 | 0.060000 | 0.000750 | 0.000250 | 1.088e-6 |
| higher-feed | 0.067250 | 0.064750 | 0.067000 | 0.065000 | 0.000250 | 0.000250 | 1.238e-11 |

### Why this matters

It tells students not to confuse "good pattern fit" with "fully identified
parameters."

## Experiment 7: noise sensitivity

### Why this exists

Clean synthetic targets are not the whole story.

This experiment asks:

> If we add controlled noise to the target field, when does recovery stop being
> stable?

### What happened

Across four deterministic seeds:

- stable through noise amplitude `0.020`,
- mixed failure by `0.050`,
- clearer degradation by `0.100`.

Noise summary across seeds:

| Noise amplitude | Stable recovered candidate? | Representative shift |
| ---: | --- | --- |
| 0.000 | yes | none |
| 0.020 | yes | none |
| 0.050 | mixed | some seeds move to `(0.059000, 0.063000)` |
| 0.100 | degraded | several seeds move away from clean candidate |

### Why this matters

This is stronger than saying "it seems robust." It gives a concrete failure
boundary for this exact setup.

### Tradeoff

The noise model is intentionally simple: synthetic independent uniform field
noise. That is useful as a controlled stress test, but it is not the same thing
as real experimental measurement noise.

Representative grid-search noise table:

| Noise | Seed | Best F | Best k | F abs err | k abs err | Loss vs noisy target | Loss vs clean target |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 0.020 | 24301 | 0.060500 | 0.062500 | 0.000050 | 0.000050 | 7.205e-5 | 2.997e-7 |
| 0.050 | 51966 | 0.059000 | 0.063000 | 0.001550 | 0.000550 | 4.354e-4 | 1.364e-6 |
| 0.100 | 51966 | 0.057000 | 0.063500 | 0.003550 | 0.001050 | 1.727e-3 | 7.231e-6 |

## Experiment 8: fixed-step AD optimization

### Why this exists

After checking AD gradients, the next question is:

> Can an AD-based optimizer reduce loss with far fewer evaluations than dense
> grid search?

### What happened

Yes, but with an important limit.

The fixed-step AD optimizer used only `9` primal-equivalent evaluations, stayed
stable across the tested seeds, and reduced loss. But it did not beat the grid
baseline on clean or noisy loss.

Fixed-step AD vs grid, clean target:

| Method | Final F | Final k | F abs err | k abs err | Clean loss | Evaluated |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| grid | 0.060500 | 0.062500 | 0.000050 | 0.000050 | 2.997e-7 | 1581 |
| ad-opt | 0.059956 | 0.062867 | 0.000594 | 0.000417 | 1.318e-5 | 9 |

### Why this matters

It shows that "using AD" is not enough by itself. The optimizer rule matters
too.

### Tradeoff

- **good**: very cheap in evaluations.
- **bad**: fixed step size is too rigid to make the best use of the gradient.

Protocol:

- `8` optimizer iterations,
- learning rate `= 1e-4`,
- compare against the same target cases used for grid-search noise runs.

## Experiment 9: backtracking AD optimization

### Why this exists

This is the method-improvement experiment.

Instead of changing the derivative machinery, the repo changes the optimization
rule and asks:

> If the step size adapts with Armijo backtracking, does the AD optimizer
> become much stronger?

### What happened

Yes.

Compared with fixed-step AD:

- clean loss improved by about two orders of magnitude,
- evaluation count only increased from `9` to `17`,
- the optimizer stayed in a similar parameter region across noisy seeds.

On the clean target, `ad-line` even reached lower field loss than the dense
grid candidate because it was not restricted to the sampled grid points.

Backtracking AD vs fixed-step AD vs grid, clean target:

| Method | Final F | Final k | F abs err | k abs err | Clean loss | Evaluated |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| grid | 0.060500 | 0.062500 | 0.000050 | 0.000050 | 2.997e-7 | 1581 |
| ad-opt | 0.059956 | 0.062867 | 0.000594 | 0.000417 | 1.318e-5 | 9 |
| ad-line | 0.059894 | 0.062671 | 0.000656 | 0.000221 | 1.740e-7 | 17 |

### What it does **not** mean

It does **not** mean `ad-line` always has the smallest parameter error.

One of the most important lessons in the whole inverse chapter is this:

- lower field loss,
- lower noisy loss,
- lower clean loss,
- lower parameter error

are related, but they are not identical objectives.

### Tradeoff

- **grid search**:
  more brute force, easy to explain, expensive.
- **fixed-step AD**:
  cheap but less effective.
- **backtracking AD**:
  better loss-quality-to-evaluation balance, but more optimizer logic.

Noise comparison by method:

| Noise | Method | Representative final F | Representative final k | Clean loss | Evaluated | Reading |
| ---: | --- | ---: | ---: | ---: | ---: | --- |
| 0.000 | grid | 0.060500 | 0.062500 | 2.997e-7 | 1581 | best parameter match on sampled grid |
| 0.000 | ad-opt | 0.059956 | 0.062867 | 1.318e-5 | 9 | cheap but weaker optimizer |
| 0.000 | ad-line | 0.059894 | 0.062671 | 1.740e-7 | 17 | best field loss of the three |
| 0.050 | grid | 0.059000 | 0.063000 | 1.364e-6 | 1581 | can jump to a different sampled basin |
| 0.050 | ad-opt | 0.059954 | 0.062865 | 1.296e-5 | 9 | stable, but higher clean loss |
| 0.050 | ad-line | 0.059883 | 0.062666 | 1.966e-7 | 17 | stable and strong clean-loss result |
| 0.100 | grid | 0.057000 | 0.063500 | 7.231e-6 | 1581 | degradation clearly visible |
| 0.100 | ad-opt | 0.059952 | 0.062861 | 1.236e-5 | 9 | remains near same parameter region |
| 0.100 | ad-line | 0.059868 | 0.062653 | 3.717e-7 | 17 | still near same region with lower clean loss |

## Why is this not trivial?

Because the solver cannot simply inspect the final picture and read off the
correct answer directly.

Several things make inverse recovery hard:

- different parameter pairs can lead to somewhat similar-looking patterns,
- noise can disturb the target,
- the loss surface can have awkward regions,
- the simulation itself is expensive enough that naive search can cost a lot.

So the job is not just "guess and check once." The real job is to search
carefully and efficiently.

## Why use multiple methods?

Because one optimizer result by itself is not a strong research argument.

The repo compares:

- dense grid search,
- finite-difference gradients,
- forward-mode AD gradients,
- fixed-step AD optimization,
- backtracking AD optimization,
- noise robustness across multiple seeds.

This matters because students should see that research evidence is built from
comparisons, not from a single “best” run.

## What did the project find?

The strongest inverse result is not “we solved inverse design generally.”

The stronger and more honest result is:

- forward-mode AD is cheaper than finite differences for this two-parameter
  problem,
- Armijo backtracking works better than naive fixed-step descent,
- the recovery stays stable through moderate noise,
- the browser can run the same inverse loop as the native artifact.

## What is the browser-side inverse result actually saying?

It is **not** saying:

> Browsers are now the best place to solve giant inverse problems.

It is saying something narrower:

> Even in a browser-deliverable Rust/WASM artifact, the full small-parameter
> inverse loop can run, produce sensible answers, and stay measurable.

## Why is this still limited?

Because the inverse problem is intentionally small.

- only two scalar parameters are inferred,
- the initial condition is fixed,
- the loss is based on the final field,
- the experiments are CPU-only and local.

That does not make the work weak. It just means the result should be read as a
carefully measured small-parameter study, not as a general inverse-PDE solver.
