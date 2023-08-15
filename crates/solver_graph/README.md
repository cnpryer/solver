# solver-graph

Compact data structures for fast graph-based solutions.

## `SmallGraph`

Use `graph!` to build a `SmallGraph` from `Nodes` and `Edges`.

```rust
use solver_graph::small_graph::{graph, nodes, edges, shortest_path};

let nodes = nodes(vec![0, 1, 2]);
let edges = edges(
    vec![
        vec![weighted_edge(0, 1, vec![1]), weighted_edge(0, 2, vec![10])],
        vec![weighted_edge(1, 2, vec![1])],
        vec![],
    ]
)
let graph = graph![nodes, edges];
```

Run `shortest_path` on your graph.

```rust
assert_eq!(shortest_path(&graph, 0, 2).unwrap(), vec![&0, &1, &2]);
```

`SmallGraph` is designed for compact graph-based problems where `V` (N vertices) and `E` (N edges) are or can be limited.

<img src="https://github.com/cnpryer/solver/blob/master/crates/solver_graph/img/small_graph.png" alt="image of a small graph" width="1000" text-align = "center" />
