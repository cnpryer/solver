# solver-graph

`Graph` can be used for operations on `Nodes` and `Edges`. Constructing a `Graph` requires using the `graph!` macro to index `nodes` and `edges`. Use the `shortest_path` operation to find the shortest path between any two `Node`s.

```rust
use solver_graph::{graph, nodes, edges, shortest_path};

let nodes = nodes(vec![0, 0, 0]);
let edges = edges(
    vec![
        Some(vec![weighted_edge(0, 1, vec![1]), weighted_edge(0, 2, vec![10])]),
        Some(vec![weighted_edge(1, 2, vec![1])]),
        None,
    ]
)
let graph = graph![nodes, edges];
let path = shortest_path(&graph, 0, 2).unwrap()
assert_eq!(path, vec![&0, &1, &2]);
```