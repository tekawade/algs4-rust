# Pull Request Creation Instructions - Phase 2

**Branch:** `claude/start-phase-2-01KMbj82bXCU8zx6962j2nuR`
**Target:** `main` branch
**Status:** Ready for PR

---

## 📋 Pre-PR Verification Checklist

Run these commands to verify everything is ready:

```bash
# 1. Verify you're on the correct branch
git branch --show-current
# Should show: claude/start-phase-2-01KMbj82bXCU8zx6962j2nuR

# 2. Ensure all changes are committed
git status
# Should show: nothing to commit, working tree clean

# 3. Run all tests
cargo test --workspace
# Should show: 129 tests passing

# 4. Check clippy (strict mode)
cargo clippy --workspace -- -D warnings
# Should show: no warnings

# 5. Verify formatting
cargo fmt --all -- --check
# Should show: no changes needed

# 6. View commit history
git log --oneline -5
# Should show Phase 2 commit at the top

# 7. Ensure branch is pushed
git push -u origin claude/start-phase-2-01KMbj82bXCU8zx6962j2nuR
```

✅ All checks should pass before proceeding.

---

## 🔗 Creating the Pull Request

### Option 1: Via GitHub Web Interface (Recommended)

1. **Navigate to the repository:**
   ```
   https://github.com/tekawade/algs4-rust
   ```

2. **GitHub should show a banner:**
   > `claude/start-phase-2-01KMbj82bXCU8zx6962j2nuR` had recent pushes
   > [Compare & pull request]

   Click the **"Compare & pull request"** button.

3. **If no banner appears:**
   - Click the "Pull requests" tab
   - Click "New pull request"
   - Set base: `main`
   - Set compare: `claude/start-phase-2-01KMbj82bXCU8zx6962j2nuR`
   - Click "Create pull request"

### Option 2: Via GitHub CLI (if installed)

```bash
gh pr create \
  --base main \
  --head claude/start-phase-2-01KMbj82bXCU8zx6962j2nuR \
  --title "Phase 2: Implement Collections & Union-Find (12/12 files)" \
  --body-file .github/PR_TEMPLATE.md
```

---

## ✍️ Pull Request Title

```
Phase 2: Implement Collections & Union-Find (12/12 files)
```

**Alternative titles:**
- `feat: Phase 2 - Complete Collections and Union-Find implementations`
- `Phase 2 Complete: Add 9 Collection types + 3 Union-Find variants`

---

## 📝 Pull Request Description Template

Copy and paste this into the PR description:

