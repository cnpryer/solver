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
struct Graph<T, U: Into<usize>> {
    nodes: Nodes<T>,
    edges: Edges<U, T>,
}

/// Use `graph![nodes, edges]` to create a `Graph`.
///
/// ```rust
/// use solve_graph::{graph, nodes, edges};
///
/// let nodes = nodes(vec![0, 1, 2]);
/// let edges = edges(vec![Some(vec![edge(0, 1), edge(0, 2)]), Some(vec![edge(1, 2)]), None]);
/// let graph = graph![nodes, edges];
/// ```
macro_rules! graph {
    ($nodes:expr, $edges:expr) => {{
        Graph {
            nodes: $nodes,
            edges: $edges,
        }
    }};
}

impl<T: Copy + Default, U: Copy + Into<usize>> Graph<T, U> {
    /// The `Graph` struct composes the `Nodes` and `Edges` for efficient operations.
    ///
    /// ```rust
    /// use solve_graph::Graph;
    ///
    /// let graph = Graph::new();
    /// ```
    fn new() -> Self {
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
    fn nodes(&self) -> &Nodes<T> {
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
    fn edges(&self) -> &Edges<U, T> {
        &self.edges
    }
}

/// Sort the `Nodes` of the `Graph`.
///
/// ```rust
/// use solve_graph::{Graph, sort, node_attributes};
///
/// let mut graph = Graph::new();
/// let mut attrs = node_attributes!(&mut graph)
/// let mut graph = sort!(&mut graph, &mut attrs);
/// ```
fn sort<'a, T, U: Into<usize>>(
    _graph: &'a mut Graph<T, U>,
    _attrs: &'a mut NodeAttributes<T>,
) -> &'a mut Graph<T, U> {
    unimplemented!()
}

/// Query the shortest path from a `Graph`.
///
/// ```rust
/// use solve_graph::{graph, nodes, edges, shortest_path};
///
/// let nodes = nodes(vec![0, 1, 2]);
/// let edges = edges(vec![Some(vec![edge(0, 1), edge(0, 2)]), Some(vec![edge(1, 2)]), None]);
/// let graph = graph![nodes, edges];
/// let path = find_shortest_path(&graph, 0, 1).unwrap();
/// ```
fn shortest_path<T, U: Copy + Into<usize>>(
    _graph: &Graph<T, U>,
    _from: usize,
    _to: usize,
) -> Option<Vec<&T>> {
    unimplemented!()
}

/// Query the longest path from a `Graph`.
///
/// ```rust
/// use solve_graph::{graph, nodes, edges, longest_path};
///
/// let nodes = nodes(vec![0, 1, 2]);
/// let edges = edges(vec![Some(vec![edge(0, 1), edge(0, 2)]), Some(vec![edge(1, 2)]), None]);
/// let graph = graph![nodes, edges];
/// let path = longest_path(&graph, 0, 1).unwrap();
/// ```
fn longest_path<T, U: Copy + Into<usize>>(
    _graph: &Graph<T, U>,
    _from: usize,
    _to: usize,
) -> Option<Vec<&T>> {
    unimplemented!()
}

