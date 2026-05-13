# 🚀 Dijkstra Tracking

<div align="center">

![Rust](https://img.shields.io/badge/Language-Rust-orange?style=for-the-badge&logo=rust)
![Version](https://img.shields.io/badge/Version-0.1.0-blue?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)
![Status](https://img.shields.io/badge/Status-Active-success?style=for-the-badge)

<br/>

```
    ╔════════════════════════════════════════╗
    ║   🗺️  DIJKSTRA PATHFINDING ENGINE  🗺️   ║
    ║   High-Performance Graph Algorithms    ║
    ╚════════════════════════════════════════╝
```

**A comprehensive Rust library implementing advanced graph algorithms with real-time tracking capabilities**

</div>

---

## ✨ Features

- ⚡ **Dijkstra's Algorithm** - Fast shortest path computation
- 🔄 **Bellman-Ford Algorithm** - Handle negative edge weights
- 📊 **Floyd-Warshall Algorithm** - All-pairs shortest paths
- 📈 **Real-time Tracking** - Monitor algorithm execution step-by-step
- 🏗️ **Modular Architecture** - Clean, extensible codebase
- 🦀 **Pure Rust** - Zero unsafe code, type-safe implementation

---

## 🎯 Quick Start

### Prerequisites

- Rust 1.56+ ([Install Rust](https://www.rust-lang.org/tools/install))

### Installation

Clone the repository:

```bash
git clone https://github.com/yourusername/dijkstra-tracking.git
cd dijkstra-tracking
```

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --release
```

---

## 📚 Project Structure

```
dijkstra-tracking/
├── src/
│   ├── main.rs              # Application entry point
│   ├── lib.rs               # Library root
│   ├── algorithms/          # Algorithm implementations
│   │   ├── dijkstra.rs      # Dijkstra's shortest path
│   │   ├── bellman_ford.rs  # Bellman-Ford algorithm
│   │   ├── floyd_warshall.rs # Floyd-Warshall algorithm
│   │   └── mod.rs           # Module exports
│   └── graph/               # Graph data structures
│       └── mod.rs           # Graph module
├── Cargo.toml               # Project manifest
├── Cargo.lock               # Dependency lock file
└── README.md                # This file
```

---

## 🔧 Algorithms Overview

### Dijkstra's Algorithm
Find the shortest path between nodes in a weighted graph with non-negative weights.

```rust
let path = dijkstra::find_shortest_path(&graph, start, end);
```

**Complexity:** O((V + E) log V)

### Bellman-Ford Algorithm
Compute shortest paths even with negative edge weights.

```rust
let distances = bellman_ford::compute_distances(&graph, source);
```

**Complexity:** O(V × E)

### Floyd-Warshall Algorithm
Find shortest paths between all pairs of vertices.

```rust
let all_pairs = floyd_warshall::compute_all_pairs(&graph);
```

**Complexity:** O(V³)

---

## 🚀 Performance

| Algorithm | Time Complexity | Space Complexity | Best For |
|-----------|-----------------|------------------|----------|
| Dijkstra | O((V+E)log V) | O(V) | Single source, non-negative weights |
| Bellman-Ford | O(V×E) | O(V) | Negative weights, small graphs |
| Floyd-Warshall | O(V³) | O(V²) | All-pairs, dense graphs |

---

## 💡 Usage Examples

### Example 1: Finding Shortest Path

```rust
use dijkstra_tracking::algorithms::dijkstra;
use dijkstra_tracking::graph::Graph;

fn main() {
    let mut graph = Graph::new();
    
    // Add edges: (from, to, weight)
    graph.add_edge(0, 1, 4);
    graph.add_edge(0, 2, 2);
    graph.add_edge(1, 3, 1);
    graph.add_edge(2, 3, 5);
    
    // Find shortest path
    if let Some(path) = dijkstra::find_shortest_path(&graph, 0, 3) {
        println!("Shortest path: {:?}", path);
    }
}
```

### Example 2: All-Pairs Shortest Paths

```rust
use dijkstra_tracking::algorithms::floyd_warshall;

fn main() {
    let graph = create_graph();
    let distances = floyd_warshall::compute_all_pairs(&graph);
    
    println!("All shortest distances: {:?}", distances);
}
```

---

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

Run with output:

```bash
cargo test -- --nocapture
```

Run a specific test:

```bash
cargo test test_dijkstra
```

---

## 📊 Benchmarking

Create benchmarks in `benches/` directory and run:

```bash
cargo bench
```

---

## 🐛 Known Issues

None currently. Please report issues via GitHub Issues.

---

## 🤝 Contributing

We welcome contributions! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Style

- Follow Rust naming conventions
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes with no warnings

---

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 👨‍💻 Authors

- **Your Name** - *Initial work*

---

## 🙏 Acknowledgments

- Inspired by classic computer science algorithms
- Built with ❤️ in Rust

---

## 📞 Support

For questions and support, please open an issue on GitHub.

<div align="center">

**⭐ Star us on GitHub if this project helped you!**

</div>
