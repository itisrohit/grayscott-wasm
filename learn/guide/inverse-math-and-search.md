---
sidebar_position: 10
title: Inverse Math and Search
---

# Inverse Math and Search

This page explains the main algorithms in `src/inverse.rs`.

If the previous page was about:

> how one guessed parameter pair produces a final pattern,

this page is about:

> how the repo searches for a good parameter pair when the target pattern is
> already known.

## The inverse problem as a mathematical map

Let the forward solver be a map:

```math
S(F,k) = \text{final simulated fields after the chosen number of steps}
```

More explicitly:

```math
S(F,k) = \bigl(u_{\mathrm{final}}(F,k),\, v_{\mathrm{final}}(F,k)\bigr)
```

The inverse task is then:

```math
\text{given } (u^*, v^*), \text{ find } (F,k) \text{ such that } S(F,k) \approx (u^*, v^*)
```

That is the formal mathematical heart of the whole inverse section.

## Step 1: define the inverse question carefully

The inverse problem here is intentionally small.

Known:

- grid size,
- number of steps,
- initial seed shape,
- $D_u$, $D_v$, and $\Delta t$,
- final target fields.

Unknown:

- $F$
- $k$

That narrow setup is important. It makes the inverse question clean enough to
study carefully.

If the repo had also tried to recover:

- the initial condition,
- diffusion constants,
- time-step size,
- or many more parameters,

then the search problem would get much harder and much easier to misinterpret.

## Step 2: generate a target with known truth

The repo first creates a target using `generate_target`.

That means:

1. choose true parameters,
2. run the seeded forward solver,
3. keep the final $u$ and $v$ fields.

Why do that instead of using an arbitrary picture?

Because then the repo knows the true answer. That lets it measure:

- parameter error,
- final loss,
- gradient quality,
- robustness under added noise.

Without a known truth, the inverse experiments would be much harder to defend.

This is also why `InverseTarget` stores:

- grid shape,
- step count,
- seed radius,
- true forward parameters.

It is the full recipe for “what target are we pretending to observe?”

## Step 3: convert “looks close” into a real score

The repo needs a scoring rule. It uses mean squared error over the final $u$
and $v$ fields.

In plain text:

```math
L(F,k) = \text{average squared difference between simulated and target final fields}
```

`field_mse` does that field comparison.

Using $N$ grid cells, the loss is:

```math
L(F,k) = \frac{1}{2N} \sum_{i=1}^{N} \left[(u_i(F,k)-u_i^*)^2 + (v_i(F,k)-v_i^*)^2\right]
```

where:

- $u_i(F,k)$ and $v_i(F,k)$ are the simulated final fields,
- $u_i^*$ and $v_i^*$ are the target final fields.

Why squared difference?

- it is standard,
- easy to compute,
- smooth enough for gradient methods,
- and it penalizes larger mismatches more strongly than smaller ones.

Why final-field loss only?

Because that keeps the experiment focused and cheap enough to repeat many
times.

Why not compare full time histories?

That would be possible, but it would:

- store much more data,
- cost more memory,
- cost more computation,
- change the meaning of the inverse task.

The repo chose the smaller, clearer version.

There are actually two related loss ideas in the repo:

- noisy-target loss:
  how well a candidate matches the noisy observed target
- clean-target loss:
  how well the recovered candidate matches the original clean target

That distinction matters in the noise experiments and in the browser inverse
summary output.

## The optimization objective in one line

The inverse code is solving:

```math
\min_{F,k} L(F,k)
\quad \text{subject to} \quad
F_{\min} \le F \le F_{\max}, \;
k_{\min} \le k \le k_{\max}
```

That “subject to” part is important. The repo is solving a bounded optimization
problem, not an unconstrained one.

## Step 4: understand the search space

Once a loss exists, the inverse problem becomes:

> find the $(F, k)$ pair that makes loss as small as possible.

This search space is only two-dimensional, which is why the repo can compare
multiple methods honestly.

