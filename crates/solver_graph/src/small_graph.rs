use crate::{helpers, ops, small_array::SmallArray, Position, Value};
pub use helpers::{edge, edges, nodes, weighted_edge};
pub use ops::shortest_path;
use std::ops::Deref;

#[macro_export]
/// Use `graph![nodes, edges]` to create a `SmallGraph`.
///
/// ```rust
/// use solve_graph::small_graph::{graph, nodes, edges};
///
/// let nodes = nodes(vec![0, 1, 2]);
/// let edges = edges(vec![Some(vec![edge(0, 1), edge(0, 2)]), Some(vec![edge(1, 2)]), None]);
/// let graph = graph![nodes, edges];
/// ```
macro_rules! graph {
    ($nodes:expr, $edges:expr) => {{
        $crate::small_graph::small_graph($nodes, $edges)
    }};
}

/// `SmallGraph`s are compact data structures composed of `Nodes` and `Edges`.
///
///  ```rust
/// use solve_graph::{graph, nodes, edges};
///
/// let nodes = nodes(vec![0, 1, 2]);
/// let edges = edges(vec![Some(vec![edge(0, 1), edge(0, 2)]), Some(vec![edge(1, 2)]), None]);
/// let graph = graph![nodes, edges];
/// ```
#[derive(Debug)]
pub struct SmallGraph<V: Value, P: Position> {
    nodes: Nodes<V>,
    edges: Edges<P, V>,
}

/// The `SmallGraph` is composed of `Nodes` and `Edges`.
///
/// ```rust
/// use solver_graph::small_graph::graph;
///
/// let graph = graph![nodes(vec![]), edges(vec![]))];
/// ```
pub fn small_graph<V: Value, P: Position>(nodes: Nodes<V>, edges: Edges<P, V>) -> SmallGraph<V, P> {
    SmallGraph { nodes, edges }
}

impl<V: Value, P: Position> SmallGraph<V, P> {
    /// The `SmallGraph` struct composes the `Nodes` and `Edges` for efficient operations.
    ///
    /// ```rust
    /// use solve_graph::small_graph::SmallGraph;
    ///
    /// let graph = SmallGraph::new();
    /// ```
    pub(crate) fn new() -> Self {
        Self {
            nodes: Nodes(Vec::new()),
            edges: Edges(Vec::new()),
        }
    }

    /// Get the `Nodes` of the `SmallGraph`.
    ///
    /// ```rust
    /// use solve_graph::small_graph::SmallGraph;
    ///
    /// let graph = SmallGraph::new();
    /// let nodes = graph.nodes();
    /// ```
    pub(crate) fn nodes(&self) -> &Nodes<V> {
        &self.nodes
    }

    /// Get the `Edges` of the `SmallGraph`.
    ///
    /// ```rust
    /// use solve_graph::small_graph::SmallGraph;
    ///
    /// let graph = SmallGraph::new();
    /// let edges = graph.edges();
    /// ```
    pub(crate) fn edges(&self) -> &Edges<P, V> {
        &self.edges
    }
}

/// The `SmallGraph` struct composes the `Nodes` and `Edges` for efficient operations.
///
/// ```rust
/// use solver_graph::small_graph::SmallGraph;
///
/// let graph = SmallGraph::default();
/// ```
impl<V: Value, P: Position> Default for SmallGraph<V, P> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct Nodes<V>(pub(crate) Vec<V>);

impl<V> Nodes<V> {
    /// Get an indexed `Node`.
    ///
    /// ```rust
    /// use solver_graph::small_graph::nodes;
    ///
    /// let nodes = nodes(vec![0, 1, 2]);
    /// let first = nodes.get(0).unwrap()
    /// ```
    pub(crate) fn get(&self, index: usize) -> Option<&V> {
        self.0.get(index)
    }

    /// Get the first `Node`.
    ///
    /// ```rust
    /// use solver_graph::small_graph::nodes;
    ///
    /// let nodes = nodes(vec![0, 1, 2]);
    /// let first = nodes.first().unwrap()
    /// ```
    pub(crate) fn first(&self) -> Option<&V> {
        self.0.first()
    }

    /// Get the last `Node`.
    ///
    /// ```rust
    /// use solver_graph::small_graph::nodes;
    ///
    /// let nodes = nodes(vec![0, 1, 2]);
    /// let last = nodes.last().unwrap()
    /// ```
    pub(crate) fn last(&self) -> Option<&V> {
        self.0.last()
    }

    /// Get the length of the `Nodes`.
    ///
    /// ```rust
    /// use solver_graph::small_graph::nodes;
    ///
    /// let nodes = nodes(vec![0, 1, 2]);
    /// let last = nodes.last().unwrap()
    /// ```
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Default, Clone, Debug)]
pub struct Edges<P: Position, V: Value>(pub(crate) Vec<SmallArray<Edge<P, V>>>);

#[derive(Default, Clone, Debug)]
pub struct Edge<P: Position, V: Value> {
    pub(crate) from: P,
    pub(crate) to: P,
    pub(crate) weights: Option<SmallArray<V>>,
}

impl<P: Position, V: Value> Edge<P, V> {
    pub(crate) fn weights(&self) -> Option<&SmallArray<V>> {
        self.weights.as_ref()
    }
}

