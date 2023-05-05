use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Node {
    id: i32,
    cost: i32,
}

impl Node {
    pub fn new(id: i32, cost: i32) -> Node {
        Node {
            id,
            cost,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

pub struct Graph {
    nodes: Vec<Node>,
    edges: HashMap<i32, Vec<Node>>,
}

impl Graph {
    pub fn new(nodes: Vec<Node>, edges: HashMap<i32, Vec<Node>>) -> Graph {
        Graph {
            nodes,
            edges,
        }
    }
}

pub fn djikstra(graph: Graph, start: Node) -> HashMap<i32, i32> {
    let mut visited = HashSet::new();
    let mut distances = HashMap::new();
    let mut heap = BinaryHeap::new();

    heap.push(start);

    while let Some(node) = heap.pop() {
        if visited.contains(&node.id) {
            continue;
        }

        visited.insert(node.id);
        distances.insert(node.id, node.cost);

        if let Some(neighbors) = graph.edges.get(&node.id) {
            for neighbor in neighbors {
                let cost = neighbor.cost + node.cost;
                heap.push(Node::new(neighbor.id, cost));
            }
        }
    }

    distances
}