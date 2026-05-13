use crate::graph::{Graph, Edge, reconstruct_path};

/// Bellman-Ford Shortest Path Algorithm
/// Time Complexity: O(V * E)
/// Space Complexity: O(V)
/// 
/// Key Features:
/// - Can handle negative edge weights
/// - Can detect negative cycles
/// - Slower than Dijkstra but more versatile
/// 
/// # Arguments
/// * `graph` - Adjacency list representation of the graph
/// * `start` - Starting vertex index
/// 
/// # Returns
/// A tuple containing:
/// * `Result::Ok((distances, parent))` - Successful computation
/// * `Result::Err(String)` - Negative cycle detected
pub fn bellman_ford(graph: &Graph, start: usize) -> Result<(Vec<i32>, Vec<Option<usize>>), String> {
    let n = graph.len();

    // Distance array initialized to infinity (using i32 for negative weights)
    let mut dist = vec![i32::MAX; n];

    // Parent tracking for path reconstruction
    let mut parent = vec![None; n];

    // Start node has distance 0
    dist[start] = 0;

    // Relax edges V-1 times
    for _ in 0..n - 1 {
        for u in 0..n {
            if dist[u] == i32::MAX {
                continue;
            }

            for edge in &graph[u] {
                let v = edge.node;
                let weight = edge.cost as i32;

                // Relaxation step
                if dist[u] + weight < dist[v] {
                    dist[v] = dist[u] + weight;
                    parent[v] = Some(u);
                }
            }
        }
    }

    // Check for negative cycles
    for u in 0..n {
        if dist[u] == i32::MAX {
            continue;
        }

        for edge in &graph[u] {
            let v = edge.node;
            let weight = edge.cost as i32;

            if dist[u] + weight < dist[v] {
                return Err("Negative cycle detected!".to_string());
            }
        }
    }

    Ok((dist, parent))
}

/// Get shortest path between two nodes using Bellman-Ford
pub fn get_shortest_path(graph: &Graph, start: usize, target: usize) -> Result<(Vec<usize>, i32), String> {
    let (distances, parent) = bellman_ford(graph, start)?;
    let path = reconstruct_path(&parent, target);
    let distance = distances[target];
    Ok((path, distance))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bellman_ford_basic() {
        let mut graph: Graph = vec![vec![]; 5];
        graph[0].push(Edge { node: 1, cost: 4 });
        graph[0].push(Edge { node: 2, cost: 2 });
        graph[1].push(Edge { node: 2, cost: 1 });
        graph[2].push(Edge { node: 3, cost: 1 });
        graph[1].push(Edge { node: 3, cost: 5 });

        let result = bellman_ford(&graph, 0);
        assert!(result.is_ok());
        let (dist, _) = result.unwrap();
        assert_eq!(dist[3], 4);
    }

    #[test]
    fn test_bellman_ford_negative_cycle() {
        let mut graph: Graph = vec![vec![]; 3];
        graph[0].push(Edge { node: 1, cost: 1 });
        graph[1].push(Edge { node: 2, cost: 3 });
        graph[2].push(Edge { node: 0, cost: 1 }); // Creates negative cycle

        let result = bellman_ford(&graph, 0);
        assert!(result.is_err());
    }
}
