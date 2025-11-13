# algs4-rust

> Rust implementation of algorithms from "Algorithms, 4th Edition" by Robert Sedgewick and Kevin Wayne

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

## Overview

This project is a Rust port of the Java implementation from [kevin-wayne/algs4](https://github.com/kevin-wayne/algs4), containing fundamental algorithms and data structures from the acclaimed textbook *Algorithms, 4th Edition*.

The implementation follows Rust best practices while maintaining the educational clarity of the original Java code.

## Features

- ✅ **Memory Safe:** Leverages Rust's ownership system to prevent common bugs
- ✅ **Zero-Cost Abstractions:** Performance comparable to hand-written C code
- ✅ **Type Safe:** Strong static typing catches errors at compile time
- ✅ **Well Documented:** Comprehensive documentation with examples
- ✅ **Thoroughly Tested:** Unit tests, integration tests, and benchmarks
- ✅ **Idiomatic:** Follows Rust API guidelines and conventions

## Project Status

🚧 **Work in Progress** - Currently in Phase 0 (Project Foundation)

See [CONVERSION_PLAN.md](CONVERSION_PLAN.md) for detailed roadmap and progress tracking.

## Project Structure

```
algs4-rust/
├── modules/
│   ├── fundamentals/     # Basic data structures and utilities
│   ├── sorting/          # Sorting algorithms
│   ├── searching/        # Search algorithms and symbol tables
│   ├── graphs/           # Graph algorithms
│   ├── strings/          # String processing algorithms
│   ├── geometry/         # Geometric algorithms
│   └── advanced/         # Advanced topics
├── examples/             # Usage examples
└── tests/                # Integration tests
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
algs4-fundamentals = "0.1"
algs4-sorting = "0.1"
algs4-searching = "0.1"
algs4-graphs = "0.1"
# ... other modules as needed
```

Or to use all modules:

```toml
[dependencies]
algs4 = "0.1"
```

## Usage

```rust
use algs4_fundamentals::{Stack, Queue};
use algs4_sorting::quick_sort;
use algs4_graphs::Graph;

fn main() {
    // Use a stack
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    println!("Popped: {}", stack.pop().unwrap());

    // Sort an array
    let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
    quick_sort(&mut arr);
    println!("Sorted: {:?}", arr);

    // Create a graph
    let graph = Graph::new(5);
    // ... add edges and run algorithms
}
```

## Building from Source

```bash
# Clone the repository
git clone https://github.com/tekawade/algs4-rust.git
cd algs4-rust

# Build all modules
cargo build --release

# Run tests
cargo test --all

# Run benchmarks
cargo bench --all

# Generate documentation
cargo doc --open --no-deps
```

## Development

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Quick Start

```bash
# Check code compiles
cargo check --all

# Format code
cargo fmt --all

# Run linter
cargo clippy --all -- -D warnings

# Run tests with coverage
cargo test --all -- --nocapture
```

### Project Guidelines

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Write comprehensive tests for all public APIs
- Document all public items with examples
- Run `cargo fmt` and `cargo clippy` before committing
- See [CONVERSION_PLAN.md](CONVERSION_PLAN.md) for detailed contribution guidelines

## Modules

| Module | Description | Status |
|--------|-------------|--------|
| `fundamentals` | Basic data structures, I/O, utilities | 🔜 Planned |
| `sorting` | Sorting algorithms | 🔜 Planned |
| `searching` | Binary search trees, hash tables, tries | 🔜 Planned |
| `graphs` | Graph algorithms, shortest paths, MST | 🔜 Planned |
| `strings` | String processing, pattern matching | 🔜 Planned |
| `geometry` | Geometric algorithms, union-find | 🔜 Planned |
| `advanced` | Linear programming, FFT, advanced topics | 🔜 Planned |

## Documentation

- [Conversion Plan](CONVERSION_PLAN.md) - Detailed roadmap and guidelines
- [API Documentation](https://docs.rs/algs4) - Auto-generated docs (coming soon)
- [Original Textbook](https://algs4.cs.princeton.edu/) - Reference material

## Performance

Performance benchmarks will be added as modules are implemented. The goal is to match or exceed the performance of the Java implementation while maintaining code clarity.

## Testing

The project includes:

- **Unit tests** - Test individual functions and methods
- **Integration tests** - Test module interactions
- **Doc tests** - Ensure documentation examples work
- **Property-based tests** - Verify algorithmic properties (planned)
- **Benchmarks** - Track performance (planned)

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

This license is inherited from the original [kevin-wayne/algs4](https://github.com/kevin-wayne/algs4) repository.

## Credits

- **Original Authors:** Robert Sedgewick and Kevin Wayne
- **Original Java Implementation:** [kevin-wayne/algs4](https://github.com/kevin-wayne/algs4)
- **Textbook:** *Algorithms, 4th Edition* by Sedgewick & Wayne

## Contributing

Contributions are welcome! Please read the [CONVERSION_PLAN.md](CONVERSION_PLAN.md) for guidelines on:

- Code style and conventions
- Testing requirements
- Documentation standards
- Development workflow

## Related Projects

- [algs4 (Java)](https://github.com/kevin-wayne/algs4) - Original Java implementation
- [Algorithms (Rust)](https://github.com/TheAlgorithms/Rust) - General algorithms in Rust
- [petgraph](https://github.com/petgraph/petgraph) - Graph data structures for Rust

## Acknowledgments

Special thanks to Robert Sedgewick and Kevin Wayne for creating the original algorithms and making them freely available to the programming community.

---

**Note:** This project is for educational purposes and follows the same GPL-3.0 license as the original work.
