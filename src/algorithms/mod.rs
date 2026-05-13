/// Dijkstra's Shortest Path Algorithm
/// Time Complexity: O((V + E) log V)
/// Best for: Non-negative weights, single-source shortest paths
pub mod dijkstra;

/// Bellman-Ford Algorithm
/// Time Complexity: O(V * E)
/// Best for: Negative weights, single-source shortest paths, cycle detection
pub mod bellman_ford;

/// Floyd-Warshall Algorithm
/// Time Complexity: O(V³)
/// Best for: All-pairs shortest paths, dense graphs
pub mod floyd_warshall;

// Re-export commonly used items
pub use dijkstra::dijkstra;
pub use bellman_ford::bellman_ford;
pub use floyd_warshall::floyd_warshall;
