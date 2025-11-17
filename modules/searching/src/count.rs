/// Count - Count Occurrences
///
/// A utility for counting occurrences of specific keys in a collection.
/// Demonstrates using symbol tables for occurrence counting.
///
/// # Examples
///
/// ```
/// use searching::count::count_key;
///
/// let items = vec!["apple", "banana", "apple", "cherry", "apple"];
/// let count = count_key(&items, &"apple");
///
/// assert_eq!(count, 3);
/// ```
use std::collections::HashMap;
use std::hash::Hash;

/// Counts the number of times a specific key appears in a slice.
///
/// # Arguments
///
/// * `items` - Slice of items to search
/// * `key` - The key to count
///
/// # Returns
///
/// The number of times the key appears.
///
/// # Examples
///
/// ```
/// use searching::count::count_key;
///
/// let items = vec![1, 2, 3, 2, 4, 2, 5];
/// assert_eq!(count_key(&items, &2), 3);
/// assert_eq!(count_key(&items, &6), 0);
/// ```
pub fn count_key<T: Eq>(items: &[T], key: &T) -> usize {
    items.iter().filter(|&item| item == key).count()
}

/// Counts all occurrences of each unique key in a slice.
///
/// # Arguments
///
/// * `items` - Slice of items to count
///
/// # Returns
///
/// A HashMap mapping each unique item to its occurrence count.
///
/// # Examples
///
/// ```
/// use searching::count::count_all;
///
/// let items = vec!["a", "b", "a", "c", "b", "a"];
/// let counts = count_all(&items);
///
/// assert_eq!(counts.get("a"), Some(&3));
/// assert_eq!(counts.get("b"), Some(&2));
/// assert_eq!(counts.get("c"), Some(&1));
/// ```
pub fn count_all<T: Eq + Hash + Clone>(items: &[T]) -> HashMap<T, usize> {
    let mut counts = HashMap::new();
    for item in items {
        *counts.entry(item.clone()).or_insert(0) += 1;
    }
    counts
}

/// Counts occurrences of multiple specific keys.
///
/// # Arguments
///
/// * `items` - Slice of items to search
/// * `keys` - Slice of keys to count
///
/// # Returns
///
/// A HashMap mapping each key to its occurrence count (0 if not found).
///
/// # Examples
///
/// ```
/// use searching::count::count_keys;
///
/// let items = vec!["a", "b", "a", "c", "b", "a"];
/// let keys = vec!["a", "b", "d"];
/// let counts = count_keys(&items, &keys);
///
/// assert_eq!(counts.get(&"a"), Some(&3));
/// assert_eq!(counts.get(&"b"), Some(&2));
/// assert_eq!(counts.get(&"d"), Some(&0));
/// ```
pub fn count_keys<T: Eq + Hash + Clone>(items: &[T], keys: &[T]) -> HashMap<T, usize> {
    let mut counts = HashMap::new();

    // Initialize all keys with count 0
    for key in keys {
        counts.insert(key.clone(), 0);
    }

    // Count occurrences
    for item in items {
        if let Some(count) = counts.get_mut(item) {
            *count += 1;
        }
    }

    counts
}

/// Finds the most common element in a slice.
///
/// # Arguments
///
/// * `items` - Slice of items to analyze
///
/// # Returns
///
/// An Option containing a tuple of (item, count) for the most common element,
/// or None if the slice is empty.
///
/// # Examples
///
/// ```
/// use searching::count::most_common;
///
/// let items = vec![1, 2, 3, 2, 4, 2, 5];
/// assert_eq!(most_common(&items), Some((2, 3)));
/// ```
pub fn most_common<T: Eq + Hash + Clone>(items: &[T]) -> Option<(T, usize)> {
    let counts = count_all(items);
    counts.into_iter().max_by_key(|(_, count)| *count)
}

/// Finds the least common element in a slice.
///
/// # Arguments
///
/// * `items` - Slice of items to analyze
///
/// # Returns
///
/// An Option containing a tuple of (item, count) for the least common element,
/// or None if the slice is empty.
///
/// # Examples
///
/// ```
/// use searching::count::least_common;
///
/// let items = vec![1, 2, 3, 2, 4, 2, 5];
/// let result = least_common(&items);
/// // 1, 3, 4, or 5 could be returned (all appear once)
/// assert_eq!(result.unwrap().1, 1);
/// ```
pub fn least_common<T: Eq + Hash + Clone>(items: &[T]) -> Option<(T, usize)> {
    let counts = count_all(items);
    counts.into_iter().min_by_key(|(_, count)| *count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_key() {
        let items = vec![1, 2, 3, 2, 4, 2, 5];
        assert_eq!(count_key(&items, &2), 3);
        assert_eq!(count_key(&items, &6), 0);
    }

    #[test]
    fn test_count_key_strings() {
        let items = vec!["apple", "banana", "apple", "cherry", "apple"];
        assert_eq!(count_key(&items, &"apple"), 3);
        assert_eq!(count_key(&items, &"banana"), 1);
        assert_eq!(count_key(&items, &"grape"), 0);
    }

    #[test]
    fn test_count_all() {
        let items = vec!["a", "b", "a", "c", "b", "a"];
        let counts = count_all(&items);

        assert_eq!(counts.get("a"), Some(&3));
        assert_eq!(counts.get("b"), Some(&2));
        assert_eq!(counts.get("c"), Some(&1));
        assert_eq!(counts.get("d"), None);
    }

    #[test]
    fn test_count_all_empty() {
        let items: Vec<i32> = vec![];
        let counts = count_all(&items);

        assert!(counts.is_empty());
    }

    #[test]
    fn test_count_keys() {
        let items = vec!["a", "b", "a", "c", "b", "a"];
        let keys = vec!["a", "b", "d"];
        let counts = count_keys(&items, &keys);

        assert_eq!(counts.get(&"a"), Some(&3));
        assert_eq!(counts.get(&"b"), Some(&2));
        assert_eq!(counts.get(&"d"), Some(&0));
    }

    #[test]
    fn test_most_common() {
        let items = vec![1, 2, 3, 2, 4, 2, 5];
        assert_eq!(most_common(&items), Some((2, 3)));
    }

    #[test]
    fn test_most_common_strings() {
        let items = vec!["apple", "banana", "apple", "cherry", "apple"];
        let result = most_common(&items);
        assert!(result.is_some());
        let (word, count) = result.unwrap();
        assert_eq!(word, &"apple".to_string());
        assert_eq!(count, 3);
    }

    #[test]
    fn test_most_common_empty() {
        let items: Vec<i32> = vec![];
        assert_eq!(most_common(&items), None);
    }

    #[test]
    fn test_least_common() {
        let items = vec![1, 2, 3, 2, 4, 2, 5];
        let result = least_common(&items);
        assert!(result.is_some());
        assert_eq!(result.unwrap().1, 1);
    }

    #[test]
    fn test_least_common_tie() {
        let items = vec![1, 1, 2, 2, 3, 3];
        let result = least_common(&items);
        assert!(result.is_some());
        assert_eq!(result.unwrap().1, 2);
    }
}
