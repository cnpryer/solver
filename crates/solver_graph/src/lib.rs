///! # solver-graph
///!
///! `Graph` can be used for operations on `Nodes` and `Edges`.
///!
///! ```rust
///! struct Graph<V, P: Into<usize>> {
///!     nodes: Nodes<V>,
///!     edges: Edges<P, V>,
///! }
///! ```
///!
///! Constructing a `Graph` requires using the `graph!` macro to index `nodes` and `edges`.
///!
///! ```rust
///! use solve_graph::{graph, nodes, edges};
///!
///! let nodes = nodes(vec![0, 1, 2]);
///! let edges = edges(vec![Some(vec![edge(0, 1), edge(0, 2)]), Some(vec![edge(1, 2)]), None]);
///! let graph = graph![nodes, edges];
///! ```
mod graph;
mod helpers;
mod ops;
mod queue;
mod small_array;

trait Value: Default + Copy + Clone {}
impl<V: Default + Copy + Clone> Value for V {}

trait Position: Default + Copy + Clone + Into<usize> {}
impl<P: Default + Copy + Clone + Into<usize>> Position for P {}
