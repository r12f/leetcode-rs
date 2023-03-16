//!
//! Problem #785: Is Graph Bipartite?
//!
//! - Link: <https://leetcode.com/problems/is-graph-bipartite/>
//! - Discussions: <https://leetcode.com/problems/is-graph-bipartite/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! There is an **undirected** graph with `n` nodes, where each node is numbered between `0` and `n - 1`. You are given a 2D array `graph`, where `graph[u]` is an array of nodes that node `u` is adjacent to. More formally, for each `v` in `graph[u]`, there is an undirected edge between node `u` and node `v`. The graph has the following properties:
//!
//! * There are no self-edges (`graph[u]` does not contain `u`).
//! * There are no parallel edges (`graph[u]` does not contain duplicate values).
//! * If `v` is in `graph[u]`, then `u` is in `graph[v]` (the graph is undirected).
//! * The graph may not be connected, meaning there may be two nodes `u` and `v` such that there is no path between them.
//!
//! A graph is **bipartite** if the nodes can be partitioned into two independent sets `A` and `B` such that **every** edge in the graph connects a node in set `A` and a node in set `B`.
//!
//! Return `true` *if and only if it is **bipartite***.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/10/21/bi2.jpg)
//!
//! ```
//! Input: graph = [[1,2,3],[0,2],[0,1,3],[0,2]]
//! Output: false
//! Explanation: There is no way to partition the nodes into two independent sets such that every edge connects a node in one and a node in the other.
//! ```
//!
//! **Example 2:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/10/21/bi1.jpg)
//!
//! ```
//! Input: graph = [[1,3],[0,2],[1,3],[0,2]]
//! Output: true
//! Explanation: We can partition the nodes into two sets: {0, 2} and {1, 3}.
//! ```
//!
//! **Constraints:**
//!
//! * `graph.length == n`
//! * `1 <= n <= 100`
//! * `0 <= graph[u].length < n`
//! * `0 <= graph[u][i] <= n - 1`
//! * `graph[u]` does not contain `u`.
//! * All the values of `graph[u]` are **unique**.
//! * If `graph[u]` contains `v`, then `graph[v]` contains `u`.
//!

#[solution]
impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut node_type_map: Vec<Option<i32>> = vec![None; graph.len()];

        for i in 0..graph.len() {
            let start_node_type = if let Some(node_type) = node_type_map[i] {
                node_type
            } else {
                0
            };

            if !Self::dfs(&graph, i, start_node_type, &mut node_type_map) {
                return false;
            }
        }

        true
    }

    fn dfs(
        graph: &Vec<Vec<i32>>,
        i: usize,
        node_type: i32,
        node_type_map: &mut Vec<Option<i32>>,
    ) -> bool {
        if node_type_map[i].is_some() {
            if *node_type_map[i].as_ref().unwrap() == node_type {
                return true;
            } else {
                return false;
            }
        } else {
            node_type_map[i] = Some(node_type);
        }

        let next_node_type = if node_type == 0 { 1 } else { 0 };
        for next_node in graph[i].iter() {
            if !Self::dfs(graph, *next_node as usize, next_node_type, node_type_map) {
                return false;
            }
        }

        true
    }
}

//
// Tests
//
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_785() {}
}
