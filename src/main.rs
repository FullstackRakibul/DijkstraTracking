use dijkstra_tracking::{
    algorithms::{dijkstra, bellman_ford, floyd_warshall},
    graph::{Edge, Graph, reconstruct_path},
};

fn create_sample_graph() -> Graph {
    let mut graph: Graph = vec![vec![]; 6];

    // Weighted directed graph
    graph[0].push(Edge { node: 1, cost: 4 });
    graph[0].push(Edge { node: 2, cost: 1 });

    graph[2].push(Edge { node: 1, cost: 2 });
    graph[2].push(Edge { node: 3, cost: 5 });

    graph[1].push(Edge { node: 3, cost: 1 });

    graph[3].push(Edge { node: 4, cost: 3 });

    graph[4].push(Edge { node: 5, cost: 1 });

    graph
}

fn main() {
    let graph = create_sample_graph();
    let start = 0;
    let target = 5;

    let separator = "=".repeat(60);
    let line = "-".repeat(60);

    println!("{}", separator);
    println!("SHORTEST PATH ALGORITHMS DEMONSTRATION");
    println!("{}", separator);

    // ==================== DIJKSTRA'S ALGORITHM ====================
    println!("\n1. DIJKSTRA'S ALGORITHM");
    println!("{}", line);
    println!("Time Complexity: O((V + E) log V)");
    println!("Best for: Non-negative weights, single-source shortest paths\n");

    let (dist_dijkstra, parent_dijkstra) = dijkstra::dijkstra(&graph, start);

    println!("Shortest distances from vertex {}:", start);
    for i in 0..dist_dijkstra.len() {
        if dist_dijkstra[i] == usize::MAX {
            println!("  {} -> {} = UNREACHABLE", start, i);
        } else {
            println!("  {} -> {} = {}", start, i, dist_dijkstra[i]);
        }
    }

    let path_dijkstra = reconstruct_path(&parent_dijkstra, target);
    println!(
        "\nShortest path from {} to {}: {:?}",
        start, target, path_dijkstra
    );
    println!("Distance: {}", dist_dijkstra[target]);

    // ==================== BELLMAN-FORD ALGORITHM ====================
    println!("\n{}", separator);
    println!("2. BELLMAN-FORD ALGORITHM");
    println!("{}", line);
    println!("Time Complexity: O(V * E)");
    println!(
        "Best for: Negative weights, single-source, cycle detection\n"
    );

    match bellman_ford::bellman_ford(&graph, start) {
        Ok((dist_bf, parent_bf)) => {
            println!("Shortest distances from vertex {}:", start);
            for i in 0..dist_bf.len() {
                if dist_bf[i] == i32::MAX {
                    println!("  {} -> {} = UNREACHABLE", start, i);
                } else {
                    println!("  {} -> {} = {}", start, i, dist_bf[i]);
                }
            }

            let path_bf = reconstruct_path(&parent_bf, target);
            println!(
                "\nShortest path from {} to {}: {:?}",
                start, target, path_bf
            );
            println!("Distance: {}", dist_bf[target]);
        }
        Err(e) => println!("Error: {}", e),
    }

    // ==================== FLOYD-WARSHALL ALGORITHM ====================
    println!("\n{}", separator);
    println!("3. FLOYD-WARSHALL ALGORITHM");
    println!("{}", line);
    println!("Time Complexity: O(V³)");
    println!("Best for: All-pairs shortest paths, dense graphs\n");

    let (dist_matrix, next_matrix) = floyd_warshall::floyd_warshall(&graph);

    println!("Shortest distances from vertex {}:", start);
    for i in 0..dist_matrix[start].len() {
        let dist = dist_matrix[start][i];
        if dist >= i32::MAX / 2 {
            println!("  {} -> {} = UNREACHABLE", start, i);
        } else {
            println!("  {} -> {} = {}", start, i, dist);
        }
    }

    let path_fw = floyd_warshall::reconstruct_fw_path(&next_matrix, start, target)
        .unwrap_or_default();
    println!(
        "\nShortest path from {} to {}: {:?}",
        start, target, path_fw
    );
    println!("Distance: {}", dist_matrix[start][target]);

    // ==================== COMPARISON ====================
    println!("\n{}", separator);
    println!("ALGORITHM COMPARISON");
    println!("{}", line);
    println!("Distance from {} to {} (all algorithms): {}", start, target, dist_dijkstra[target]);
    println!("\nAlgorithm                 | Time Complexity  | Space Complexity | Best Use Case");
    println!("{}", "-".repeat(90));
    println!("Dijkstra                  | O((V+E) log V)   | O(V)             | Single-source, non-negative");
    println!("Bellman-Ford              | O(V * E)         | O(V)             | Negative weights, cycle detect");
    println!("Floyd-Warshall            | O(V³)            | O(V²)            | All-pairs shortest paths");

    println!("\n{}", separator);
}