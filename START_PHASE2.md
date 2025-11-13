# Quick Start: Phase 2

## TL;DR - Start Phase 2 Now

```bash
# 1. Get the code
cd /home/user/algs4-rust
git checkout claude/algs4-java-to-rust-011CV5y1Rjk75TTTXhAdfXbX
git pull

# 2. Verify Phase 1 works
cargo test -p algs4-fundamentals
# Should see: "test result: ok. 41 passed"

# 3. Read the plan
cat CONVERSION_PLAN.md | sed -n '234,307p'  # Phase 2 section

# 4. Start coding!
mkdir -p modules/fundamentals/src/collections
mkdir -p modules/fundamentals/src/union_find
```

## Phase 2: Collections & Union-Find

**Goal:** Implement 16 fundamental data structures
**Time:** 3 sessions (estimated)
**Priority:** HIGH

### Files to Implement (in order)

#### Start Here (Day 1)
1. ✅ `LinkedStack` - Easiest linked structure
2. ✅ `LinkedQueue` - Learn iteration
3. ✅ `LinkedBag` - Consolidate pattern

#### Then (Day 2)
4. ✅ `ResizingArrayStack` - Array resizing
5. ✅ `ResizingArrayQueue` - Circular array
6. ✅ `ResizingArrayBag` - Consolidate arrays

#### Finally (Day 3)
7. ✅ `UF` interface
8. ✅ `QuickFindUF`
9. ✅ `QuickUnionUF`
10. ✅ `WeightedQuickUnionUF` - With path compression
11. ✅ `Stack`, `Queue`, `Bag` (wrapper types)
12. ✅ `SET`, `ST`, `Knuth`

## Quick Code Template

### Linked Stack Template
```rust
// modules/fundamentals/src/collections/linked_stack.rs

pub struct LinkedStack<T> {
    first: Option<Box<Node<T>>>,
    size: usize,
}

struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedStack<T> {
    pub fn new() -> Self {
        Self { first: None, size: 0 }
    }

    pub fn push(&mut self, item: T) {
        let old_first = self.first.take();
        self.first = Some(Box::new(Node {
            item,
            next: old_first,
        }));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.first.take().map(|node| {
            self.first = node.next;
            self.size -= 1;
            node.item
        })
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.first.is_none()
    }
}

// TODO: Implement Iterator, IntoIterator, Debug, Default

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut stack = LinkedStack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }
}
```

### Union-Find Template
```rust
// modules/fundamentals/src/union_find/weighted_quick_union_uf.rs

pub struct WeightedQuickUnionUF {
    parent: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl WeightedQuickUnionUF {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            count: n,
        }
    }

    pub fn find(&mut self, mut p: usize) -> usize {
        while p != self.parent[p] {
            // Path compression
            self.parent[p] = self.parent[self.parent[p]];
            p = self.parent[p];
        }
        p
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let root_p = self.find(p);
        let root_q = self.find(q);

        if root_p == root_q {
            return;
        }

        // Weighted union
        if self.size[root_p] < self.size[root_q] {
            self.parent[root_p] = root_q;
            self.size[root_q] += self.size[root_p];
        } else {
            self.parent[root_q] = root_p;
            self.size[root_p] += self.size[root_q];
        }

        self.count -= 1;
    }

    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn count(&self) -> usize {
        self.count
    }
}
```

## Java Source References

```
https://raw.githubusercontent.com/kevin-wayne/algs4/master/src/main/java/edu/princeton/cs/algs4/

Stack.java
LinkedStack.java
Queue.java
LinkedQueue.java
Bag.java
LinkedBag.java
UF.java
WeightedQuickUnionUF.java
```

## Key Files to Reference

1. **CONVERSION_PLAN.md** (lines 234-307) - Full Phase 2 spec
2. **SESSION_SUMMARY_PHASE1.md** - Detailed handoff notes
3. **.claude/context/conversion-guidelines.md** - Rust patterns
4. **PROGRESS.md** (lines 70-102) - Phase 2 checklist

## Testing Command

```bash
# After implementing each structure:
cargo test -p algs4-fundamentals
cargo clippy -p algs4-fundamentals -- -D warnings
cargo fmt --all

# Run example (once created):
cargo run -p algs4-fundamentals --example phase2_demo
```

## Session Goal

By end of session:
- [ ] At least 5-6 structures implemented
- [ ] All tests passing
- [ ] Code documented with examples
- [ ] Progress committed regularly
- [ ] PROGRESS.md updated

## Ready?

**Start with:** `LinkedStack` - it's the simplest!

See SESSION_SUMMARY_PHASE1.md for complete details.
