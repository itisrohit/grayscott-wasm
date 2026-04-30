---
sidebar_position: 9
title: Forward Solver Math
---

# Forward Solver Math

This page explains the math and algorithm behind the forward solver in
`src/solver.rs`.

## The continuous Gray-Scott model

At the PDE level, the model is:

```math
\frac{\partial u}{\partial t} = D_u \nabla^2 u - u v^2 + F(1-u)
\frac{\partial v}{\partial t} = D_v \nabla^2 v + u v^2 - (F+k)v
```

Read this term by term:

- $\dfrac{\partial u}{\partial t}$, $\dfrac{\partial v}{\partial t}$:
  rate of change over time
- $D_u \nabla^2 u$, $D_v \nabla^2 v$:
  diffusion of the two fields
- $u v^2$:
  nonlinear reaction term
- $F(1-u)$:
  feed of $u$
- $(F+k)v$:
  total removal pressure on $v$

This is the real mathematical starting point. Everything in `src/solver.rs` is
an approximation of these two equations.

## Start with the physical story

The Gray-Scott model tracks two fields:

- `u`
- `v`

Each one changes for two reasons:

1. **diffusion**
   values spread to nearby cells
2. **reaction**
   local values interact nonlinearly

The standard Gray-Scott equations can be read in plain text like this:

```text
change in u
  = diffusion of u
  - reaction term
  + feed term

change in v
  = diffusion of v
  + reaction term
  - removal term
```

The specific reaction term used here is:

```math
u v^2
```

That term is why the dynamics are nonlinear. If the model were only “add,
subtract, and average,” the behavior would be much less interesting.

Mathematically, that nonlinearity matters because it bends the loss landscape
in the inverse problem. If the dynamics were linear, recovery would often be
much simpler.

## The exact baseline parameter story in this repo

The repo keeps $D_u$, $D_v$, and $\Delta t$ fixed in the main experiments and
focuses the inverse task on:

- $F$ = feed rate
- $k$ = kill rate

Those fit into the update as:

```text
feed adds fresh u
kill removes v
```

This is why $F$ and $k$ are the two “mystery knobs” the inverse section tries
to recover.

The default forward parameters in `GrayScottParams::default()` are:

```math
F = 0.060,\quad
k = 0.062,\quad
D_u = 0.16,\quad
D_v = 0.08,\quad
\Delta t = 1.0
```

That matters because many benchmarks and browser pages begin from this default
baseline before any overrides are applied.

## Why a PDE has to become a grid algorithm

The scientific model is continuous:

- every point in space,
- every instant in time.

But the computer stores arrays. So the solver makes three concrete decisions.

### Decision 1: discretize space

The field becomes a 2D grid:

```text
width x height
```

Every cell stores:

- one $u$ value
- one $v$ value

### Decision 2: discretize time

Instead of “continuous change,” the solver takes repeated time steps:

```math
(u^0, v^0) \rightarrow (u^1, v^1) \rightarrow (u^2, v^2) \rightarrow \cdots
```

If we use $n$ for the current time step and $n+1$ for the next one, the solver
is really computing:

```math
u^n \rightarrow u^{n+1}, \qquad v^n \rightarrow v^{n+1}
```

### Decision 3: approximate local curvature with neighbors

Diffusion depends on a Laplacian operator. In this repo, that is approximated
with a 5-point stencil:

```math
\nabla^2 u_{i,j} \approx u_{i-1,j} + u_{i+1,j} + u_{i,j-1} + u_{i,j+1} - 4u_{i,j}
```

In the scalar code this appears directly inside `update_cell_scalar`.

In other words, the mathematical derivative information is replaced with local
array arithmetic. That replacement is the central numerical approximation of
the forward solver.

Using grid indices, the discrete Laplacian at cell `(i, j)` is:

```math
\nabla^2 u_{i,j} \approx u_{i-1,j} + u_{i+1,j} + u_{i,j-1} + u_{i,j+1} - 4u_{i,j}
```

```math
\nabla^2 v_{i,j} \approx v_{i-1,j} + v_{i+1,j} + v_{i,j-1} + v_{i,j+1} - 4v_{i,j}
```

This is the standard 5-point stencil approximation used by the scalar update.

## What one cell update means

For each cell, the solver does this in order:

1. find the four neighbors,
2. compute the Laplacian for $u$,
3. compute the Laplacian for $v$,
4. compute the reaction term $uv^2$,
5. apply diffusion, reaction, feed, and kill,
6. write the result into the next buffer.

In plain text, the implemented update is:

