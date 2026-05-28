# Nodus
## Vision
This project serves two purposes:

1. It is a learning exercise for me on how to use Rust to it's fullest, and to broaden my understanding of mathematics.
2. It is designed to be a tool to help with understanding mathematics.

The result should be a powerful and fast mathematical assistant. It should help with creating proofs, understanding theorems and applying them to real world scenarios. Part of this will be a glorified calculator but designed in such a way to help you visualize the problems you are solving.

Read the [purpose statement](./Purpose.md).
## Goals
- A geometric solver that explains **how** it got the answer.
- A simple sidecar for editing mathematic files.
- An embedded scripting runtime to simulate your theories.
- A cli that contains useful functionality.
- A tool to convert images into the geometric format.
- A fully featured GUI for creating shapes.
## Non-Goals
- A fully functioning AI assistant to help with all of your mathematical needs.
- A Desmos replacement.
- A calculation spreadsheet.
## Core Features
- Geometric evaluation of shapes.
- Embedded scripting.
- Transformation between mathematical formats (equations to diagrams and vice versa, etc...).
## Architecture Overview
- Uses a graph representation to store mathematical data.
- Stores the derivations from facts as a separate graph.
## Current Status
- It has not been started yet.
## Roadmap
### Phase 1
- Representation of geometric notation.
- Finds paths to solutions which it can then exploit to determine answers.
- Simple CLI tool.
### First Milestone
Solves all of [Catriona Agg’s Puzzles](https://notes.mathforge.org/notes/published/HomePage).
### Phase 2
- DSL
- Transformation between mathematical formats.
### Second Milestone
CLI tool is easy to use and expressive.
DSL is expressive and easy to use.
### Phase 3
- Graphical UI.
- Transformation from drawn graphical formats to internal representation.
### Third Milestone
Computations with the DSL are fast and it integrates well with other systems.
The GUI is responsive and comfortable.
The geometric representation is fast and easily viewable.
### Potential Extras
More mathematical formats.
Simulations run directly in the app.
## Research Areas
[Research Notes](Research/Research.md)
- Graph theory.
- Topology.
