use std::cmp::Ordering;

/// Edge representation in a graph
#[derive(Clone, Debug)]
pub struct Edge {
    pub node: usize,
    pub cost: usize,
}

/// State for priority queue in Dijkstra's algorithm
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State {
    pub cost: usize,
    pub position: usize,
}

/// Min-Heap implementation for State
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Graph representation as adjacency list
pub type Graph = Vec<Vec<Edge>>;

/// Reconstruct shortest path from parent array
pub fn reconstruct_path(parent: &[Option<usize>], target: usize) -> Vec<usize> {
    let mut path = vec![];
    let mut current = Some(target);

    while let Some(node) = current {
        path.push(node);
        current = parent[node];
    }

    path.reverse();
    path
}
