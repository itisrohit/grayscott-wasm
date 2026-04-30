---
sidebar_position: 7
title: Inverse Recovery
---

# Inverse Recovery

## What is the inverse task here?

The project generates a target pattern using known parameters and then tries to
recover the feed and kill parameters from the resulting final field.

Those parameters are:

- `F`
- `k`

The model keeps the initial condition and several other simulation settings
fixed.

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

## Why is this still limited?

Because the inverse problem is intentionally small.

- only two scalar parameters are inferred,
- the initial condition is fixed,
- the loss is based on the final field,
- the experiments are CPU-only and local.

That does not make the work weak. It just means the result should be read as a
carefully measured small-parameter study, not as a general inverse-PDE solver.
