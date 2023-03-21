//!
//! Problem #323: Number of Connected Components in an Undirected Graph
//!
//! - Link: <https://leetcode.com/problems/number-of-connected-components-in-an-undirected-graph/>
//! - Discussions: <https://leetcode.com/problems/number-of-connected-components-in-an-undirected-graph/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! You have a graph of `n` nodes. You are given an integer `n` and an array `edges` where `edges[i] = [a<sub>i</sub>, b<sub>i</sub>]` indicates that there is an edge between `a<sub>i</sub>` and `b<sub>i</sub>` in the graph.
//!
//! Return *the number of connected components in the graph*.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/03/14/conn1-graph.jpg)
//!
//! ```
//! Input: n = 5, edges = [[0,1],[1,2],[3,4]]
//! Output: 2
//!
//! ```
//!
//! **Example 2:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/03/14/conn2-graph.jpg)
//!
//! ```
//! Input: n = 5, edges = [[0,1],[1,2],[2,3],[3,4]]
//! Output: 1
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= n <= 2000`
//! * `1 <= edges.length <= 5000`
//! * `edges[i].length == 2`
//! * `0 <= a<sub>i</sub> <= b<sub>i</sub> < n`
//! * `a<sub>i</sub> != b<sub>i</sub>`
//! * There are no repeated edges.
//!

#[solution]
impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut graph = vec![Vec::<usize>::new(); n as usize];
        for edge in edges {
            graph[edge[0] as usize].push(edge[1] as usize);
            graph[edge[1] as usize].push(edge[0] as usize);
        }

        let mut count = 0;
        let mut visited = vec![false; n as usize];
        for i in 0..(n as usize) {
            if visited[i] {
                continue;
            }

            count += 1;
            Self::dfs(&graph, i, &mut visited);
        }

        count
    }

    fn dfs(graph: &Vec<Vec<usize>>, node: usize, visited: &mut Vec<bool>) {
        if visited[node] {
            return;
        }
        visited[node] = true;

        for connected_node in graph[node].iter() {
            Self::dfs(graph, *connected_node, visited);
        }
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
    fn test_323() {
        assert_eq!(
            2,
            Solution::count_components(5, lc_vecvec![[0, 1], [1, 2], [3, 4]])
        );

        assert_eq!(
            1,
            Solution::count_components(5, lc_vecvec![[0, 1], [1, 2], [2, 3], [3, 4]])
        );
    }
}
