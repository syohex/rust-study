use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

type Node = usize;
type Cost = usize;

struct Graph {
    edges: HashMap<Node, Vec<(Node, Cost)>>,
    nodes: HashSet<Node>,
}

impl Graph {
    fn from_edge_list(edge_list: &Vec<(Node, Node, Cost)>) -> Self {
        let mut adjacency_list: HashMap<Node, Vec<(Node, Cost)>> = HashMap::new();
        let mut nodes = HashSet::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = adjacency_list.entry(source).or_insert_with(|| Vec::new());

            destinations.push((destination, cost));

            nodes.insert(source);
            nodes.insert(destination);
        }

        Graph {
            edges: adjacency_list,
            nodes,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Data {
    node: Node,
    cost: Cost,
    path: Vec<Node>,
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        // self.cost.cmp(&other.cost)
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cost.cmp(&other.cost))
    }
}

fn shortest_path(g: &Graph, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {
    let mut q = BinaryHeap::new();
    q.push(Data {
        node: start,
        cost: 0,
        path: vec![start],
    });

    let mut costs: HashMap<Node, (Cost, Vec<Node>)> =
        g.nodes.iter().fold(HashMap::new(), |mut acc, n| {
            acc.insert(*n, (std::usize::MAX, vec![]));
            acc
        });

    while !q.is_empty() {
        let d = q.pop().unwrap();

        if let Some(nexts) = g.edges.get(&d.node) {
            for (next, cost) in nexts {
                if let Some((min_cost, min_path)) = costs.get_mut(&next) {
                    let total = d.cost + cost;
                    if total < *min_cost {
                        *min_cost = total;

                        let mut path = d.path.clone();
                        path.push(*next);
                        *min_path = path.clone();

                        q.push(Data {
                            node: *next,
                            cost: total,
                            path: path,
                        });
                    }
                }
            }
        }
    }

    if let Some((cost, path)) = costs.get(&goal) {
        Some((path.clone(), *cost))
    } else {
        None
    }
}

fn main() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    if let Some((path, cost)) = shortest_path(&g, 1000, 9000) {
        println!("1000->9000, {:?} {}", path, cost);
    };
}

#[test]
fn large_graph() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    let path = shortest_path(&g, 1000, 9000);
    assert!(path.is_some());
    assert_eq!(path.unwrap().1, 24);
}
