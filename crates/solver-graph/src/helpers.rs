use crate::{
    small_array::SmallArray,
    small_graph::{Edge, Edges, Nodes, SmallGraph},
    Position, Value,
};

/// Query the neighbors of a `Node` from a `SmallGraph`.
///
/// ```rust
/// use solve_graph::small_graph::{graph, nodes, edges, neighbors};
///
/// let nodes = nodes(vec![0, 1, 2]);
/// let edges = edges(vec![Some(vec![edge(0, 1), edge(0, 2)]), Some(vec![edge(1, 2)]), None]);
/// let graph = graph![nodes, edges];
/// let neighbors = neighbors(&graph, 0).unwrap();
/// ```
pub(crate) fn neighbors<V: Value, P: Position>(
    graph: &SmallGraph<V, P>,
    index: P,
) -> Option<Vec<&P>> {
    graph
        .edges()
        .get(index)
        .map(|edges| edges.iter().map(|e| &e.to).collect())
}

/// The `Nodes` struct composes `Node` data.
///
/// ```rust
/// use solver_graph::small_graph::nodes;
///
/// let nodes = nodes(vec![]);
/// ```
pub fn nodes<V>(nodes: Vec<V>) -> Nodes<V> {
    Nodes(nodes)
}

/// The `Edges` struct composes `Edge` data.
///
/// ```rust
/// use solver_graph::small_graph::edges;
///
/// let edges = edges(vec![]);
/// ```
pub fn edges<P: Position, V: Value>(edges: Vec<Vec<Edge<P, V>>>) -> Edges<P, V> {
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
/// use solver_graph::small_graph::edge;
///
/// let edge = edge(0, 1);
/// ```
pub fn edge<P: Position, V: Value>(from: P, to: P) -> Edge<P, V> {
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
/// use solver_graph::small_graph::weighted_edge;
///
/// let edge = weighted_edge(0, 1, vec![100]);
/// ```
pub fn weighted_edge<P: Position, V: Value>(from: P, to: P, weights: Vec<V>) -> Edge<P, V> {
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
#[macro_use]
mod tests {
    use crate::{
        graph,
        small_graph::test_fixtures::{sample_edges, sample_nodes},
    };

    use super::*;

    #[test]
    fn test_neighbors() {
        let (nodes, edges) = (sample_nodes(), sample_edges());
        let graph: _ = graph![nodes, edges.clone()];
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
