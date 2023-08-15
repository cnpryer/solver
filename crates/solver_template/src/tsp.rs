use solver_graph::{
    graph,
    small_graph::{edges, nodes, shortest_path, weighted_edge, Edges},
};

pub fn resolve() {
    println!("running small tsp...");
    let nodes = nodes(vec![0, 1, 2, 3]);
    // TODO(cnpryer): Use Indexable or some trait to implement hashing for From<Number>
    let edges: Edges<usize, i32> = edges(vec![
        vec![weighted_edge(0, 1, vec![1]), weighted_edge(0, 2, vec![100])],
        vec![weighted_edge(1, 2, vec![1])],
        vec![weighted_edge(2, 0, vec![2])],
        vec![],
    ]);
    let graph = graph![nodes, edges];
    println!("graph: {:?}", &graph);
    let shortest_path = shortest_path(&graph, 0, 2);
    println!("solution: {:?}", shortest_path);
}