```math
u^{n+1}_{i,j} = u^n_{i,j} + \Delta t \left(D_u \nabla^2 u^n_{i,j} - u^n_{i,j}(v^n_{i,j})^2 + F(1-u^n_{i,j})\right)
```

```math
v^{n+1}_{i,j} = v^n_{i,j} + \Delta t \left(D_v \nabla^2 v^n_{i,j} + u^n_{i,j}(v^n_{i,j})^2 - (F+k)v^n_{i,j}\right)
```

That exact structure is what turns the continuous model into repeated CPU work.

Using per-cell notation, the same update is:

```math
u_{i,j}^{n+1} = u_{i,j}^{n} + \Delta t \left[D_u \nabla^2 u_{i,j}^{n} - u_{i,j}^{n}(v_{i,j}^{n})^2 + F(1-u_{i,j}^{n})\right]
```

```math
v_{i,j}^{n+1} = v_{i,j}^{n} + \Delta t \left[D_v \nabla^2 v_{i,j}^{n} + u_{i,j}^{n}(v_{i,j}^{n})^2 - (F+k)v_{i,j}^{n}\right]
```

That is the explicit Euler discretization of the continuous PDE.

Technically, one `step()` call updates the entire grid once. One
`run(steps, params)` call repeats that global update many times.

## Why periodic boundaries are used

At the edges of the grid, the solver wraps around:

- left edge reads from the right edge,
- top edge reads from the bottom edge.

That is called a **periodic boundary condition**.

Why choose it here?

- it keeps the edge handling simple and deterministic,
- it avoids special edge formulas,
- it matches the small teaching-style simulation well.

Why not fixed-value boundaries or no-flux boundaries?

- they are possible,
- but they add another modeling choice,
- and they would complicate comparison and interpretation.

This repo chose the simpler closed story.

In code, this shows up as “wrap if we hit an edge” logic such as:

- `if x == 0 { width - 1 } else { x - 1 }`
- `if x + 1 == width { 0 } else { x + 1 }`

That is how periodic-boundary math becomes integer index arithmetic.

## Why double buffering is necessary

The solver stores:

- current $u$, current $v$
- next $u$, next $v$

That is not waste. It prevents a subtle bug.

If the solver updated `u` and `v` in place, then cells processed later in the
loop would accidentally read partly updated neighbors. That would mean one time
step is no longer mathematically consistent.

So the solver:

1. reads from the current buffers,
2. writes into the next buffers,
3. swaps them at the end.

That is why you see `core::mem::swap` in `step`.

## Why the solver uses explicit Euler stepping

The time integration here is the simple explicit form:

```math
x^{n+1} = x^n + \Delta t \cdot \mathrm{change}(x^n)
```

Why use that?

- it is easy to explain,
- easy to test,
- easy to mirror in scalar, SIMD, AD, and browser paths,
- and sufficient for the scope of this repo.

Why not a higher-order integrator?

Options like Runge-Kutta could improve accuracy in some settings, but they
would also:

- increase implementation complexity,
- increase runtime cost per step,
- complicate AD and browser explanations,
- make the “same algorithm across all paths” story weaker.

That tradeoff is not worth it for this artifact’s goal.

There is also a code-maintenance reason:

- explicit Euler keeps the scalar path simple,
- keeps the dual-number inverse path aligned,
- keeps browser and native behavior easier to compare.

The tradeoff is the classic one:

- explicit Euler is easy and cheap per step,
- but it is less accurate and less stable than more advanced time integrators.

This repo accepts that tradeoff because the research question is about a
portable, inspectable computational stack, not maximum numerical sophistication.

## Why the data layout is split into separate arrays

The solver stores:

- all `u` values in one array,
- all `v` values in another array.

That is a structure-of-arrays design.

Why use it?

- it makes neighbor reads predictable,
- it fits SIMD better,
- it makes WASM typed-array views simpler,
- and it keeps the scalar and browser paths aligned.

If the repo had used an array of structs like:

```text
[(u0, v0), (u1, v1), ...]
```

then each pass would have to hop back and forth more awkwardly through memory.

It also would have made the WASM typed-array exposure less clean. With separate
arrays, `u_view()` and `v_view()` can expose direct contiguous memory ranges to
JavaScript.

## Why the SIMD path looks the way it does

The SIMD path in `step_wasm_simd` does not rewrite the whole algorithm. It
keeps the same math and changes how several neighboring floats are processed.

The practical idea is:

```math
\text{scalar: } 1 \text{ lane} \rightarrow 1 \text{ value}, \qquad
\text{SIMD: } 1 \text{ instruction} \rightarrow \text{multiple values}
```