/// Query the neighbors of a `Node` from a `Graph`.
///
/// ```rust
/// use solve_graph::{graph, nodes, edges, neighbors};
///
/// let nodes = nodes(vec![0, 1, 2]);
/// let edges = edges(vec![Some(vec![edge(0, 1), edge(0, 2)]), Some(vec![edge(1, 2)]), None]);
/// let graph = graph![nodes, edges];
/// let neighbors = neighbors(&graph, 0).unwrap();
/// ```
fn neighbors<T, U: Copy + Into<usize>>(graph: &Graph<T, U>, index: usize) -> Option<Vec<&T>> {
    let edges = match graph.edges.get(index) {
        Some(it) => it,
        None => return None,
    };
    let mut neighbors = Vec::with_capacity(edges.len());
    for edge in edges {
        if let Some(node) = graph.nodes.get(edge.to.into()) {
            neighbors.push(node);
        }
    }
    Some(neighbors)
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

/// The `Nodes` struct composes `Node` data.
///
/// ```rust
/// use solver_graph::nodes;
///
/// let nodes = nodes(vec![]);
/// ```
fn nodes<T>(nodes: Vec<T>) -> Nodes<T> {
    Nodes(nodes)
}

/// The `Edges` struct composes `Edge` data.
///
/// ```rust
/// use solver_graph::edges;
///
/// let edges = edges(vec![]);
/// ```
fn edges<U: Copy + Into<usize>, T>(edges: Vec<Option<Vec<Edge<U, T>>>>) -> Edges<U, T> {
    Edges(edges)
}

/// The `Edge` struct composes the indexes of a 'from' and 'to' `Node`.
///
/// ```rust
/// use solver_graph::edge;
///
/// let edge = edge(0, 1);
/// ```
fn edge<U: Copy + Into<usize>, T>(from: U, to: U) -> Edge<U, T> {
    Edge {
        from,
        to,
        weights: None,
    }
}

/// The `Edge` struct composes the indexes of a 'from` and 'to' `Node`. Some `Edge`s can have
/// 'weight' data assigned to them.
///
/// ```rust
/// use solver_graph::weighted_edge;
///
/// let edge = weighted_edge(0, 1, vec![100]);
/// ```
fn weighted_edge<U: Copy + Into<usize>, T>(from: U, to: U, weights: Vec<T>) -> Edge<U, T> {
    Edge {
        from,
        to,
        weights: Some(weights),
    }
}

#[derive(Debug)]
struct NodeAttributes<T>(Vec<T>);

#[derive(Clone, Debug)]
struct Nodes<T>(Vec<T>);

/// Get the `NodeAttributes` of a `Graph`.
///
/// ```rust
/// use solve_graph::{Graph, node_attributes};
///
/// let mut graph = Graph::new();
/// let mut attrs = node_attributes!(&mut graph)
/// ```
fn node_attributes<T, U: Into<usize>>(graph: &mut Graph<T, U>) -> NodeAttributes<T> {
    NodeAttributes(Vec::with_capacity(graph.nodes.len()))
}

impl<T> Nodes<T> {
    /// Get an indexed `Node`.
    ///
    /// ```rust
    /// use solver_graph::nodes;
    ///
    /// let nodes = nodes(vec![0, 1, 2]);
    /// let first = nodes.get(0).unwrap()
    /// ```
    fn get(&self, index: usize) -> Option<&T> {
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
    fn first(&self) -> Option<&T> {
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
    fn last(&self) -> Option<&T> {
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
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Clone, Debug)]
struct Edges<U: Into<usize>, T>(Vec<Option<Vec<Edge<U, T>>>>);

#[derive(Clone, Debug)]
struct Edge<U: Into<usize>, T> {
    from: U,
    to: U,
    weights: Option<Vec<T>>,
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
    fn get(&self, index: usize) -> Option<&Vec<Edge<U, T>>> {
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
    fn first(&self) -> Option<&Vec<Edge<U, T>>> {
        get_edges(self, 0)
    }

    /// Get the last `Edge`.
    ///
    /// ```rust
    /// use solver_graph::{edges, edge};
    ///
    /// let edges = edges(vec![Some(vec![edge(0, 1), edge(0, 2)]), Some(vec![edge(1, 2)]), None]);
    /// let last = edges.last().unwrap()
    fn last(&self) -> Option<&Vec<Edge<U, T>>> {
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
    fn len(&self) -> usize {
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
    use super::*;

    #[test]
    fn test_graph() {
        let (nodes, edges) = (sample_nodes(), sample_edges());
        let graph = graph![nodes.clone(), edges.clone()];
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
        let graph = graph![nodes.clone(), edges.clone()];
        assert_eq!(nodes.last(), graph.nodes().last());
        assert_eq!(edges.last(), graph.edges().last());
    }

    #[test]
    fn test_neighbors() {
        let (nodes, edges) = (sample_nodes(), sample_edges());
        let graph = graph![nodes.clone(), edges.clone()];
        let neighbors = neighbors(&graph, 0).unwrap();
        let ans = edges
            .get(0)
            .unwrap()
            .iter()
            .map(|e| nodes.get(e.to).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(ans, neighbors);
    }

    // #[test]
    // fn test_shortest_path() {
    //     let (nodes, edges) = (sample_nodes(), sample_weighted_edges());
    //     let graph = graph![nodes.clone(), edges.clone()];
    //     let path = shortest_path(&graph, 0, 2).unwrap();
    //     assert_eq!(path, vec![&0, &1, &2]);
    // }

    // #[test]
    // fn test_longest_path() {
    //     let (nodes, edges) = (sample_nodes(), sample_edges());
    //     let graph = graph![nodes.clone(), edges.clone()];
    //     let path = longest_path(&graph, 0, 1).unwrap();
    //     assert_eq!(path, vec![&0, &2]);
    // }

    fn sample_nodes() -> Nodes<i32> {
        nodes(vec![0, 1, 2, 3])
    }

    fn sample_edges() -> Edges<usize, i32> {
        edges(vec![
            Some(vec![edge(0, 1), edge(0, 2)]),
            Some(vec![edge(1, 2)]),
            Some(vec![edge(2, 0)]),
            None,
        ])
    }

    fn sample_weighted_edges() -> Edges<usize, i32> {
        edges(vec![
            Some(vec![
                weighted_edge(0, 1, vec![1]),
                weighted_edge(0, 2, vec![100]),
            ]),
            Some(vec![weighted_edge(1, 2, vec![1])]),
            Some(vec![weighted_edge(2, 0, vec![2])]),
            None,
        ])
    }
}
