use std::collections::LinkedList;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Node {
    name: &'static str,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Edge {
    name: &'static str,
    weight: u32,
    from: &'static str,
    to: &'static str,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Train {
    name: &'static str,
    capacity: u32,
    start: &'static str,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Package {
    name: &'static str,
    weight: u32,
    start: &'static str,
    dest: &'static str,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Move {
    w: u32,
    t: &'static str,
    n1: &'static str,
    p1: &'static str,
    n2: &'static str,
    p2: &'static str,
}

#[derive(Debug, PartialEq, Eq)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    trains: Vec<Train>,
    packages: Vec<Package>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            trains: Vec::new(),
            packages: Vec::new(),
        }
    }

    fn input(
        &mut self,
        nodes: Vec<Node>,
        edges: Vec<Edge>,
        trains: Vec<Train>,
        packages: Vec<Package>
    ) {
        self.nodes = nodes;
        self.edges = edges;
        self.trains = trains;
        self.packages = packages;
    }

    // Assuming everyone is happy;
    // there must be at least one train with enough capacity
    // there is always a path to the package
    // there is always a path to package destination
    // there is no negative weights
    fn start(&self) -> Vec<Vec<Move>> {
        let mut result: Vec<Vec<Move>> = vec![];
        for package in self.packages.clone().into_iter() {
            let mut train_choice = None;
            let mut shortest_path: Vec<Node> = Vec::new();
            let mut shortest_duration: Vec<u32> = vec![u32::MAX];

            // this does not scale, but problem for future me
            for train in self.trains.clone().into_iter() {
                if train.capacity < package.weight {
                    continue;
                }

                match self.shortest_path(train.start, package.start) {
                    Ok((path, duration)) => {
                        if duration.iter().fold(0, |acc, x| acc + x) < shortest_duration.iter().fold(0, |acc, x| acc + x) {
                            train_choice = Some(train);
                            shortest_duration = duration;
                            shortest_path = path;
                        }
                    },
                    Err(err) => panic!("{} :(", err),
                }
            }

            match self.shortest_path(package.start, package.dest) {
                Ok((mut path, mut duration)) => {
                    shortest_path.append(&mut path);
                    shortest_duration.append(&mut duration);
                },
                Err(err) => panic!("{} :(", err),
            }

            let moves = self.get_moves(train_choice.unwrap(), shortest_path, package, shortest_duration);
            result.push(moves);
        }
        result
    }

    fn get_moves(&self, train: Train, path: Vec<Node>, package: Package, duration: Vec<u32>) -> Vec<Move> {
        let mut w = 0;
        let t = train.name;
        let mut n1 = train.start;
        let mut p1 = "";
        let mut n2 = path[0].name;
        let mut p2 = "";

        // edge case
        if n1 == package.start {
            p1 = package.name;
        }

        // source
        let mut moves: Vec<Move> = vec![Move {
            w,
            t,
            n1,
            p1,
            n2,
            p2,
        }];

        // travelling
        for i in 0..path.len() - 1 {
            w += duration[i];

            n1 = path[i].name;
            match n1 == package.start {
                true => p1 = package.name,
                false => p1 = "",
            }

            n2 = path[i + 1].name;
            match n2 == package.dest {
                true => p2 = package.name,
                false => p2 = "",
            }
            moves.push(Move {
                w,
                t,
                n1,
                p1,
                n2,
                p2,
            });
        }

        moves
    }

    fn get_names(&self) -> Vec<&str> {
        self.nodes.iter().map(|m| m.name).collect::<Vec<&str>>()
    }

    // tried with adj list, but didn't work as expected
    fn create_adj_matrix(&self) -> Vec<Vec<u32>> {
        let edges = self.edges.clone();
        let names = self.get_names();
        let mut adj: Vec<Vec<u32>> = vec![vec![0u32; names.len()]; names.len()];
        for edge in edges.into_iter() {
            let row = self.get_names()
                .into_iter()
                .position(|p| p == edge.from)
                .clone()
                .unwrap();
            let col = self.get_names()
                .into_iter()
                .position(|p| p == edge.to)
                .clone()
                .unwrap();
            adj[row][col] = edge.weight;
            adj[col][row] = edge.weight;
        }
        adj
    }

    fn min_dist(dist: Vec<u32>, seen: Vec<bool>) -> usize {
        let mut min = u32::MAX;
        let mut index: usize = 0;
        for (i, k) in dist.into_iter().enumerate() {
            if !seen[i] && k <= min {
                min = k;
                index = i;
            }
        }
        index
    }

    fn dijkstra(graph:Vec<Vec<u32>>, src: usize, dest: usize) -> Option<Vec<u32>> {
        let length = graph.len();

        // min heap can improve this, but problem for future me
        let mut dist = vec![u32::MAX; length];
        let mut seen = vec![false; length];
        let mut prev = vec![None; length];

        dist[src] = 0;

        for _ in 0..length - 1 {
            let min_graph = Graph::min_dist(dist.clone(), seen.clone());
            seen[min_graph] = true;

            for i in 0..length {
                if graph[min_graph][i] > 0 {
                    let shortest_to_min_graph = dist[min_graph];
                    let dist_to_next_graph = graph[min_graph][i];
                    let total_dist = shortest_to_min_graph + dist_to_next_graph;
                    if total_dist < dist[i] {
                        dist[i] = total_dist;
                        prev[i] = Some(min_graph);
                    }
                }
            }
        }

        if dist[dest] == u32::MAX {
            return None;
        }

        let mut path: LinkedList<u32> = LinkedList::new();
        let mut current_graph: Option<usize> = Some(dest);

        while let Some(p) = current_graph {
            path.push_front(p as u32);
            current_graph = prev[p];
        }

        Some(path.into_iter().collect::<Vec<u32>>())
    }

    fn shortest_path(&self, src: &str, dest: &str) -> Result<(Vec<Node>, Vec<u32>), &'static str> {
        let graph = self.create_adj_matrix();

        let source = self.get_names()
            .into_iter()
            .position(|p| p == src)
            .clone()
            .unwrap(); // usize

        let destination = self.get_names()
            .into_iter()
            .position(|p| p == dest)
            .clone()
            .unwrap(); // usize

        let paths = Graph::dijkstra(graph.clone(), source, destination);

        if let Some(p) = paths {
            let mut duration: Vec<u32> = Vec::new();
            let mut nodes: Vec<Node> = Vec::new();

            for i in p.clone().into_iter() {
                if let Some(node) = self.nodes
                    .clone()
                    .into_iter()
                    .enumerate()
                    .find_map(|(n, k)|
                        match n == i as usize && n != source {
                            true => Some(k),
                            false => None,
                        }
                    ) {
                    nodes.push(node);
                }
            }
            // minus the source
            for i in 0..p.len() - 1 {
                let weight: u32 = graph[p[i] as usize][p[i + 1] as usize];
                duration.push(weight);
            }

            return Ok((nodes, duration));
        } else {
            return Err("No Path");
        }
    }
}

