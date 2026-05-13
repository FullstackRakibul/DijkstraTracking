use crate::graph::Graph;

const INF: i32 = i32::MAX / 2;

/// Floyd-Warshall All-Pairs Shortest Path Algorithm
/// Time Complexity: O(V³)
/// Space Complexity: O(V²)
/// 
/// Key Features:
/// - Computes shortest paths between all pairs of vertices
/// - Can handle negative edge weights (but no negative cycles)
/// - Works with dense graphs
/// - Can detect negative cycles
/// 
/// # Arguments
/// * `graph` - Adjacency list representation of the graph
/// 
/// # Returns
/// A tuple containing:
/// * `distance_matrix` - V x V matrix of shortest distances between all pairs
/// * `next_matrix` - V x V matrix for path reconstruction
pub fn floyd_warshall(graph: &Graph) -> (Vec<Vec<i32>>, Vec<Vec<Option<usize>>>) {
    let n = graph.len();

    // Initialize distance matrix
    let mut dist = vec![vec![INF; n]; n];
    let mut next = vec![vec![None; n]; n];

    // Distance from a node to itself is 0
    for i in 0..n {
        dist[i][i] = 0;
    }

    // Fill initial distances from adjacency list
    for u in 0..n {
        for edge in &graph[u] {
            let v = edge.node;
            let weight = edge.cost as i32;
            dist[u][v] = weight;
            next[u][v] = Some(v);
        }
    }

    // Floyd-Warshall main algorithm
    // Try all intermediate vertices
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                // Check if path through k is shorter
                if dist[i][k] != INF && dist[k][j] != INF {
                    let new_dist = dist[i][k] + dist[k][j];
                    if new_dist < dist[i][j] {
                        dist[i][j] = new_dist;
                        next[i][j] = next[i][k];
                    }
                }
            }
        }
    }

    (dist, next)
}

/// Reconstruct path between two vertices using Floyd-Warshall result
pub fn reconstruct_fw_path(next: &[Vec<Option<usize>>], start: usize, end: usize) -> Option<Vec<usize>> {
    if next[start][end].is_none() {
        return None;
    }

    let mut path = vec![start];
    let mut current = start;

    while current != end {
        current = next[current][end]?;
        path.push(current);
    }

    Some(path)
}

/// Get shortest distance and path between two nodes
pub fn get_shortest_path(
    graph: &Graph,
    start: usize,
    end: usize,
) -> (Vec<usize>, i32) {
    let (dist_matrix, next_matrix) = floyd_warshall(graph);
    
    let distance = dist_matrix[start][end];
    let path = reconstruct_fw_path(&next_matrix, start, end).unwrap_or_default();
    
    (path, distance)
}

/// Get all distances from a source vertex
pub fn get_distances_from(graph: &Graph, start: usize) -> Vec<i32> {
    let (dist_matrix, _) = floyd_warshall(graph);
    dist_matrix[start].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floyd_warshall_basic() {
        let mut graph: Graph = vec![vec![]; 4];
        graph[0].push(crate::graph::Edge { node: 1, cost: 5 });
        graph[0].push(crate::graph::Edge { node: 3, cost: 10 });
        graph[1].push(crate::graph::Edge { node: 2, cost: 3 });
        graph[2].push(crate::graph::Edge { node: 3, cost: 1 });

        let (dist, _) = floyd_warshall(&graph);
        
        assert_eq!(dist[0][0], 0);
        assert_eq!(dist[0][2], 8); // 0->1->2
        assert_eq!(dist[0][3], 9); // 0->1->2->3
    }

    #[test]
    fn test_floyd_warshall_path_reconstruction() {
        let mut graph: Graph = vec![vec![]; 3];
        graph[0].push(crate::graph::Edge { node: 1, cost: 2 });
        graph[1].push(crate::graph::Edge { node: 2, cost: 3 });

        let (_, next) = floyd_warshall(&graph);
        let path = reconstruct_fw_path(&next, 0, 2).unwrap();
        
        assert_eq!(path, vec![0, 1, 2]);
    }
}
