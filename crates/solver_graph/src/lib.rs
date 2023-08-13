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

/// Use `graph![nodes, edges]` to create a `Graph`.
///
/// ```rust
/// use solve_graph::{graph, nodes, edges, find_neighbors};
///
/// let nodes = nodes(vec![0, 1, 2]);
/// let edges = edges(vec![Some(vec![edge(0, 1), edge(0, 2)]), Some(vec![edge(1, 2)]), None]);
/// let graph = graph![nodes, edges];
/// let neighbors = find_neighbors(&graph, 0).unwrap();
/// ```
fn find_neighbors<T, U: Copy + Into<usize>>(graph: &Graph<T, U>, index: usize) -> Option<Vec<&T>> {
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

#[derive(Clone, Debug)]
struct Nodes<T>(Vec<T>);

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
        get_edges(self, self.0.len() - 1)
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

fn nodes<T>(nodes: Vec<T>) -> Nodes<T> {
    Nodes(nodes)
}

fn edges<U: Copy + Into<usize>, T>(edges: Vec<Option<Vec<Edge<U, T>>>>) -> Edges<U, T> {
    Edges(edges)
}

fn edge<U: Copy + Into<usize>, T>(from: U, to: U) -> Edge<U, T> {
    Edge {
        from,
        to,
        weights: None,
    }
}

fn weighted_edge<U: Copy + Into<usize>, T>(from: U, to: U, weights: Vec<T>) -> Edge<U, T> {
    Edge {
        from,
        to,
        weights: Some(weights),
    }
}

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
    fn test_graph_neighbors() {
        let (nodes, edges) = (sample_nodes(), sample_edges());
        let graph = graph![nodes.clone(), edges.clone()];
        let neighbors = find_neighbors(&graph, 0).unwrap();
        let ans = edges
            .get(0)
            .unwrap()
            .iter()
            .map(|e| nodes.get(e.to).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(ans, neighbors);
    }

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
}
