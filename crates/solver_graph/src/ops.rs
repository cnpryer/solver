use crate::graph::Graph;

/// Sort the `Nodes` of the `Graph`.
///
/// ```rust
/// use solve_graph::{Graph, sort};
///
/// let mut graph = Graph::new();
/// let mut graph = sort(&mut graph);
/// ```
pub(crate) fn sort<T, U: Into<usize>>(_graph: &mut Graph<T, U>) -> &mut Graph<T, U> {
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
pub(crate) fn shortest_path<T, U: Copy + Into<usize>>(
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
pub(crate) fn longest_path<T, U: Copy + Into<usize>>(
    _graph: &Graph<T, U>,
    _from: usize,
    _to: usize,
) -> Option<Vec<&T>> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    // use crate::{test_fixtures::{sample_nodes, sample_edges}, helpers::neighbors, graph};

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
}