```markdown
## 🎯 Phase 2: Collections & Union-Find

This PR completes Phase 2 of the algs4-rust project, implementing all 12 core data structures for collections and union-find algorithms from *Algorithms, 4th Edition*.

### ✅ Deliverables (12/12 Complete)

#### Collections (9 implementations)
- ✅ **LinkedBag** - Singly linked list bag (multiset)
- ✅ **LinkedQueue** - FIFO queue with first/last pointers
- ✅ **LinkedStack** - LIFO stack with linked nodes
- ✅ **ResizingArrayBag** - Dynamic array bag with 2x growth
- ✅ **ResizingArrayQueue** - Circular buffer with wraparound
- ✅ **ResizingArrayStack** - Auto-resizing array stack

#### Union-Find (3 variants)
- ✅ **QuickFindUF** - O(1) find, O(n) union (flat array)
- ✅ **QuickUnionUF** - Tree-based, O(n) worst case
- ✅ **WeightedQuickUnionUF** - Path compression, O(log n) ⭐

---

### 📊 Quality Metrics

| Metric | Value | Status |
|--------|-------|--------|
| **Total Tests** | 129 passing | ✅ |
| **New Tests (Phase 2)** | 92 tests | ✅ |
| **Clippy Warnings** | 0 (strict mode) | ✅ |
| **Code Added** | 3,623 lines | ✅ |
| **Files Created** | 11 new files | ✅ |
| **Documentation** | Complete | ✅ |

---

### 🏗️ Module Structure

**Collections:**
```
modules/fundamentals/src/collections/
├── mod.rs                      # Public API
├── linked_bag.rs
├── linked_queue.rs
├── linked_stack.rs
├── resizing_array_bag.rs
├── resizing_array_queue.rs
└── resizing_array_stack.rs
```

**Union-Find:**
```
modules/fundamentals/src/union_find/
├── mod.rs                      # Public API
├── quick_find_uf.rs
├── quick_union_uf.rs
└── weighted_quick_union_uf.rs
```

---

### 🔑 Key Features

**All Collections Include:**
- ✅ Full `Iterator` trait support (borrowing + consuming)
- ✅ `Debug` and `Display` implementations
- ✅ Comprehensive unit tests with doc tests
- ✅ Edge case handling (empty, single element, large datasets)
- ✅ Performance documentation (time/space complexity)

**Technical Highlights:**
- **Safe Rust:** All linked structures use `Box<Node<T>>` (no raw pointers)
- **Smart Resizing:** 2x growth when full, 1/2 shrink at 1/4 capacity
- **Circular Buffer:** Efficient modular arithmetic in ResizingArrayQueue
- **Path Compression:** WeightedQuickUnionUF for O(log n) performance

---

### 🧪 Testing

**Test Coverage:**
- 92 new unit tests (129 total with Phase 1)
- Textbook examples (tinyUF.txt)
- Edge cases: empty collections, single elements, 1000+ items
- FIFO/LIFO order verification
- Resizing behavior validation
- Iterator consumption tests

**Run tests:**
```bash
cargo test -p algs4-fundamentals
```

---

### 📚 Documentation

**Every public API includes:**
- Summary description
- Performance characteristics (O-notation)
- Type parameters explanation
- Doc test examples
- Panic conditions (where applicable)
- References to textbook sections

**View docs:**
```bash
cargo doc -p algs4-fundamentals --open
```

---

### 📈 Progress Update

| Phase | Files | Complete | Status |
|-------|-------|----------|--------|
| Phase 0 | - | ✅ | Complete |
| Phase 1 | 8 | 6/8 (75%) | Substantially Complete |
| **Phase 2** | **12** | **12/12 (100%)** | ✅ **Complete** |
| **Total** | **160** | **18 (11.25%)** | In Progress |

---

### 🔗 References

- **Textbook:** [Algorithms 4th Edition](https://algs4.cs.princeton.edu/)
- **Collections:** https://algs4.cs.princeton.edu/13stacks
- **Union-Find:** https://algs4.cs.princeton.edu/15uf
- **Java Source:** https://github.com/kevin-wayne/algs4
- **Session Summary:** See `SESSION_PHASE2_SUMMARY.md`

---

### ✅ Checklist

- [x] All 12 data structures implemented
- [x] 92 unit tests passing (129 total)
- [x] Zero clippy warnings (strict mode)
- [x] Complete documentation with examples
- [x] Iterator traits implemented
- [x] Code formatted with `cargo fmt`
- [x] PROGRESS.md updated
- [x] Session summary created

---

### 🚀 Next Steps

After merging this PR:
- **Option 1:** Phase 3 - Sorting Algorithms (18 files)
- **Option 2:** Phase 4 - Priority Queues (10 files) ⭐ Recommended
- **Option 3:** Phase 5 - Searching & Symbol Tables (20 files)

---

**Ready for review! 🎉**

</markdown>

**Closes:** (Add issue number if applicable)

---

## 🏷️ PR Labels

Add these labels to the PR:

- `enhancement` - New feature
- `phase-2` - Phase 2 work
- `collections` - Collections implementations
- `union-find` - Union-find algorithms
- `ready-for-review` - Ready for review
- `documentation` - Includes documentation

---

## 👥 Reviewers

Request review from:
- Project maintainers
- Anyone familiar with Rust collections
- Anyone familiar with algorithms

**Review focus areas:**
1. **Correctness** - Do implementations match textbook algorithms?
2. **Performance** - Are complexity guarantees met?
3. **API Design** - Are the APIs idiomatic Rust?
4. **Testing** - Is test coverage adequate?
5. **Documentation** - Are docs clear and complete?

---

## 💬 PR Comments

If reviewers request changes, address them with:

1. **Code changes:**
   ```bash
   # Make requested changes
   git add .
   git commit -m "Address review feedback: <description>"
   git push
   ```

2. **Documentation updates:**
   ```bash
   # Update docs
   cargo doc -p algs4-fundamentals --open
   git add .
   git commit -m "docs: Update documentation per review"
   git push
   ```

3. **Test additions:**
   ```bash
   # Add requested tests
   cargo test -p algs4-fundamentals
   git add .
   git commit -m "test: Add tests for <feature>"
   git push
   ```

The PR will automatically update with new commits.

---

## ✅ PR Approval Criteria

The PR should be approved when:

1. ✅ All CI checks pass (tests, clippy, formatting)
2. ✅ At least one maintainer approval
3. ✅ All review comments addressed
4. ✅ No merge conflicts with main
5. ✅ Documentation is clear and complete
6. ✅ Test coverage is adequate

---

## 🔀 Merging the PR

### After Approval

**Option 1: GitHub Web Interface**
1. Click "Squash and merge" (recommended) or "Merge pull request"
2. Confirm the merge
3. Delete the branch (optional but recommended)

**Option 2: Command Line**
```bash
# Update main branch
git checkout main
git pull origin main

# Merge the feature branch
git merge claude/start-phase-2-01KMbj82bXCU8zx6962j2nuR

# Push to main
git push origin main

# Delete feature branch (optional)
git branch -d claude/start-phase-2-01KMbj82bXCU8zx6962j2nuR
git push origin --delete claude/start-phase-2-01KMbj82bXCU8zx6962j2nuR
```

### After Merge

1. **Verify merge:**
   ```bash
   git checkout main
   git pull origin main
   git log --oneline -3
   ```

2. **Run tests on main:**
   ```bash
   cargo test --workspace
   ```

3. **Update local branches:**
   ```bash
   git fetch --prune
   git branch -d claude/start-phase-2-01KMbj82bXCU8zx6962j2nuR
   ```

---

## 🎯 Post-Merge Actions

After successful merge:

1. ✅ Close related issues (if any)
2. ✅ Update project board (if using one)
3. ✅ Announce completion in team chat
4. ✅ Archive this session summary
5. ✅ Start planning next phase

---

## 🆘 Troubleshooting

### PR Shows Merge Conflicts

```bash
# Update your branch with latest main
git checkout claude/start-phase-2-01KMbj82bXCU8zx6962j2nuR
git fetch origin main
git merge origin/main

# Resolve conflicts (if any)
# Edit conflicting files, then:
git add .
git commit -m "Merge main and resolve conflicts"
git push
```

### CI Checks Failing

```bash
# Run checks locally
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check

# Fix issues and push
git add .
git commit -m "fix: Address CI failures"
git push
```

### Need to Update PR Description

1. Go to PR page on GitHub
2. Click "..." next to the title
3. Click "Edit"
4. Update description
5. Click "Save"

---

## 📞 Need Help?

- Check `SESSION_PHASE2_SUMMARY.md` for detailed context
- Review `CONVERSION_PLAN.md` for overall strategy
- Check existing code for patterns and examples
- Consult the textbook: https://algs4.cs.princeton.edu/

---

**End of PR Creation Instructions**

*Phase 2 is complete and ready for review! 🚀*
