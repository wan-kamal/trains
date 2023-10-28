### Limitation and Assumption
The solution is not optimized, therefore not suitable for large inputs. No conccurency currently supported.
A train can only deliver one package at a time.
There must be at least one train with enough capacity.
There must always be a path to the package.
There must always be a path to the package destination.
There is no negative weights.

Example:

> use `cargo run` at root to run the program

```
W=0, T=Q1, N1=B, P1=[], N2=A, P2=[]
W=30, T=Q1, N1=A, P1=[K1], N2=B, P2=[]
W=60, T=Q1, N1=B, P1=[], N2=C, P2=[K1]
W=0, T=Q2, N1=C, P1=[K2], N2=B, P2=[]
W=10, T=Q2, N1=B, P1=[], N2=A, P2=[K2]
```

### Usage

```rs
// Add input to graph
let mut graph = Graph::new();

let nodes = vec![
    Node {
        name: "A"
    },
    Node {
        name: "B"
    },
    Node {
        name: "C"
    },
];

let edges = vec![
    Edge {
        name: "E1",
        weight: 30,
        from: "A",
        to: "B",
    },
    Edge {
        name: "E2",
        weight: 10,
        from: "B",
        to: "C",
    },
];

let trains = vec![
    Train {
        name: "Q1",
        capacity: 6,
        start: "B",
    },
    Train {
        name: "Q2",
        capacity: 6,
        start: "C",
    },
];

let packages = vec![
    Package {
        name: "K1",
        weight: 5,
        start: "A",
        dest: "C",
    },
    Package {
        name: "K2",
        weight: 5,
        start: "C",
        dest: "A",
    },
];

graph.input(nodes, edges, trains, packages);

// Get resulting moves for all packages and print
let result = graph.start();
for moves in result {
    for m in moves {
        println!("W={}, T={}, N1={}, P1=[{}], N2={}, P2=[{}]", m.w, m.t, m.n1, m.p1, m.n2, m.p2);
    }
}
```

### Test

```bash
cargo test
```
