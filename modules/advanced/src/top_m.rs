//! Provides a `TopM` data structure to find the M largest items from a stream.
//!
//! It uses a min-priority queue to efficiently keep track of the M largest items
//! seen so far. This can be used to build an application that reads items from
//! standard input (like a sequence of transactions) and prints the top M.

/// Finds the top M items from a stream using a min priority queue.
///
/// This implementation maintains a min-heap of size at most M+1. When a new item
/// arrives, it's inserted into the heap. If the heap size exceeds M, the minimum
/// is removed. This ensures that only the M largest items remain.
///
/// # Time Complexity
///
/// - Per item: O(log M)
/// - Total for N items: O(N log M)
///
/// # Examples
///
/// ```
/// use algs4_advanced::top_m::TopM;
///
/// let mut top = TopM::new(3); // Keep top 3 items
/// top.insert(10);
/// top.insert(5);
/// top.insert(20);
/// top.insert(15);
/// top.insert(8);
///
/// let result = top.into_sorted_vec();
/// assert_eq!(result, vec![20, 15, 10]); // Top 3 in descending order
/// ```
#[derive(Debug, Clone)]
pub struct TopM<T> {
    m: usize,
    pq: Vec<T>,
}

impl<T: Ord + Clone> TopM<T> {
    /// Creates a new TopM finder for the top M items.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::top_m::TopM;
    ///
    /// let top: TopM<i32> = TopM::new(5);
    /// assert_eq!(top.size(), 0);
    /// ```
    pub fn new(m: usize) -> Self {
        TopM {
            m,
            pq: Vec::with_capacity(m + 1),
        }
    }

    /// Inserts an item into the stream.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::top_m::TopM;
    ///
    /// let mut top = TopM::new(2);
    /// top.insert(10);
    /// top.insert(20);
    /// top.insert(5); // This will be discarded as it's not in top 2
    /// ```
    pub fn insert(&mut self, item: T) {
        self.pq.push(item);
        self.swim(self.pq.len() - 1);

        // If we exceed m items, remove the minimum
        if self.pq.len() > self.m {
            self.del_min();
        }
    }

    /// Returns the number of items currently tracked.
    pub fn size(&self) -> usize {
        self.pq.len()
    }

    /// Returns true if no items are being tracked.
    pub fn is_empty(&self) -> bool {
        self.pq.is_empty()
    }

    /// Consumes self and returns the top M items in descending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_advanced::top_m::TopM;
    ///
    /// let mut top = TopM::new(3);
    /// top.insert(5);
    /// top.insert(2);
    /// top.insert(8);
    /// top.insert(1);
    /// top.insert(9);
    ///
    /// let result = top.into_sorted_vec();
    /// assert_eq!(result, vec![9, 8, 5]);
    /// ```
    pub fn into_sorted_vec(mut self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.pq.len());

        // Extract all items (they come out in min-heap order)
        while !self.is_empty() {
            result.push(self.del_min());
        }

        // Reverse to get descending order (largest first)
        result.reverse();
        result
    }

    // Helper: swim up to maintain heap invariant
    fn swim(&mut self, mut k: usize) {
        while k > 0 {
            let parent = (k - 1) / 2;
            if self.pq[k] >= self.pq[parent] {
                break;
            }
            self.pq.swap(k, parent);
            k = parent;
        }
    }

    // Helper: sink down to maintain heap invariant
    fn sink(&mut self, mut k: usize) {
        let n = self.pq.len();
        while 2 * k + 1 < n {
            let mut j = 2 * k + 1;
            if j + 1 < n && self.pq[j + 1] < self.pq[j] {
                j += 1;
            }
            if self.pq[k] <= self.pq[j] {
                break;
            }
            self.pq.swap(k, j);
            k = j;
        }
    }

    // Helper: delete minimum element
    fn del_min(&mut self) -> T {
        let n = self.pq.len();
        assert!(n > 0, "Priority queue underflow");

        self.pq.swap(0, n - 1);
        let min = self.pq.pop().unwrap();

        if !self.pq.is_empty() {
            self.sink(0);
        }

        min
    }
}

/// Helper function to find top M items from an iterator.
///
/// # Examples
///
/// ```
/// use algs4_advanced::top_m::top_m;
///
/// let numbers = vec![5, 2, 8, 1, 9, 3, 7];
/// let top3 = top_m(numbers, 3);
/// assert_eq!(top3, vec![9, 8, 7]);
/// ```
pub fn top_m<T, I>(items: I, m: usize) -> Vec<T>
where
    T: Ord + Clone,
    I: IntoIterator<Item = T>,
{
    let mut finder = TopM::new(m);
    for item in items {
        finder.insert(item);
    }
    finder.into_sorted_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut top = TopM::new(3);
        top.insert(10);
        top.insert(5);
        top.insert(20);
        top.insert(15);
        top.insert(8);

        let result = top.into_sorted_vec();
        assert_eq!(result, vec![20, 15, 10]);
    }

    #[test]
    fn test_exact_m() {
        let mut top = TopM::new(3);
        top.insert(1);
        top.insert(2);
        top.insert(3);

        let result = top.into_sorted_vec();
        assert_eq!(result, vec![3, 2, 1]);
    }

    #[test]
    fn test_fewer_than_m() {
        let mut top = TopM::new(5);
        top.insert(3);
        top.insert(1);
        top.insert(2);

        let result = top.into_sorted_vec();
        assert_eq!(result, vec![3, 2, 1]);
    }

    #[test]
    fn test_single_item() {
        let mut top = TopM::new(1);
        top.insert(5);
        top.insert(10);
        top.insert(3);

        let result = top.into_sorted_vec();
        assert_eq!(result, vec![10]);
    }

    #[test]
    fn test_duplicates() {
        let mut top = TopM::new(3);
        top.insert(5);
        top.insert(5);
        top.insert(5);
        top.insert(10);
        top.insert(10);

        let result = top.into_sorted_vec();
        assert_eq!(result, vec![10, 10, 5]);
    }

    #[test]
    fn test_helper_function() {
        let numbers = vec![5, 2, 8, 1, 9, 3, 7];
        let top3 = top_m(numbers, 3);
        assert_eq!(top3, vec![9, 8, 7]);
    }

    #[test]
    fn test_strings() {
        let mut top = TopM::new(3);
        top.insert("apple".to_string());
        top.insert("zebra".to_string());
        top.insert("banana".to_string());
        top.insert("orange".to_string());

        let result = top.into_sorted_vec();
        assert_eq!(result, vec!["zebra", "orange", "banana"]);
    }

    #[test]
    fn test_empty() {
        let top: TopM<i32> = TopM::new(5);
        assert!(top.is_empty());
        assert_eq!(top.size(), 0);
    }
}
