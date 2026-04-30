---
sidebar_position: 3
title: Gray-Scott Basics
---

import { CellUpdateVisualizer } from "../src/components/GuideVisuals";

# Gray-Scott Basics

## What is being simulated?

The Gray-Scott model tracks two quantities, usually called `u` and `v`.

If you are not comfortable with the word *concentration*, use this simpler
picture:

> Imagine two kinds of colored material spread over a tiled board.

Each little tile stores how much of material `u` and material `v` is present.

In more scientific language, those amounts are concentrations. But you do not
need a strong math background to follow the basic idea.

You can think of them as two chemical concentrations spread across a 2D grid.
Each cell in the grid stores one `u` value and one `v` value.

Over time, those values change because of two effects:

- **diffusion**: values spread to nearby cells,
- **reaction**: `u` and `v` interact locally through nonlinear rules.

## If you hate equations, start here instead

The simulation keeps repeating a simple story:

1. each tile looks at its nearby tiles,
2. some material spreads,
3. some material reacts,
4. fresh material can be fed in,
5. some material can be removed,
6. the process repeats many times.

That is enough to generate surprisingly rich patterns.

## The math idea in one picture

At each time step, each cell is updated using:

```text
new value
  = old value
  + diffusion effect from neighbors
  + local reaction effect
  + feed/kill terms
```

For this project, the important mental model is:

```text
nearby cells matter   +   local chemistry matters   +   time repeats
```

That repeated local update is enough to create large visible patterns.

You can read this as:

> The next moment depends on the current moment, nearby tiles, and a few rules
> about spreading and reacting.

<CellUpdateVisualizer />

## Why do patterns appear?

If the reactions and diffusion rates balance in the right way, the system stops
being visually uniform. Spots, worms, stripes, and other textures appear.

An everyday analogy is:

> Imagine drops of ink spreading in water while also transforming each other.
> If the balance is right, the picture does not stay smooth. It organizes into
> visible structure.

That makes Gray-Scott useful for both:

- science and numerical methods,
- visual teaching and browser demos.

## What parameters matter most here?

This project keeps many parameters fixed and focuses on two:

- `F`: feed rate
- `k`: kill rate

Those two are the stars of the inverse problem.

You can read them in beginner language like this:

- `F` says how strongly fresh `u` material is supplied,
- `k` says how strongly `v` is removed.

Changing them changes the balance of the system.

## What is a time step?

A time step is one small update of the whole grid.

The computer does not jump directly from “start” to “final pattern.” Instead it
does:

```text
step 1 -> step 2 -> step 3 -> ... -> step 1000
```

At each step, every cell is updated using the current rules.

So when you see commands like “run 100 steps” or “run 1000 steps,” that means:

- do the same update rule again and again,
- let the pattern gradually evolve,
- observe what the field looks like after many repetitions.

The inverse question is:

> If I see a final pattern, can I work backward and recover the `F` and `k`
> values that probably generated it?

## Why is that hard?

Because the final pattern is not a simple formula.

- small parameter changes can alter behavior,
- different parameter pairs can sometimes look similar,
- noise makes the problem harder,
- local optimization can get stuck or drift.

That is why the repo does not only show final recovered parameters. It also
measures losses, gradients, overhead, and failure behavior.

## What is the grid?

The simulation lives on a 2D rectangular lattice such as:

- `64 x 64`
- `128 x 128`
- `256 x 256`
- `512 x 512`

Each grid cell stores floating-point values for `u` and `v`.

If you are new to floating-point numbers, just read them as:

> decimal-like numbers stored by the computer.

They are used because the field values are not just 0 or 1. They vary
continuously.

## A cell-update diagram

The update at one cell uses a 5-point neighborhood:

```mermaid
flowchart TD
    U[up] --> C[center]
    L[left] --> C
    R[right] --> C
    D[down] --> C
```

So the solver is always doing two things at once:

1. looking locally around each cell,
2. repeating that local rule across the whole grid.

## What is the seed?

The simulation starts from a mostly uniform field and then inserts a small
center square with different values. That seed creates the disturbance that
starts the pattern.

This matters because if the initial condition were changed, the final pattern
could also change. That is one reason the paper is careful about its limits.

You can think of the seed as the “starting disturbance.” Without some
disturbance, a completely uniform system would stay boring for much longer.

## What does the solver actually do each round?

At a very beginner-friendly level:

```mermaid
flowchart TD
    A[Start with the current grid]
    B[Look at one cell and its neighbors]
    C[Apply spreading rules]
    D[Apply local reaction rules]
    E[Store the updated value]
    F[Repeat for every cell]
    G[Repeat for many time steps]

    A --> B --> C --> D --> E --> F --> G
```

That is the core loop behind the whole project.

## Why this is a good teaching model

Gray-Scott is popular in teaching because the equations are small enough to
study, but the output is rich enough to feel surprising.

That makes it a strong example for learning:

- nonlinear dynamics,
- grid-based simulation,
- floating-point computation,
- inverse problems,
- browser-delivered scientific software.
