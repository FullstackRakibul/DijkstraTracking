use std::collections::BinaryHeap;
use crate::graph::{Edge, Graph, State, reconstruct_path};

/// Dijkstra's Shortest Path Algorithm
/// Time Complexity: O((V + E) log V)
/// Space Complexity: O(V)
/// 
/// # Arguments
/// * `graph` - Adjacency list representation of the graph
/// * `start` - Starting vertex index
/// 
/// # Returns
/// A tuple containing:
/// * `distances` - Vector of shortest distances from start to each vertex
/// * `parent` - Vector for path reconstruction
pub fn dijkstra(graph: &Graph, start: usize) -> (Vec<usize>, Vec<Option<usize>>) {
    let n = graph.len();

    // Distance array initialized to infinity
    let mut dist = vec![usize::MAX; n];

    // Parent tracking for path reconstruction
    let mut parent = vec![None; n];

    // Priority queue (min-heap)
    let mut heap = BinaryHeap::new();

    // Start node has distance 0
    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Process nodes in order of increasing distance
    while let Some(State { cost, position }) = heap.pop() {
        // Skip outdated states
        if cost > dist[position] {
            continue;
        }

        // Explore neighbors and perform relaxation
        for edge in &graph[position] {
            let next_cost = cost + edge.cost;
            let next_node = edge.node;

            // If we found a shorter path, update it
            if next_cost < dist[next_node] {
                dist[next_node] = next_cost;
                parent[next_node] = Some(position);
                heap.push(State {
                    cost: next_cost,
                    position: next_node,
                });
            }
        }
    }

    (dist, parent)
}

/// Get shortest path between two nodes using Dijkstra
pub fn get_shortest_path(graph: &Graph, start: usize, target: usize) -> (Vec<usize>, usize) {
    let (distances, parent) = dijkstra(graph, start);
    let path = reconstruct_path(&parent, target);
    let distance = distances[target];
    (path, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra_basic() {
        let mut graph: Graph = vec![vec![]; 6];
        graph[0].push(Edge { node: 1, cost: 4 });
        graph[0].push(Edge { node: 2, cost: 1 });
        graph[2].push(Edge { node: 1, cost: 2 });
        graph[2].push(Edge { node: 3, cost: 5 });
        graph[1].push(Edge { node: 3, cost: 1 });
        graph[3].push(Edge { node: 4, cost: 3 });
        graph[4].push(Edge { node: 5, cost: 1 });

        let (dist, _) = dijkstra(&graph, 0);
        
        assert_eq!(dist[0], 0);
        assert_eq!(dist[5], 12);
    }
}
