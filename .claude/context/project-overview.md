# algs4-rust Project Context

## Project Overview

This is a Rust port of the algorithms from "Algorithms, 4th Edition" by Robert Sedgewick and Kevin Wayne. The original Java implementation is at https://github.com/kevin-wayne/algs4.

## Key Goals

1. **Correctness:** Accurately implement all algorithms from the original
2. **Clarity:** Maintain the educational nature of the code
3. **Idiomatic Rust:** Use Rust best practices and patterns
4. **Performance:** Leverage Rust's zero-cost abstractions
5. **Safety:** Use Rust's type system to prevent bugs

## Project Structure

The project is organized as a Cargo workspace with separate crates for each major topic:

- `modules/fundamentals/` - Basic data structures, I/O, utilities
- `modules/sorting/` - All sorting algorithms
- `modules/searching/` - Binary search trees, hash tables, tries
- `modules/graphs/` - Graph algorithms, shortest paths, MST, flow
- `modules/strings/` - String processing, pattern matching, compression
- `modules/geometry/` - Geometric algorithms, union-find
- `modules/advanced/` - Advanced topics (FFT, linear programming, etc.)

## Conversion Strategy

The conversion is divided into 10 phases (see CONVERSION_PLAN.md):

- **Phase 0:** Project foundation (current)
- **Phase 1-2:** Fundamentals (I/O, basic collections)
- **Phase 3-4:** Sorting and priority queues
- **Phase 5:** Searching and symbol tables
- **Phase 6-7:** Graph algorithms
- **Phase 8:** String processing
- **Phase 9:** Geometry and union-find
- **Phase 10:** Advanced topics

Each phase is designed to be manageable within a single Claude session or by a sub-agent.

## Coding Standards

### Naming Conventions
- Java classes → Rust modules/structs
- `camelCase` → `snake_case`
- Keep algorithm names recognizable

### Type Mappings
- `int` → `i32` or `usize` (for indices)
- `double` → `f64`
- `boolean` → `bool`
- `null` → `Option<T>`
- Arrays → `Vec<T>` or slices `&[T]`

### Error Handling
- Use `Result<T, E>` for fallible operations
- Use `Option<T>` for optional values
- Use `panic!` only for contract violations

### Documentation
- Use `///` for public API documentation
- Include examples in doc comments
- Document time and space complexity
- Reference textbook sections when relevant

### Testing
- Write unit tests for all public functions
- Include integration tests for complex algorithms
- Add benchmarks for performance-critical code
- Aim for >80% code coverage

## Dependencies

Minimize dependencies where possible:
- `rand` - Random number generation (phase 1)
- `byteorder` - Binary I/O (phase 1)

Optional dependencies (behind feature flags):
- Graphics/visualization crates (for StdDraw equivalent)
- Audio crates (for StdAudio equivalent)
- Image processing (for Picture equivalent)

## Common Patterns

### Iterators
All collections should implement `Iterator`:
```rust
impl<T> Iterator for Stack<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> { ... }
}
```

### Generic Algorithms
Use trait bounds for generic code:
```rust
pub fn quick_sort<T: Ord>(arr: &mut [T]) { ... }
```

### Graph Representation
Use adjacency lists with `Vec<Vec<usize>>`:
```rust
pub struct Graph {
    v: usize,              // number of vertices
    e: usize,              // number of edges
    adj: Vec<Vec<usize>>,  // adjacency lists
}
```

## Important Files

- `CONVERSION_PLAN.md` - Complete roadmap and guidelines
- `Cargo.toml` - Workspace configuration
- `README.md` - Project documentation
- `.claude/` - Claude Code context and commands

## Git Workflow

- Work on branch: `claude/algs4-java-to-rust-011CV5y1Rjk75TTTXhAdfXbX`
- Commit regularly with descriptive messages
- Push when phase is complete
- Create PR for review (if applicable)

## Resources

- Original Java repo: https://github.com/kevin-wayne/algs4
- Textbook site: https://algs4.cs.princeton.edu/
- Rust book: https://doc.rust-lang.org/book/
- Rust API guidelines: https://rust-lang.github.io/api-guidelines/