impl<P: Position + PartialEq, V: Value + PartialEq> PartialEq for Edge<P, V> {
    fn eq(&self, other: &Self) -> bool {
        // TODO(cnpryer): Better weight handling
        let eq = self.from == other.from && self.to == other.to;
        match (&self.weights, &other.weights) {
            (Some(a), Some(b)) if eq => a == b,
            (None, None) if eq => true,
            _ => false,
        }
    }
}

impl<P: Position + Eq, V: Value + Eq> Eq for Edge<P, V> {}

impl<P: Position + PartialOrd, V: Value + PartialEq + Ord> PartialOrd for Edge<P, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.from.partial_cmp(&other.from) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.to.partial_cmp(&other.to) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.weights.partial_cmp(&other.weights)
    }
}

impl<P: Position + Ord, V: Value + Ord> Ord for Edge<P, V> {
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
        unimplemented!()
    }
}

impl<P: Position, V: Value> Edges<P, V> {
    /// Get an indexed `Edge`.
    ///
    /// ```rust
    /// use solver_graph::small_graph::{edges, edge};
    ///
    /// let edges = edges(vec![Some(vec![edge(0, 1), edge(0, 2)]), Some(vec![edge(1, 2)]), None]);
    /// let first = edges.get(0).unwrap()
    /// ```
    pub(crate) fn get(&self, index: P) -> Option<&SmallArray<Edge<P, V>>> {
        self.0.get(index.into())
    }

    /// Get the first `Edge`.
    ///
    /// ```rust
    /// use solver_graph::small_graph::{edges, edge};
    ///
    /// let edges = edges(vec![Some(vec![edge(0, 1), edge(0, 2)]), Some(vec![edge(1, 2)]), None]);
    /// let first = edges.first().unwrap()
    /// ```
    pub(crate) fn first(&self) -> Option<&SmallArray<Edge<P, V>>> {
        self.0.get(0)
    }

    /// Get the last `Edge`.
    ///
    /// ```rust
    /// use solver_graph::small_graph::{edges, edge};
    ///
    /// let edges = edges(vec![Some(vec![edge(0, 1), edge(0, 2)]), Some(vec![edge(1, 2)]), None]);
    /// let last = edges.last().unwrap()
    pub(crate) fn last(&self) -> Option<&SmallArray<Edge<P, V>>> {
        self.0.get(self.len() - 1)
    }

    /// Get the number of edges.
    ///
    /// ```rust
    /// use solver_graph::small_graph::nodes;
    ///
    /// let edges = edges(vec![Some(vec![edge(0, 1), edge(0, 2)]), Some(vec![edge(1, 2)]), None]);
    /// let count = edges.len()
    /// ```
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}

impl<P: PartialEq + Position, V: Value + PartialEq> PartialEq for Edges<P, V> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<P: Position + Eq, V: Value + Eq> Eq for Edges<P, V> {}

fn compare_to_static<P: Position + PartialEq, V: Value + PartialEq>(
    a: &Vec<Edge<P, V>>,
    b: &SmallArray<Edge<P, V>>,
) -> bool {
    a.as_slice() == b.deref()
}

fn compare_static<P: Position + PartialEq, V: Value + PartialEq>(
    a: &SmallArray<Edge<P, V>>,
    b: &SmallArray<Edge<P, V>>,
) -> bool {
    a.deref() == b.deref()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        helpers::{edge, nodes},
        small_graph::test_fixtures::{sample_edges, sample_nodes, sample_weighted_edges},
    };

    #[test]
    fn test_graph() {
        let (nodes, edges) = (sample_nodes(), sample_edges());
        let graph = small_graph(nodes.clone(), edges.clone());
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
        assert_eq!(
            edges.first(),
            Some(&SmallArray::Two([edge(0, 1), edge(0, 2)]))
        );
        assert_eq!(edges.last(), Some(&SmallArray::Empty));
    }

    #[test]
    fn test_weighted_edges() {
        let (nodes, edges) = (sample_nodes(), sample_weighted_edges());
        let graph = small_graph(nodes.clone(), edges.clone());
        assert_eq!(nodes.last(), graph.nodes().last());
        assert_eq!(edges.last(), graph.edges().last());
    }
}

#[cfg(test)]
pub(crate) mod test_fixtures {
    use crate::{
        helpers::{edge, edges, nodes, weighted_edge},
        small_graph::{Edges, Nodes},
    };

    pub(crate) fn sample_nodes() -> Nodes<i32> {
        println!("test");
        nodes(vec![0, 1, 2, 3])
    }

    pub(crate) fn sample_edges() -> Edges<usize, i32> {
        edges(vec![
            vec![edge(0, 1), edge(0, 2)],
            vec![edge(1, 2)],
            vec![edge(1, 0)],
            vec![],
        ])
    }

    pub(crate) fn sample_weighted_edges() -> Edges<usize, i32> {
        edges(vec![
            vec![weighted_edge(0, 1, vec![1]), weighted_edge(0, 2, vec![100])],
            vec![weighted_edge(1, 2, vec![1])],
            vec![weighted_edge(2, 0, vec![2])],
            vec![],
        ])
    }
}
