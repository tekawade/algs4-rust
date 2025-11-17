//! Nondeterministic Finite Automaton (NFA) for regular expressions.
//!
//! Implements a simple regular expression matcher using NFA simulation.
//! Supports: concatenation, alternation (|), closure (*), and parentheses.

use std::collections::HashSet;

/// NFA for regular expression matching.
///
/// Supports the following operators:
/// - Concatenation: `AB` matches A followed by B
/// - Alternation: `A|B` matches either A or B
/// - Closure: `A*` matches zero or more occurrences of A
/// - Parentheses: `(AB)` for grouping
///
/// # Examples
///
/// ```
/// use algs4_strings::regex::NFA;
///
/// // Match pattern with alternation
/// let nfa = NFA::new("(A*B|AC)D");
/// assert!(nfa.recognizes("AABD"));
/// assert!(nfa.recognizes("ACD"));
/// // Note: ABD also matches since A* can be empty
/// // assert!(!nfa.recognizes("ABD"));
/// ```
#[derive(Clone, Debug)]
pub struct NFA {
    regex: Vec<char>,       // Regular expression
    graph: Vec<Vec<usize>>, // Epsilon transitions (digraph)
    m: usize,               // Number of states
}

impl NFA {
    /// Creates a new NFA from the given regular expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::regex::NFA;
    ///
    /// let nfa = NFA::new("(A*B|AC)D");
    /// assert!(nfa.recognizes("AABD"));
    /// ```
    pub fn new(regexp: &str) -> Self {
        let regex: Vec<char> = regexp.chars().collect();
        let m = regex.len();
        let mut graph = vec![Vec::new(); m + 1];
        let mut ops = Vec::new(); // Operator stack

        for i in 0..m {
            let mut lp = i;

            if regex[i] == '(' || regex[i] == '|' {
                ops.push(i);
            } else if regex[i] == ')' {
                let or = ops.pop().unwrap();
                if regex[or] == '|' {
                    lp = ops.pop().unwrap();
                    graph[lp].push(or + 1);
                    graph[or].push(i);
                } else {
                    lp = or;
                }
            }

            // Closure operator
            if i < m - 1 && regex[i + 1] == '*' {
                graph[lp].push(i + 1);
                graph[i + 1].push(lp);
            }

            // Add epsilon transitions for metasymbols and characters after alternation
            if regex[i] == '(' || regex[i] == '*' || regex[i] == ')' {
                graph[i].push(i + 1);
            }
        }

        // Add final epsilon transition to accept state
        graph[m].push(m);

        NFA { regex, graph, m }
    }

    /// Tests whether the NFA recognizes the given text.
    ///
    /// # Examples
    ///
    /// ```
    /// use algs4_strings::regex::NFA;
    ///
    /// let nfa = NFA::new("(A*B|AC)D");
    /// assert!(nfa.recognizes("AABD"));
    /// assert!(nfa.recognizes("ACD"));
    /// assert!(!nfa.recognizes("ABCD"));
    /// ```
    pub fn recognizes(&self, text: &str) -> bool {
        // States reachable from start by epsilon transitions
        let mut pc = HashSet::new();
        self.dfs(&self.graph, 0, &mut pc);

        // Simulate NFA on text
        for ch in text.chars() {
            let mut match_states = HashSet::new();

            // For each state in pc, if we can match ch, add next state
            for &v in &pc {
                if v < self.m && (self.regex[v] == ch || self.regex[v] == '.') {
                    match_states.insert(v + 1);
                }
            }

            // Find states reachable by epsilon transitions
            pc.clear();
            for &v in &match_states {
                self.dfs(&self.graph, v, &mut pc);
            }

            // If no states, pattern doesn't match
            if pc.is_empty() {
                return false;
            }
        }

        // Check if we can reach accept state (m) from any state in pc
        for &v in &pc {
            if v == self.m {
                return true;
            }
        }
        false
    }

    /// Depth-first search to find states reachable via epsilon transitions.
    #[allow(clippy::only_used_in_recursion)]
    fn dfs(&self, graph: &[Vec<usize>], v: usize, visited: &mut HashSet<usize>) {
        if visited.contains(&v) {
            return;
        }
        visited.insert(v);
        for &w in &graph[v] {
            self.dfs(graph, w, visited);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nfa_simple() {
        let nfa = NFA::new("AB");
        assert!(nfa.recognizes("AB"));
        assert!(!nfa.recognizes("A"));
        assert!(!nfa.recognizes("ABC"));
    }

    #[test]
    fn test_nfa_alternation() {
        // Note: Alternation requires parentheses in this NFA implementation
        let nfa = NFA::new("(A|B)");
        assert!(nfa.recognizes("A"));
        assert!(nfa.recognizes("B"));
        assert!(!nfa.recognizes("AB"));
        assert!(!nfa.recognizes("C"));
    }

    #[test]
    fn test_nfa_closure() {
        let nfa = NFA::new("A*");
        assert!(nfa.recognizes(""));
        assert!(nfa.recognizes("A"));
        assert!(nfa.recognizes("AA"));
        assert!(nfa.recognizes("AAA"));
        assert!(!nfa.recognizes("AB"));
    }

    #[test]
    fn test_nfa_closure_with_char() {
        let nfa = NFA::new("A*B");
        assert!(nfa.recognizes("B"));
        assert!(nfa.recognizes("AB"));
        assert!(nfa.recognizes("AAB"));
        assert!(nfa.recognizes("AAAB"));
        assert!(!nfa.recognizes("A"));
    }

    #[test]
    fn test_nfa_complex() {
        let nfa = NFA::new("(A*B|AC)D");
        assert!(nfa.recognizes("BD"));
        assert!(nfa.recognizes("ABD"));
        assert!(nfa.recognizes("AABD"));
        assert!(nfa.recognizes("ACD"));
        assert!(!nfa.recognizes("AD"));
        assert!(!nfa.recognizes("ABCD"));
    }

    #[test]
    fn test_nfa_parentheses() {
        let nfa = NFA::new("(AB)*");
        assert!(nfa.recognizes(""));
        assert!(nfa.recognizes("AB"));
        assert!(nfa.recognizes("ABAB"));
        assert!(!nfa.recognizes("A"));
        assert!(!nfa.recognizes("ABA"));
    }

    #[test]
    fn test_nfa_wildcard() {
        let nfa = NFA::new("A.B");
        assert!(nfa.recognizes("AXB"));
        assert!(nfa.recognizes("A1B"));
        assert!(!nfa.recognizes("AB"));
    }

    #[test]
    fn test_nfa_empty() {
        let nfa = NFA::new("");
        assert!(nfa.recognizes(""));
        assert!(!nfa.recognizes("A"));
    }

    #[test]
    fn test_nfa_textbook_example() {
        // Example from the textbook
        let nfa = NFA::new("((A*B|AC)D)");
        assert!(nfa.recognizes("AABD"));
        assert!(nfa.recognizes("ACD"));
        // Note: ABD should also match since A* can match empty string
        // assert!(!nfa.recognizes("ABD"));
    }
}
