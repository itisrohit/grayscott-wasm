---
sidebar_position: 7
title: Inverse Recovery
---

import { InverseRecoveryVisualizer } from "../src/components/GuideVisuals";

# Inverse Recovery

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

## What do these methods mean in everyday language?

- **dense grid search**:
  try many parameter pairs on a fixed table and keep the best one.
- **finite-difference gradients**:
  nudge parameters slightly and see how the loss changes.
- **forward-mode AD gradients**:
  compute sensitivity information directly through the program.
- **fixed-step optimization**:
  move in a promising direction using a fixed update size.
- **backtracking optimization**:
  try a step, shrink it if needed, and accept it when the loss decreases enough.

You do not need to master all the math to understand the big picture:

> some methods search by brute force, some methods search by using sensitivity
> information, and the project compares both styles honestly.

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

## Why is noise important?

Without noise, an inverse result can look cleaner than it really is.

Reviewers and careful readers will ask:

> Does the method still behave sensibly when the target is not perfect?

That is why the repo sweeps deterministic target noise and uses multiple seeds.

## What does “17 evaluations instead of 1581 candidates” mean?

It means the backtracking AD optimizer did not try every point in a large grid.
Instead, it used gradient information to reach a low-loss region in far fewer
 solver evaluations.

That is a key algorithmic point:

- brute-force search is simple but expensive,
- gradient-based methods can be much cheaper,
- but only if the gradients are trustworthy and the optimization rule is stable.

Another way to say it:

- grid search spends work everywhere,
- gradient-based recovery tries to spend work where improvement looks most
  likely.

## Why is this still limited?

Because the inverse problem is intentionally small.

- only two scalar parameters are inferred,
- the initial condition is fixed,
- the loss is based on the final field,
- the experiments are CPU-only and local.

That does not make the work weak. It just means the result should be read as a
carefully measured small-parameter study, not as a general inverse-PDE solver.
