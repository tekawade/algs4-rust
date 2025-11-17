/// DeDup - Remove Duplicates
///
/// A utility for removing duplicate elements from a sequence using a SET.
/// Demonstrates practical usage of set implementations for deduplication.
///
/// # Examples
///
/// ```
/// use searching::dedup::remove_duplicates;
///
/// let items = vec!["apple", "banana", "apple", "cherry", "banana"];
/// let unique = remove_duplicates(&items);
///
/// assert_eq!(unique.len(), 3);
/// assert!(unique.contains(&"apple"));
/// assert!(unique.contains(&"banana"));
/// assert!(unique.contains(&"cherry"));
/// ```
use std::collections::HashSet;
use std::hash::Hash;

/// Removes duplicate elements from a slice, preserving first occurrence order.
///
/// # Arguments
///
/// * `items` - Slice of items to deduplicate
///
/// # Returns
///
/// A Vec containing unique elements in order of first occurrence.
///
/// # Examples
///
/// ```
/// use searching::dedup::remove_duplicates;
///
/// let items = vec![1, 2, 3, 2, 4, 1, 5];
/// let unique = remove_duplicates(&items);
///
/// assert_eq!(unique, vec![1, 2, 3, 4, 5]);
/// ```
pub fn remove_duplicates<T: Eq + Hash + Clone>(items: &[T]) -> Vec<T> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for item in items {
        if seen.insert(item.clone()) {
            result.push(item.clone());
        }
    }

    result
}

/// Removes duplicate elements and returns them as a HashSet.
///
/// # Arguments
///
/// * `items` - Slice of items to deduplicate
///
/// # Returns
///
/// A HashSet containing all unique elements.
///
/// # Examples
///
/// ```
/// use searching::dedup::unique_set;
///
/// let items = vec!["a", "b", "a", "c", "b"];
/// let unique = unique_set(&items);
///
/// assert_eq!(unique.len(), 3);
/// assert!(unique.contains("a"));
/// assert!(unique.contains("b"));
/// assert!(unique.contains("c"));
/// ```
pub fn unique_set<T: Eq + Hash + Clone>(items: &[T]) -> HashSet<T> {
    items.iter().cloned().collect()
}

/// Counts the number of duplicate occurrences for each item.
///
/// # Arguments
///
/// * `items` - Slice of items to analyze
///
/// # Returns
///
/// A Vec of tuples (item, duplicate_count) for items that appear more than once.
///
/// # Examples
///
/// ```
/// use searching::dedup::count_duplicates;
///
/// let items = vec!["a", "b", "a", "c", "a", "b"];
/// let dups = count_duplicates(&items);
///
/// // "a" appears 3 times (2 duplicates), "b" appears 2 times (1 duplicate)
/// assert_eq!(dups.len(), 2);
/// ```
pub fn count_duplicates<T: Eq + Hash + Clone>(items: &[T]) -> Vec<(T, usize)> {
    use std::collections::HashMap;

    let mut counts = HashMap::new();
    for item in items {
        *counts.entry(item.clone()).or_insert(0) += 1;
    }

    counts
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .map(|(item, count)| (item, count - 1))
        .collect()
}

/// Finds all duplicate elements (elements that appear more than once).
///
/// # Arguments
///
/// * `items` - Slice of items to check
///
/// # Returns
///
/// A HashSet containing elements that appear more than once.
///
/// # Examples
///
/// ```
/// use searching::dedup::find_duplicates;
///
/// let items = vec![1, 2, 3, 2, 4, 3, 5];
/// let dups = find_duplicates(&items);
///
/// assert_eq!(dups.len(), 2);
/// assert!(dups.contains(&2));
/// assert!(dups.contains(&3));
/// ```
pub fn find_duplicates<T: Eq + Hash + Clone>(items: &[T]) -> HashSet<T> {
    use std::collections::HashMap;

    let mut counts = HashMap::new();
    for item in items {
        *counts.entry(item.clone()).or_insert(0) += 1;
    }

    counts
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .map(|(item, _)| item)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let items = vec![1, 2, 3, 2, 4, 1, 5];
        let unique = remove_duplicates(&items);

        assert_eq!(unique, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_remove_duplicates_strings() {
        let items = vec!["apple", "banana", "apple", "cherry", "banana"];
        let unique = remove_duplicates(&items);

        assert_eq!(unique.len(), 3);
        assert_eq!(unique[0], "apple");
        assert_eq!(unique[1], "banana");
        assert_eq!(unique[2], "cherry");
    }

    #[test]
    fn test_remove_duplicates_empty() {
        let items: Vec<i32> = vec![];
        let unique = remove_duplicates(&items);

        assert!(unique.is_empty());
    }

    #[test]
    fn test_remove_duplicates_no_duplicates() {
        let items = vec![1, 2, 3, 4, 5];
        let unique = remove_duplicates(&items);

        assert_eq!(unique, items);
    }

    #[test]
    fn test_unique_set() {
        let items = vec!["a", "b", "a", "c", "b"];
        let unique = unique_set(&items);

        assert_eq!(unique.len(), 3);
        assert!(unique.contains("a"));
        assert!(unique.contains("b"));
        assert!(unique.contains("c"));
    }

    #[test]
    fn test_count_duplicates() {
        let items = vec!["a", "b", "a", "c", "a", "b"];
        let dups = count_duplicates(&items);

        assert_eq!(dups.len(), 2);

        // Find the counts for "a" and "b"
        let a_dups = dups
            .iter()
            .find(|(item, _)| item == &"a")
            .map(|(_, count)| count);
        let b_dups = dups
            .iter()
            .find(|(item, _)| item == &"b")
            .map(|(_, count)| count);

        assert_eq!(a_dups, Some(&2)); // "a" has 2 duplicates (appears 3 times)
        assert_eq!(b_dups, Some(&1)); // "b" has 1 duplicate (appears 2 times)
    }

    #[test]
    fn test_count_duplicates_no_duplicates() {
        let items = vec![1, 2, 3, 4, 5];
        let dups = count_duplicates(&items);

        assert!(dups.is_empty());
    }

    #[test]
    fn test_find_duplicates() {
        let items = vec![1, 2, 3, 2, 4, 3, 5];
        let dups = find_duplicates(&items);

        assert_eq!(dups.len(), 2);
        assert!(dups.contains(&2));
        assert!(dups.contains(&3));
    }

    #[test]
    fn test_find_duplicates_none() {
        let items = vec![1, 2, 3, 4, 5];
        let dups = find_duplicates(&items);

        assert!(dups.is_empty());
    }

    #[test]
    fn test_find_duplicates_all() {
        let items = vec![1, 1, 2, 2, 3, 3];
        let dups = find_duplicates(&items);

        assert_eq!(dups.len(), 3);
        assert!(dups.contains(&1));
        assert!(dups.contains(&2));
        assert!(dups.contains(&3));
    }
}