That is also why the repo uses forward-mode AD instead of reverse-mode or an
adjoint method.

With only two unknowns:

- forward-mode AD is simple,
- the implementation stays readable,
- the cost advantage over finite differences is still meaningful.

If the repo had hundreds or thousands of unknown parameters, this choice would
look very different.

The parameter search is also bounded deliberately. The optimizers do not roam
everywhere. They stay inside chosen feed and kill ranges so the experiments are
measuring a realistic, controlled region instead of arbitrary numerical drift.

This also explains the differentiation choice:

- finite differences cost about $2p + 1$ forward evaluations for $p$
  parameters,
- forward-mode AD scales roughly with the number of tangent channels carried,
- reverse-mode or adjoint methods become attractive when `p` is large.

In this repo, $p = 2$, so forward-mode AD is a rational fit.

## Algorithm 1: dense grid search

`grid_search` is the blunt instrument.

It does this:

1. pick a feed range,
2. pick a kill range,
3. create a rectangular table of candidate pairs,
4. run the forward solver for every pair,
5. keep the pair with the smallest loss.

Why use something so expensive?

Because it is:

- simple,
- transparent,
- easy to verify,
- and hard to fool yourself with.

It is the baseline that says:

> if we just brute-force a sensible rectangle, what answer do we get?

Why not use only grid search?

Because cost grows quickly:

```math
\text{more candidate points} \;\Rightarrow\; \text{more full forward solves} \;\Rightarrow\; \text{more runtime}
```

So grid search is a good baseline, not the best long-term optimizer.

The helper `linspace` is part of this algorithm. It decides exactly where the
candidate points lie. That means grid search quality depends not only on the
range but also on the count and spacing of those samples.

## Algorithm 2: finite-difference gradients

Finite differences estimate slope by re-running the solver with small
parameter nudges.

For one point $(F, k)$, the repo computes:

- base loss,
- loss at $F + \varepsilon$,
- loss at $F - \varepsilon$,
- loss at $k + \varepsilon$,
- loss at $k - \varepsilon$.

Then it estimates the gradient from those changes.

The central-difference formulas are:

```math
\frac{\partial L}{\partial F} \approx \frac{L(F+\varepsilon,k)-L(F-\varepsilon,k)}{2\varepsilon}
```

```math
\frac{\partial L}{\partial k} \approx \frac{L(F,k+\varepsilon)-L(F,k-\varepsilon)}{2\varepsilon}
```

Why does this matter?

Because it gives a “sensitivity direction”:

```math
\text{If } F \text{ increases slightly, does } L \text{ go up or down?}
```

```math
\text{If } k \text{ increases slightly, does } L \text{ go up or down?}
```

Why not stop here?

Because it is expensive. In this two-parameter setup it costs five forward
evaluations per gradient.

That is exactly why the repo benchmarks it against forward-mode AD.

There is also a practical tuning issue here: $\varepsilon$ cannot be chosen
carelessly.

- too large: the approximation becomes coarse
- too small: floating-point noise starts to matter more

That is another reason finite differences are useful as a reference but not a
perfect long-term method.

## Algorithm 3: forward-mode automatic differentiation

Forward-mode AD in this repo is not a magic library call. It is implemented
explicitly with a small `Dual2` type.

Each value carries:

- the ordinary field value,
- derivative with respect to $F$,
- derivative with respect to $k$.

So one dual number is:

```math
(x,\; \partial x/\partial F,\; \partial x/\partial k)
```

Every arithmetic operation is overloaded so derivative information moves
through the same update steps as the original values.

The core rules are the chain-rule versions of ordinary arithmetic.

For two dual values

```math
a = (a,\; a_F,\; a_k), \qquad b = (b,\; b_F,\; b_k)
```

the repo uses:

```math
a+b = (a+b,\; a_F+b_F,\; a_k+b_k)
```

```math
a-b = (a-b,\; a_F-b_F,\; a_k-b_k)
```