fn main() {
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

    let result = graph.start();
    for moves in result {
        for m in moves {
            println!("W={}, T={}, N1={}, P1=[{}], N2={}, P2=[{}]", m.w, m.t, m.n1, m.p1, m.n2, m.p2);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn setup(graph: &mut Graph) -> &Graph {
        let nodes = vec![
            Node {
                name: "A"
            },
            Node {
                name: "B"
            },
        ];

        let edges = vec![
            Edge {
                name: "E1",
                weight: 30,
                from: "A",
                to: "B",
            },
        ];

        let trains = vec![
            Train {
                name: "Q1",
                capacity: 2,
                start: "B",
            },
        ];

        let packages = vec![
            Package {
                name: "K1",
                weight: 1,
                start: "A",
                dest: "B",
            },
        ];

        graph.input(nodes, edges, trains, packages);

        graph
    }

    #[test]
    fn test_shortest_path() {
        let mut graph = Graph::new();
        setup(&mut graph);

        let train_test = graph.trains[0].clone();
        let package_test = graph.packages[0].clone();

        let test = graph.shortest_path(train_test.start, package_test.start);
        let test2 = graph.shortest_path(package_test.start, package_test.dest);

        match test {
            Ok((a, b)) => {
                assert_eq!(a[0], Node { name: "A"});
                assert_eq!(b[0], 30);
            },
            Err(err) => panic!("test failed -> {}", err),
        }

        match test2 {
            Ok((a, b)) => {
                assert_eq!(a[0], Node { name: "B"});
                assert_eq!(b[0], 30);
            },
            Err(err) => panic!("test failed -> {}", err),
        }
    }

    // technically should be an e2e test
    #[test]
    fn test_e2e() {
        let mut graph = Graph::new();
        setup(&mut graph);

        let train_test = graph.trains[0].clone();
        let package_test = graph.packages[0].clone();

        let test = graph.start();
        for moves in test {
            assert_eq!(moves, vec![
                Move {
                    w: 0,
                    t: "Q1",
                    n1: "B",
                    p1: "",
                    n2: "A",
                    p2: "",
                },
                Move {
                    w: 30,
                    t: "Q1",
                    n1: "A",
                    p1: "K1",
                    n2: "B",
                    p2: "K1",
                },
            ]);
        }
    }
}
