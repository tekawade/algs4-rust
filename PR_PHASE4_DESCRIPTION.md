## Phase 4: Core Priority Queues

This PR implements the 4 fundamental priority queue data structures from *Algorithms, 4th Edition*. Advanced priority queues (Binomial, Fibonacci, Multiway heaps) are deferred to Phase 11 as they are specialized structures primarily for advanced graph algorithms.

## Summary

Implemented priority queue data structures with binary heap representation:
- **MaxPQ** - Maximum priority queue with O(log n) insert/delete
- **MinPQ** - Minimum priority queue with O(log n) insert/delete
- **IndexMaxPQ** - Indexed maximum priority queue with change-key operations
- **IndexMinPQ** - Indexed minimum priority queue with change-key operations

## Key Features

✅ **Complete Implementation**
- Binary heap with 1-based array indexing
- Automatic resizing (2x growth, 1/4 shrink threshold)
- Iterator support (both borrowing and consuming)
- Three-array structure for indexed priority queues (pq, qp, keys)

✅ **Comprehensive Testing**
- 48 new unit tests (177 total workspace tests)
- 149 doc tests passing
- Heap property validation
- Edge cases: empty, single element, large datasets
- Index operations: insert, delete, change, increase, decrease

✅ **Quality Metrics**
- Zero clippy warnings (strict mode with `-D warnings`)
- Complete API documentation with examples
- Performance characteristics documented
- All tests passing

## Implementation Details

**Basic Priority Queues (MaxPQ/MinPQ):**
- 1-based Vec<Option<T>> for heap storage
- Parent at k/2, children at 2k and 2k+1
- Swim (bottom-up) and sink (top-down) operations
- O(log n) insert/delete, O(1) peek

**Indexed Priority Queues (IndexMaxPQ/IndexMinPQ):**
- Three synchronized arrays: pq (heap), qp (inverse), keys (values)
- O(log n) change-key, increase-key, decrease-key operations
- Essential for graph algorithms (Dijkstra, Prim's MST)
- Index range: 0 to max_n-1

## Testing

```bash
# All tests passing
cargo test --workspace
# Result: 177 passed; 0 failed

# Priority queue tests
cargo test -p algs4-fundamentals priority_queue
# Result: 48 passed; 0 failed

# Clippy clean
cargo clippy --workspace -- -D warnings
# Result: 0 warnings
```

## Documentation

- **SESSION_PHASE4_SUMMARY.md** - Complete phase summary with metrics
- **START_PHASE3.md** - Detailed plan for next phase (Sorting Algorithms)
- **PROGRESS.md** - Updated to 22/160 files (13.75%)

## Deferred Items

Advanced priority queues moved to Phase 11:
- BinomialMinPQ, FibonacciMinPQ
- IndexBinomialMinPQ, IndexFibonacciMinPQ
- MultiwayMinPQ, IndexMultiwayMinPQ

**Rationale:** These are complex specialized structures mainly for advanced graph algorithms. The 4 core priority queues cover 95%+ of practical use cases.

## References

- **Textbook:** Section 2.4 - Priority Queues
- **Website:** https://algs4.cs.princeton.edu/24pq
- **Java Source:** https://github.com/kevin-wayne/algs4

## Next Phase

**Recommended:** Phase 3 - Sorting Algorithms (18 files)
- Natural progression (heapsort uses priority queues)
- Independent module with no dependencies
- Foundational algorithms for later phases

## Checklist

- [x] All 4 core priority queues implemented
- [x] 48 unit tests passing
- [x] 149 doc tests passing
- [x] Zero clippy warnings
- [x] Complete documentation
- [x] PROGRESS.md updated
- [x] Session summary created
- [x] Next phase planned
