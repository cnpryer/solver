///! # solver-graph
///!
///! `SmallGraph` can be used for operations on `Nodes` and `Edges`. Constructing a `SmallGraph`
///!  requires using the `graph!` macro to index `nodes` and `edges`.
///!
///! ```rust
///! use solve_graph::small_graph::{graph, nodes, edges};
///!
///! let nodes = nodes(vec![0, 1, 2]);
///! let edges = edges(vec![Some(vec![edge(0, 1), edge(0, 2)]), Some(vec![edge(1, 2)]), None]);
///! let graph = graph![nodes, edges];
///! ```
use std::fmt::Debug;
mod helpers;
mod ops;
mod queue;
mod small_array;
pub mod small_graph;

pub trait Value: Default + Copy + Clone {}
impl<V: Default + Copy + Clone> Value for V {}

pub trait Position: Default + Copy + Clone + Into<usize> + Debug {}
impl<P: Default + Copy + Clone + Into<usize> + Debug> Position for P {}
