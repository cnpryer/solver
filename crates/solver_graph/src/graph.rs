///! # solver-graph
///!
///! `Graph` can be used for operations on `Nodes` and `Edges`.
///!
///! ```rust
///! struct Graph<T, U: Into<usize>> {
///!     nodes: Nodes<T>,
///!     edges: Edges<U, T>,
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
#[derive(Debug)]
pub(crate) struct Graph<T, U: Into<usize>> {
    nodes: Nodes<T>,
    edges: Edges<U, T>,
}

/// The `Nodes` struct composes `Node` data.
///
/// ```rust
/// use solver_graph::graph;
///
/// let graph = graph(nodes(vec![]), edges(vec![]));
/// ```
pub(crate) fn graph<T, U: Into<usize>>(nodes: Nodes<T>, edges: Edges<U, T>) -> Graph<T, U> {
    Graph { nodes, edges }
}

impl<T: Copy + Default, U: Copy + Into<usize>> Graph<T, U> {
    /// The `Graph` struct composes the `Nodes` and `Edges` for efficient operations.
    ///
    /// ```rust
    /// use solve_graph::Graph;
    ///
    /// let graph = Graph::new();
    /// ```
    pub(crate) fn new() -> Self {
        Self {
            nodes: Nodes(Vec::new()),
            edges: Edges(Vec::new()),
        }
    }

    /// Get the `Nodes` of the graph.
    ///
    /// ```rust
    /// use solve_graph::Graph;
    ///
    /// let graph = Graph::new();
    /// let nodes = graph.nodes();
    /// ```
    pub(crate) fn nodes(&self) -> &Nodes<T> {
        &self.nodes
    }

    /// Get the `Edges` of the graph.
    ///
    /// ```rust
    /// use solve_graph::Graph;
    ///
    /// let graph = Graph::new();
    /// let nodes = graph.edges();
    /// ```
    pub(crate) fn edges(&self) -> &Edges<U, T> {
        &self.edges
    }
}

/// The `Graph` struct composes the `Nodes` and `Edges` for efficient operations.
///
/// ```rust
/// use solver_graph::Graph;
///
/// let graph = Graph::default();
/// ```
impl<T: Copy + Default, U: Copy + Into<usize>> Default for Graph<T, U> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Nodes<T>(pub(crate) Vec<T>);

impl<T> Nodes<T> {
    /// Get an indexed `Node`.
    ///
    /// ```rust
    /// use solver_graph::nodes;
    ///
    /// let nodes = nodes(vec![0, 1, 2]);
    /// let first = nodes.get(0).unwrap()
    /// ```
    pub(crate) fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }

    /// Get the first `Node`.
    ///
    /// ```rust
    /// use solver_graph::nodes;
    ///
    /// let nodes = nodes(vec![0, 1, 2]);
    /// let first = nodes.first().unwrap()
    /// ```
    pub(crate) fn first(&self) -> Option<&T> {
        self.0.first()
    }

    /// Get the last `Node`.
    ///
    /// ```rust
    /// use solver_graph::nodes;
    ///
    /// let nodes = nodes(vec![0, 1, 2]);
    /// let last = nodes.last().unwrap()
    /// ```
    pub(crate) fn last(&self) -> Option<&T> {
        self.0.last()
    }

    /// Get the length of the `Nodes`.
    ///
    /// ```rust
    /// use solver_graph::nodes;
    ///
    /// let nodes = nodes(vec![0, 1, 2]);
    /// let last = nodes.last().unwrap()
    /// ```
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Edges<U: Into<usize>, T>(pub(crate) Vec<Option<Vec<Edge<U, T>>>>);

#[derive(Clone, Debug)]
pub(crate) struct Edge<U: Into<usize>, T> {
    pub(crate) from: U,
    pub(crate) to: U,
    pub(crate) weights: Option<Vec<T>>,
}

impl<U: PartialEq + Into<usize>, T> PartialEq for Edge<U, T> {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
    }
}

impl<U: PartialEq + Into<usize>, T> Eq for Edge<U, T> {}

impl<U: Copy + Into<usize>, T> Edges<U, T> {
    /// Get an indexed `Edge`.
    ///
    /// ```rust
    /// use solver_graph::{edges, edge};
    ///
    /// let edges = edges(vec![Some(vec![edge(0, 1), edge(0, 2)]), Some(vec![edge(1, 2)]), None]);
    /// let first = edges.get(0).unwrap()
    /// ```
    pub(crate) fn get(&self, index: usize) -> Option<&Vec<Edge<U, T>>> {
        get_edges(self, index)
    }

    /// Get the first `Edge`.
    ///
    /// ```rust
    /// use solver_graph::{edges, edge};
    ///
    /// let edges = edges(vec![Some(vec![edge(0, 1), edge(0, 2)]), Some(vec![edge(1, 2)]), None]);
    /// let first = edges.first().unwrap()
    /// ```
    pub(crate) fn first(&self) -> Option<&Vec<Edge<U, T>>> {
        get_edges(self, 0)
    }

    /// Get the last `Edge`.
    ///
    /// ```rust
    /// use solver_graph::{edges, edge};
    ///
    /// let edges = edges(vec![Some(vec![edge(0, 1), edge(0, 2)]), Some(vec![edge(1, 2)]), None]);
    /// let last = edges.last().unwrap()
    pub(crate) fn last(&self) -> Option<&Vec<Edge<U, T>>> {
        get_edges(self, self.len() - 1)
    }

    /// Get the number of edges.
    ///
    /// ```rust
    /// use solver_graph::nodes;
    ///
    /// let edges = edges(vec![Some(vec![edge(0, 1), edge(0, 2)]), Some(vec![edge(1, 2)]), None]);
    /// let count = edges.len()
    /// ```
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}

fn get_edges<U: Copy + Into<usize>, T>(
    edges: &Edges<U, T>,
    index: usize,
) -> Option<&Vec<Edge<U, T>>> {
    if let Some(edges) = edges.0.get(index) {
        edges.as_ref()
    } else {
        None
    }
}

impl<U: PartialEq + Into<usize>, T> PartialEq for Edges<U, T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<U: PartialEq + Into<usize>, T> Eq for Edges<U, T> {}

#[cfg(test)]
mod tests {
    use crate::{
        helpers::{edge, nodes},
        test_fixtures::{sample_edges, sample_nodes, sample_weighted_edges},
    };

    use super::*;

    #[test]
    fn test_graph() {
        let (nodes, edges) = (sample_nodes(), sample_edges());
        let graph = graph(nodes.clone(), edges.clone());
        assert_eq!(nodes.first(), graph.nodes().first());
        assert_eq!(edges.first(), graph.edges().first());
    }

    #[test]
    fn test_nodes() {
        let nodes = nodes(vec![1]);
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes.first(), Some(&1));
        assert_eq!(nodes.last(), Some(&1));
    }

    #[test]
    fn test_edges() {
        let edges = sample_edges();
        assert_eq!(edges.len(), 4);
        assert_eq!(edges.first(), Some(&vec![edge(0, 1), edge(0, 2)]));
        assert_eq!(edges.last(), None);
    }

    #[test]
    fn test_weighted_edges() {
        let (nodes, edges) = (sample_nodes(), sample_weighted_edges());
        let graph = graph(nodes.clone(), edges.clone());
        assert_eq!(nodes.last(), graph.nodes().last());
        assert_eq!(edges.last(), graph.edges().last());
    }
}