```math
ab = (ab,\; a_F b + a b_F,\; a_k b + a b_k)
```

That multiplication rule is just the product rule from calculus written in dual
number form.

That means the solver can answer:

```math
L,\qquad \frac{\partial L}{\partial F},\qquad \frac{\partial L}{\partial k}
```

after one forward-style pass.

At the loss level, the gradient accumulation is:

```math
\frac{\partial L}{\partial F}
= \frac{1}{2N}\sum_{i=1}^{N}\left[2(u_i-u_i^*)\frac{\partial u_i}{\partial F}+2(v_i-v_i^*)\frac{\partial v_i}{\partial F}\right]
```

```math
\frac{\partial L}{\partial k}
= \frac{1}{2N}\sum_{i=1}^{N}\left[2(u_i-u_i^*)\frac{\partial u_i}{\partial k}+2(v_i-v_i^*)\frac{\partial v_i}{\partial k}\right]
```

That is exactly why forward-mode AD is useful here: the field sensitivities are
already available at the end of the forward-style run.

Why use this method here?

- there are only two unknowns,
- the dual-number implementation stays understandable,
- it produces gradients in one evaluation instead of five.

Why not reverse-mode AD or adjoints?

Because that would be a different project:

- more machinery,
- more complexity,
- less transparent to readers,
- not necessary for a two-parameter demonstration.

At code level, the key design choice is:

- overload `Add`, `Sub`, and `Mul` for `Dual2`
- run almost the same logical update pipeline as the primal solver
- accumulate both value and derivative information together

That makes the AD implementation small enough to audit directly.

## A curvature intuition: what the Hessian would mean here

The gradient tells you the local downhill direction. But it does not tell you
the full local shape of the loss surface.

That fuller second-order shape is captured by the Hessian:

```math
H(F,k) =
\begin{bmatrix}
\dfrac{\partial^2 L}{\partial F^2} & \dfrac{\partial^2 L}{\partial F \partial k} \\
\dfrac{\partial^2 L}{\partial k \partial F} & \dfrac{\partial^2 L}{\partial k^2}
\end{bmatrix}
```

You do not need the repo to compute this matrix explicitly to understand why it
matters.

It answers questions like:

- is the loss valley narrow or wide,
- do $F$ and $k$ interact strongly,
- does one direction curve much more sharply than the other,
- should a fixed learning rate be trusted locally.

This is why backtracking helps even without an explicit Hessian:

- the gradient gives a local direction,
- the line search checks whether the proposed step behaves well in the real
  curved loss surface.

So backtracking is a simple way to respect curvature without implementing a
full second-order optimizer.

## Algorithm 4: gradient descent

Once a gradient exists, the repo can update parameters by moving downhill.

The basic idea is:

```math
p_{\text{new}} = p_{\text{old}} - \alpha \nabla L
```

Written per parameter, that is:

```math
F_{\text{new}} = \operatorname{clamp}\!\left(F_{\text{old}} - \alpha \frac{\partial L}{\partial F}\right)
```

```math
k_{\text{new}} = \operatorname{clamp}\!\left(k_{\text{old}} - \alpha \frac{\partial L}{\partial k}\right)
```

where `α` is the learning rate.

In everyday language:

> if the slope says “loss rises this way,” step in the opposite direction.

The repo includes:

- finite-difference descent,
- forward-gradient descent.

These are useful because they show the basic search idea directly.

Why are they not the final answer?

Because fixed-step descent is fragile. A step size that works in one regime can
be too large or too small in another.

This is why the repo keeps fixed-step descent as a comparison point, not as the
final browser-facing optimizer.

## Algorithm 5: backtracking line search with Armijo condition

This is the most practical optimizer in the repo.

The idea is:

1. compute a descent direction from the AD gradient,
2. try a proposed step size,
3. if the loss decreases enough, accept it,
4. otherwise shrink the step and try again.

The Armijo condition is the “decreases enough” rule. You do not need the full
formalism to understand its purpose.