In this repo:

- interior cells use 4-wide `f32x4` operations,
- edge cells still use the scalar path.

Why keep the edges scalar?

- boundary logic is branchy,
- the interior is where most of the repeated work lives,
- keeping edges scalar reduces correctness risk.

That is a classic engineering tradeoff:

- vectorize the heavy middle,
- keep the tricky boundaries simple.

It is also why `step_simd()` is written as a controlled entry point instead of
pretending every target supports SIMD. On non-SIMD targets, the code falls back
to the scalar path. That keeps one API shape across environments.

You can think of the SIMD path mathematically as:

```text
same stencil formula
same Euler update
same parameters
different instruction grouping
```

So SIMD changes the execution width, not the model.

## Why the seed is part of the algorithm, not just presentation

The forward solver is almost always started with `seed_square`.

That routine writes:

```math
u = 0.50, \qquad v = 0.25
```

inside a centered square, while the surrounding field stays near:

```math
u = 1.0, \qquad v = 0.0
```

Why does this count as part of the algorithm story?

Because pattern formation depends on the initial disturbance. The seed is not
just a visual choice. It affects:

- whether a visible pattern develops,
- what the target-generation recipe means,
- what the inverse problem is actually recovering against.

That is why both the forward and inverse code treat seeding as a formal part of
the setup.

In other words, the target-generating map is not just:

```math
(F, k) \mapsto \text{final field}
```

It is really:

```math
(\text{seed}, F, k, D_u, D_v, \Delta t, \text{steps}) \mapsto \text{final field}
```

The repo only treats `F` and `k` as unknown because everything else is held
fixed by design.

## Why there are both `new` and `from_fields`

`GrayScott::new` creates the standard fresh simulation state.

`GrayScott::from_fields` creates a simulation from already existing field
arrays.

Why expose both?

- `new` is the natural path for forward runs and target generation,
- `from_fields` is useful for validation, controlled tests, and external data
  injection.

This is a small API choice, but it makes the solver more reusable than a pure
"demo only" implementation.

## Why checksums matter to the forward path

Several benchmark paths report a checksum over $u$ and $v$.

That checksum is not a proof of correctness, but it is a cheap guard against:

- obviously wrong field output,
- accidental benchmark paths that skip real work,
- scalar/SIMD drift that should not happen.

That is why the forward algorithm story is not only “how fast is it?” but also
“how do we know we measured the same actual computation?”

## The forward solver in compact algorithm form

If you want the whole forward method in one compact block, it is:

```text
Given:
  initial fields u⁰, v⁰
  parameters F, k, D_u, D_v, dt

For n = 0, 1, 2, ..., steps-1:
  For each cell (i, j):
    compute wrapped neighbors
    compute ∇²u and ∇²v with 5-point stencil
    compute reaction r = u(i,j) v(i,j)²
    update u(i,j) and v(i,j) with explicit Euler
  swap current and next buffers
```

That block is the computational backbone of the whole repo.

## Why not use a more exotic stencil

The Laplacian here is the simple 5-point pattern. The repo could have used:

- a 9-point stencil,
- anisotropic weights,
- spectral methods,
- adaptive meshes.

But that would have changed the question.

This project is not trying to prove the best possible discretization. It is
trying to prove that a modest, inspectable solver can support the full
forward-to-browser-to-inverse chain.

So the chosen solver is:

- standard,
- small,
- testable,
- and easy to match across runtimes.

## Where this solver fits in the bigger puzzle

Everything else depends on this page’s logic.

- validation checks whether this solver matches reference behavior,
- performance tests measure how fast this solver runs,
- rendering turns this solver’s field into visible pixels,
- inverse recovery repeatedly reruns this solver while searching for better
  parameters.

So the forward solver is the foundation. If this part were wrong, every later
result would be suspect.

## What would have happened with other choices?

This is the practical tradeoff summary.

### If we used a more accurate integrator

- probably more numerically refined,
- definitely more code,
- more runtime cost per step,
- more complexity in AD and browser explanations.

### If we used a GPU solver

- likely much higher forward throughput,
- but more complexity in deployment, validation, and browser portability,
- and a weaker “CPU-only browser-deliverable” claim.

### If we updated in place

- less memory,
- but wrong time-step semantics.

### If we changed the boundary model

- possibly different pattern behavior,
- but more modeling choices to defend.

The current solver is not “the only way.” It is the way that best matches the
artifact’s real aim: clear, reproducible, cross-runtime scientific computing.
