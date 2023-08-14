use crate::{
    graph::{Edge, Edges, Graph, Nodes},
    small_array::SmallArray,
    Position, Value,
};

#[macro_export]
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
pub(crate) fn neighbors<V: Value, P: Position>(graph: &Graph<V, P>, index: P) -> Option<Vec<&P>> {
    graph
        .edges()
        .get(index)
        .map(|edges| edges.iter().map(|e| &e.to).collect())
}

/// The `Nodes` struct composes `Node` data.
///
/// ```rust
/// use solver_graph::nodes;
///
/// let nodes = nodes(vec![]);
/// ```
pub(crate) fn nodes<V>(nodes: Vec<V>) -> Nodes<V> {
    Nodes(nodes)
}

/// The `Edges` struct composes `Edge` data.
///
/// ```rust
/// use solver_graph::edges;
///
/// let edges = edges(vec![]);
/// ```
pub(crate) fn edges<P: Position, V: Value>(edges: Vec<Vec<Edge<P, V>>>) -> Edges<P, V> {
    // TODO
    Edges(
        edges
            .into_iter()
            .map(|e| {
                if e.is_empty() {
                    SmallArray::Empty
                } else {
                    SmallArray::Dynamic(e)
                }
            })
            .collect(),
    )
}

/// The `Edge` struct composes the indexes of a 'from' and 'to' `Node`.
///
/// ```rust
/// use solver_graph::edge;
///
/// let edge = edge(0, 1);
/// ```
pub(crate) fn edge<P: Position, V: Value>(from: P, to: P) -> Edge<P, V> {
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
pub(crate) fn weighted_edge<P: Position, V: Value>(from: P, to: P, weights: Vec<V>) -> Edge<P, V> {
    // TODO(cnpryer): https://github.com/cnpryer/solver/issues/50
    assert!(weights.is_empty() || weights.len() == 1);
    Edge {
        from,
        to,
        weights: Some(if weights.is_empty() {
            SmallArray::Empty
        } else {
            SmallArray::One([weights[0]])
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::test_fixtures::{sample_edges, sample_nodes};

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