Its job is:

> reject steps that only look good on paper but do not reduce the real loss
> enough when the solver is actually rerun.

Why is this better than plain fixed-step updates?

- more stable,
- less sensitive to a bad initial learning rate,
- more honest about nonlinear behavior.

Why not use something heavier like Adam, L-BFGS, or a trust-region solver?

Those are real options, but for this repo they would add:

- more hyperparameters,
- more implementation complexity,
- a less transparent story for beginners,
- more room to accidentally optimize the optimizer instead of the scientific
  question.

The current choice is conservative on purpose.

At code level, backtracking introduces an important difference in cost
accounting:

- one AD gradient evaluation gives a proposed direction,
- then extra candidate losses may be evaluated while shrinking the step.

So “evaluation count” for backtracking is intentionally larger than “iteration
count.” They are not the same metric.

The acceptance test is the Armijo condition:

```math
L(p_{\text{candidate}}) \le L(p_{\text{current}}) + c(\nabla L \cdot \Delta p)
```

where the directional decrease term is the gradient dotted with the proposed
parameter change:

```math
\text{directional decrease} = \nabla L \cdot \Delta p
```

Because the descent step points downhill, this quantity should be negative. The
condition says:

> accept the step only if the actual decrease is large enough relative to what
> the local slope predicted.

That is the deeper optimization idea:

- the gradient is a first-order model,
- the real loss surface is nonlinear,
- Armijo backtracking checks whether the first-order picture was locally good
  enough.

## Why noise is part of the algorithm story

`add_uniform_noise` exists because a perfectly clean target can make inverse
recovery look too easy.

The repo adds deterministic bounded noise and reruns recovery to ask:

> does the algorithm still recover sensible parameters when the target is not
> perfectly clean?

Why deterministic noise?

Because reproducibility matters. The repo wants:

- same seed -> same noisy target,
- same method -> same comparison,
- cleaner experiment logs.

More specifically, `SplitMix64` is used to produce repeatable bounded
pseudorandom values, and the `u` and `v` fields use different seeds so the
noise streams are not identical.

If `r` is a pseudorandom value in `[0, 1)`, the noise rule is:

```math
\text{noise} = a(2r-1), \qquad
\tilde{x} = \operatorname{clamp}(x + \text{noise}, 0, 1)
```

So the perturbation is uniform in `[-amplitude, +amplitude]` before clamping.

## What identifiability means in this repo

An inverse problem is not only about optimization. It is also about
identifiability.

Identifiability asks:

> does the observed data contain enough information to distinguish the unknown
> parameters clearly?

In this repo, that means asking whether the final field gives enough signal to
separate:

- one candidate $F, k$ pair,
- from another nearby $F, k$ pair.

This matters because two different parameter pairs can sometimes produce
visually similar or numerically close final patterns.

That is one reason the repo:

- uses a known target,
- measures actual loss values,
- compares grid search and gradient methods,
- and studies noise sensitivity.

Those are not only optimization experiments. They are also indirect checks on
how identifiable the two-parameter problem is under the chosen setup.

## What would happen if the parameter count grew

This repo keeps only two unknown parameters. That makes the inverse problem much
cleaner than a larger one.

If the parameter count grew, several things would change at once:

### 1. Search-space size would explode

Grid search would become unrealistic quickly because the candidate lattice would
grow combinatorially.

### 2. Forward-mode AD would become less attractive

Forward-mode AD carries one tangent channel per parameter direction. So adding
many parameters increases:

- arithmetic overhead,
- memory overhead,
- implementation burden.

### 3. Identifiability would usually get worse

More unknowns means more ways for different parameter combinations to explain
similar outputs. That can flatten parts of the loss surface or create more
ambiguous valleys.

### 4. Adjoint or reverse-mode methods would become more compelling

When the parameter count is large, methods that produce many parameter
gradients more efficiently per solve become much more attractive.

So the repo’s algorithm choices should be read correctly:

- they are strong for a small two-parameter study,
- they are not a claim that the same stack is automatically ideal for a large
  inverse-design problem.

## Why there is no full adjoint derivation here

An adjoint derivation would be the natural next theoretical step for a
large-parameter PDE inverse problem.

Very roughly, the adjoint idea is:

1. solve the forward problem,
2. define a reverse sensitivity problem tied to the loss,
3. propagate sensitivity backward through the dynamics,
4. recover gradients for many parameters efficiently.

Why does the repo not do that?

- it would add a large second mathematical system,
- it would add substantial implementation complexity,
- it would be harder for beginners to audit,
- and it is not necessary for a two-parameter demonstration.

So the omission is intentional, not accidental.

This chapter mentions adjoints so readers understand the tradeoff boundary:

- for small parameter count, forward-mode AD is a sensible fit;
- for large parameter count, adjoint-style thinking becomes much more relevant.

## Why the repo clamps values and bounds parameters

You will see several forms of clamping and bounded ranges.

Examples:

- noisy field values are clamped into `[0, 1]`,
- optimization keeps `F` and `k` inside chosen bounds.

Why?

- keeps the experiment in a meaningful region,
- prevents silly parameter explosions,
- keeps comparisons fair,
- reduces misleading optimizer behavior.

This is not a proof of global optimality. It is disciplined numerical
guard-railing.

This also means the optimizer results should be read correctly:

- they are the best answers found inside the defined search policy,
- not proofs that no better answer exists anywhere.

## The inverse algorithm in compact form

If you want the whole inverse method in one compact block, it is:

```text
Choose true parameters (F*, k*)
Generate target fields (u*, v*)
Optionally add deterministic bounded noise

Choose initial guess (F₀, k₀)
Repeat:
  evaluate loss L(F, k)
  compute gradient by finite differences or forward-mode AD
  propose a descent step
  if using backtracking:
    shrink step until Armijo condition is satisfied
  clamp parameters into allowed bounds
Until iteration limit or no acceptable step remains
```

That is the full mathematical algorithm behind the inverse experiments and the
browser inverse page.

## Why these methods together tell a stronger story

No single method is enough.

Here is what each method contributes:

- **grid search**
  coarse global baseline
- **finite differences**
  trusted but more expensive gradient reference
- **forward-mode AD**
  cheaper gradient path for this small parameter count
- **fixed-step descent**
  simple optimizer baseline
- **backtracking descent**
  more stable practical optimizer
- **noise sweeps**
  robustness evidence

Together they answer a more defensible question:

> not only “did one optimizer get a nice answer once?” but also “how much work
> did it cost, how trustworthy are the gradients, and how brittle is the
> result?”

## What the browser path changes and what it does not

`src/wasm.rs` does not invent a separate inverse algorithm. That is important.

The browser path keeps the same logic:

- generate target,
- add noise if requested,
- run AD backtracking optimization,
- return result history.

What changes is the interface:

- browser-facing arguments,
- JSON output,
- worker-based execution,
- JS/WASM memory boundary.

So the browser result is not “a different method.” It is the same small inverse
method executed through a browser-deliverable runtime path.

The browser path also adds one practical reporting layer: the result is
serialized as JSON with:

- final parameters,
- absolute parameter errors,
- noisy and clean losses,
- evaluation count,
- iteration history.

That makes the browser output easy to inspect in a UI or benchmark script
without changing the underlying inverse math.

## The central tradeoff of the whole inverse section

The biggest design choice is this:

> prefer a small, inspectable, reproducible inverse problem over a larger but
> harder-to-defend one.

That choice affects everything:

- only two unknown parameters,
- final-field loss,
- forward-mode AD instead of adjoint machinery,
- simple but disciplined optimizer design,
- explicit noise testing,
- browser delivery as part of the research question.

If the goal had been raw industrial inverse-design power, the algorithm stack
would likely look very different.

But for this repo’s aim, these methods fit together cleanly.
