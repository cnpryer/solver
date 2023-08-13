use crate::graph::{Edge, Edges, Graph, Nodes};

/// Use `graph![nodes, edges]` to create a `Graph`.
///
/// ```rust
/// use solve_graph::{graph, nodes, edges};
///
/// let nodes = nodes(vec![0, 1, 2]);
/// let edges = edges(vec![Some(vec![edge(0, 1), edge(0, 2)]), Some(vec![edge(1, 2)]), None]);
/// let graph = graph![nodes, edges];
/// ```
#[macro_export]
macro_rules! graph {
    ($nodes:expr, $edges:expr) => {{
        $crate::graph::graph($nodes, $edges)
    }};
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
pub(crate) fn neighbors<T, U: Copy + Into<usize>>(
    graph: &Graph<T, U>,
    index: usize,
) -> Option<Vec<&U>>
where
    T: Copy,
    T: Default,
{
    graph.edges().get(index).map(|edges| edges.iter().map(|e| &e.to).collect())
}

/// The `Nodes` struct composes `Node` data.
///
/// ```rust
/// use solver_graph::nodes;
///
/// let nodes = nodes(vec![]);
/// ```
pub(crate) fn nodes<T>(nodes: Vec<T>) -> Nodes<T> {
    Nodes(nodes)
}

/// The `Edges` struct composes `Edge` data.
///
/// ```rust
/// use solver_graph::edges;
///
/// let edges = edges(vec![]);
/// ```
pub(crate) fn edges<U: Copy + Into<usize>, T>(edges: Vec<Option<Vec<Edge<U, T>>>>) -> Edges<U, T> {
    Edges(edges)
}

/// The `Edge` struct composes the indexes of a 'from' and 'to' `Node`.
///
/// ```rust
/// use solver_graph::edge;
///
/// let edge = edge(0, 1);
/// ```
pub(crate) fn edge<U: Copy + Into<usize>, T>(from: U, to: U) -> Edge<U, T> {
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
pub(crate) fn weighted_edge<U: Copy + Into<usize>, T>(
    from: U,
    to: U,
    weights: Vec<T>,
) -> Edge<U, T> {
    Edge {
        from,
        to,
        weights: Some(weights),
    }
}

#[cfg(test)]
mod tests {
    use crate::test_fixtures::{sample_edges, sample_nodes};

    use super::*;

    #[test]
    fn test_neighbors() {
        let (nodes, edges) = (sample_nodes(), sample_edges());
        let graph = graph![nodes, edges.clone()];
        let neighbors = neighbors(&graph, 0).unwrap();
        let ans = edges
            .get(0)
            .unwrap()
            .iter()
            .map(|e| &e.to)
            .collect::<Vec<_>>();
        assert_eq!(ans, neighbors);
    }
}
