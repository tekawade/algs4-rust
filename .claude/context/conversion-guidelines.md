# Java to Rust Conversion Guidelines

## Memory Management

### Null Handling
**Java:**
```java
Node<Item> first = null;
if (first != null) { ... }
```

**Rust:**
```rust
let first: Option<Box<Node<Item>>> = None;
if let Some(node) = first { ... }
```

### Linked Structures
**Java:**
```java
private class Node {
    Item item;
    Node next;
}
```

**Rust:**
```rust
struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}
```

### Arrays and Collections
**Java:**
```java
Item[] items = new Item[capacity];
```

**Rust:**
```rust
let items: Vec<T> = Vec::with_capacity(capacity);
// or
let items: Vec<Option<T>> = vec![None; capacity];
```

## Generics

### Generic Classes
**Java:**
```java
public class Stack<Item> {
    private Node<Item> first;
}
```

**Rust:**
```rust
pub struct Stack<T> {
    first: Option<Box<Node<T>>>,
}
```

### Generic Methods
**Java:**
```java
public static <Key extends Comparable<Key>> void sort(Key[] a) {
    // ...
}
```

**Rust:**
```rust
pub fn sort<T: Ord>(arr: &mut [T]) {
    // ...
}
```

## Iteration

### Iterable Collections
**Java:**
```java
public class Stack<Item> implements Iterable<Item> {
    public Iterator<Item> iterator() {
        return new ListIterator();
    }

    private class ListIterator implements Iterator<Item> {
        public boolean hasNext() { ... }
        public Item next() { ... }
    }
}
```

**Rust:**
```rust
pub struct Stack<T> {
    first: Option<Box<Node<T>>>,
}

pub struct StackIter<T> {
    current: Option<Box<Node<T>>>,
}

impl<T> Iterator for StackIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // ...
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = StackIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        StackIter { current: self.first }
    }
}
```

## Error Handling

### Exceptions to Results
**Java:**
```java
public Item pop() throws NoSuchElementException {
    if (isEmpty()) throw new NoSuchElementException();
    return first.item;
}
```

**Rust:**
```rust
pub fn pop(&mut self) -> Option<T> {
    self.first.take().map(|node| {
        self.first = node.next;
        node.item
    })
}
// or with Result:
pub fn pop(&mut self) -> Result<T, &'static str> {
    self.first.take()
        .map(|node| {
            self.first = node.next;
            node.item
        })
        .ok_or("Stack is empty")
}
```

### I/O Exceptions
**Java:**
```java
public static In readFile(String filename) throws IOException {
    // ...
}
```

**Rust:**
```rust
use std::io;

pub fn read_file(filename: &str) -> io::Result<In> {
    // ...
}
```

## Comparisons

### Comparable Interface
**Java:**
```java
public class Date implements Comparable<Date> {
    public int compareTo(Date that) {
        // return -1, 0, or 1
    }
}
```

**Rust:**
```rust
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Date {
    // fields
}

// or manual implementation:
impl Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // return Ordering::Less, Ordering::Equal, or Ordering::Greater
    }
}
```

### Comparator
**Java:**
```java
Comparator<String> byLength = (s1, s2) -> s1.length() - s2.length();
Arrays.sort(strings, byLength);
```

**Rust:**
```rust
strings.sort_by(|a, b| a.len().cmp(&b.len()));
// or:
strings.sort_by_key(|s| s.len());
```

## I/O Operations

### Reading Input
**Java:**
```java
StdIn.readInt();
StdIn.readDouble();
StdIn.readString();
```

**Rust:**
```rust
use std::io::{self, BufRead};

let stdin = io::stdin();
let mut line = String::new();
stdin.lock().read_line(&mut line)?;
let num: i32 = line.trim().parse()?;
```

### Printing Output
**Java:**
```java
StdOut.println("Hello");
StdOut.printf("Value: %.2f\n", x);
```

**Rust:**
```rust
println!("Hello");
println!("Value: {:.2}", x);
```

## Common Patterns

### Resizing Array
**Java:**
```java
private void resize(int capacity) {
    Item[] copy = (Item[]) new Object[capacity];
    for (int i = 0; i < n; i++)
        copy[i] = a[i];
    a = copy;
}
```

**Rust:**
```rust
// Use Vec which handles resizing automatically
let mut vec = Vec::new();
vec.push(item); // automatically resizes

// Or manual control:
vec.reserve(additional_capacity);
```

### Swapping
**Java:**
```java
private static void exch(Comparable[] a, int i, int j) {
    Comparable t = a[i];
    a[i] = a[j];
    a[j] = t;
}
```

**Rust:**
```rust
fn exch<T>(a: &mut [T], i: usize, j: usize) {
    a.swap(i, j);
}
```

### Copying Arrays
**Java:**
```java
Item[] aux = new Item[a.length];
for (int k = lo; k <= hi; k++)
    aux[k] = a[k];
```

**Rust:**
```rust
let mut aux = vec![None; a.len()];
aux[lo..=hi].copy_from_slice(&a[lo..=hi]);
// or:
let aux: Vec<_> = a[lo..=hi].to_vec();
```

## Testing

### Assertions
**Java:**
```java
assert isSorted(a);
```

**Rust:**
```rust
assert!(is_sorted(&a));

#[cfg(debug_assertions)]
{
    assert!(is_sorted(&a));
}
```

### Unit Tests
**Java:**
```java
// Usually in separate test files with JUnit
```

**Rust:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut stack = Stack::new();
        stack.push(1);
        assert_eq!(stack.pop(), Some(1));
    }
}
```

## Performance Considerations

### Inline Functions
For small, frequently-called functions:
```rust
#[inline]
pub fn is_empty(&self) -> bool {
    self.first.is_none()
}
```

### Avoiding Allocations
**Java:**
```java
// Creates new Integer objects
Integer a = 1;
Integer b = 2;
```

**Rust:**
```rust
// Primitive types, no allocation
let a: i32 = 1;
let b: i32 = 2;

// Use references to avoid moves
fn process(data: &[i32]) { ... }
```

## Common Pitfalls

1. **Index out of bounds**: Rust panics on invalid indices. Use `.get()` for safe access:
   ```rust
   if let Some(value) = arr.get(i) { ... }
   ```

2. **Integer division**: Same as Java, truncates toward zero:
   ```rust
   let result = 5 / 2; // 2, not 2.5
   ```

3. **Ownership and borrowing**: Understand when values are moved vs borrowed:
   ```rust
   let s1 = String::from("hello");
   let s2 = s1; // s1 is moved, can't use s1 anymore

   let s1 = String::from("hello");
   let s2 = &s1; // s1 is borrowed, can still use both
   ```

4. **Mutable references**: Only one mutable reference at a time:
   ```rust
   let mut v = vec![1, 2, 3];
   let r1 = &mut v; // OK
   // let r2 = &mut v; // Error: cannot borrow as mutable more than once
   ```

## Documentation

Include documentation comments with examples:

```rust
/// Returns the number of items in the stack.
///
/// # Examples
///
/// ```
/// let mut stack = Stack::new();
/// assert_eq!(stack.size(), 0);
/// stack.push(1);
/// assert_eq!(stack.size(), 1);
/// ```
///
/// # Time Complexity
///
/// O(1)
pub fn size(&self) -> usize {
    self.n
}
```
