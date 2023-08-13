#[derive(Debug)]
struct Index(usize);

#[derive(Debug)]
struct Nodes<T>(Vec<T>);

#[derive(Debug)]
struct Edges(Vec<Vec<usize>>);

#[derive(Debug)]
struct Weights<T>(Vec<T>);

#[derive(Debug)]
struct Graph<T> {
    nodes: Nodes<T>,
    edges: Edges,
}

impl<T: Copy + Default> Graph<T> {
    fn new() -> Self {
        Self {
            nodes: Nodes(Vec::new()),
            edges: Edges(Vec::new()),
        }
    }

    fn nodes(&self) -> &Nodes<T> {
        &self.nodes
    }

    fn edges(&self) -> &Edges {
        &self.edges
    }

    fn neighbors(&self, index: usize) -> Option<Vec<&T>> {
        let to = match self.edges.get(index) {
            Some(it) => it,
            None => return None,
        };
        let mut neighbors = Vec::new();
        for idx in to {
            if let Some(node) = self.nodes.get(*idx) {
                neighbors.push(node);
            }
        }
        Some(neighbors)
    }
}

impl<T: Copy + Default> Default for Graph<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Nodes<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter)
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }

    fn first(&self) -> Option<&T> {
        self.0.first()
    }

    fn last(&self) -> Option<&T> {
        self.0.last()
    }
}

impl Edges {
    fn get(&self, index: usize) -> Option<&Vec<usize>> {
        self.0.get(index)
    }

    fn first(&self) -> Option<&Vec<usize>> {
        self.0.first()
    }

    fn last(&self) -> Option<&Vec<usize>> {
        self.0.last()
    }
}

macro_rules! graph {
    ($nodes:expr, $edges:expr) => {{
        assert_eq!($nodes.len(), $edges.len());

        Graph {
            nodes: Nodes($nodes),
            edges: Edges($edges),
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph() {
        let (nodes, edges) = (sample_nodes(), sample_edges());
        let graph = graph![nodes.clone(), edges.clone()];
        assert_eq!(nodes.last(), graph.nodes().last());
        assert_eq!(edges.last(), graph.edges.last());
    }

    #[test]
    fn test_graph_neighbors() {
        let (nodes, edges) = (sample_nodes(), sample_edges());
        let graph = graph![nodes.clone(), edges.clone()];
        let neighbors = graph.neighbors(0).unwrap();
        let ans = edges
            .get(0)
            .unwrap()
            .into_iter()
            .filter_map(|i| nodes.get(*i))
            .collect::<Vec<_>>();
        assert_eq!(ans, neighbors);
    }

    fn sample_nodes() -> Vec<i32> {
        vec![0, 1, 2, 3]
    }

    fn sample_edges() -> Vec<Vec<usize>> {
        vec![vec![1, 2], vec![2], vec![0], vec![]]
    }
}
