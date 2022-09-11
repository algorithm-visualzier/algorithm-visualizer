# Algorithm Visualizer

## Authors

- Mikołaj Piróg (@aetn23)
- Mikołaj Wasiak (@RudyMis)

## Overview

A Rust application to edit graphs and run selected algorithms on them.

Inspiration: [this editor](https://csacademy.com/app/graph_editor/)

To create and edit graphs simply pick appropriate option from GUI. Run algorithms by selecting a starting node and desired algorithm. Move the graph around with WSAD, rotate it with QE, zoom with mouse wheel. Use forces sliders to adjust gravity.

It is also possible to write in the node: to do so right click on the node and start writing while keeping your cursor inside of the node. There is 10 character limit for readability.

## Libraries

Petgraph for graphs structures, egui_tetra for graphics. egui_tetra is a wrapper for
egui, a gui library, and tetra, a library for game development.

We also use dyn_partial_eq because of [this](https://dev.to/magnusstrale/rust-trait-objects-in-a-vector-non-trivial-4co5).

## Installation

Petgraph and egui install their dependencies from crates - no work required on our part.
Tetra has some dependencies that need to be installed manually -
see [this](https://tetra.seventeencups.net/installation).

