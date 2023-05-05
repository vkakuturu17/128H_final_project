use final_project::djikstra;
use final_project::Graph;
use final_project::Node;
use std::collections::HashMap;

fn main() {

    let mut edges = HashMap::new();
    edges.insert(1, vec![Node::new(2, 7), Node::new(3, 9), Node::new(6, 14)]);
    edges.insert(2, vec![Node::new(1, 7), Node::new(3, 10), Node::new(4, 15)]);
    edges.insert(3, vec![Node::new(1, 9), Node::new(2, 10), Node::new(4, 11), Node::new(6, 2)]);
    edges.insert(4, vec![Node::new(2, 15), Node::new(3, 11), Node::new(5, 6)]);
    edges.insert(5, vec![Node::new(4, 6), Node::new(6, 9)]);
    edges.insert(6, vec![Node::new(1, 14), Node::new(3, 2), Node::new(5, 9)]);

    let graph = Graph::new(vec![Node::new(1, 0), Node::new(2, 999), Node::new(3, 999), Node::new(4, 999), Node::new(5, 999), Node::new(6, 999)], edges);
    let start = Node::new(1, 0);
    let distances = djikstra(graph, start);

    assert_eq!(distances[&1], 0);
    assert_eq!(distances[&2], 7);
    assert_eq!(distances[&3], 9);
    assert_eq!(distances[&4], 20);
    assert_eq!(distances[&5], 20);
    assert_eq!(distances[&6], 11);

}
