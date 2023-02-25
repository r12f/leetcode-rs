//!
//! Problem #310: Minimum Height Trees
//!
//! - Link: <https://leetcode.com/problems/minimum-height-trees/>
//! - Discussions: <https://leetcode.com/problems/minimum-height-trees/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! A tree is an undirected graph in which any two vertices are connected by *exactly* one path. In other words, any connected graph without simple cycles is a tree.
//!
//! Given a tree of `n` nodes labelled from `0` to `n - 1`, and an array of `n - 1` `edges` where `edges[i] = [a<sub>i</sub>, b<sub>i</sub>]` indicates that there is an undirected edge between the two nodes `a<sub>i</sub>` and `b<sub>i</sub>` in the tree, you can choose any node of the tree as the root. When you select a node `x` as the root, the result tree has height `h`. Among all possible rooted trees, those with minimum height (i.e. `min(h)`) are called **minimum height trees** (MHTs).
//!
//! Return *a list of all **MHTs'** root labels*. You can return the answer in **any order**.
//!
//! The **height** of a rooted tree is the number of edges on the longest downward path between the root and a leaf.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/09/01/e1.jpg)
//!
//! ```
//! Input: n = 4, edges = [[1,0],[1,2],[1,3]]
//! Output: [1]
//! Explanation: As shown, the height of the tree is 1 when the root is the node with label 1 which is the only MHT.
//!
//! ```
//!
//! **Example 2:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/09/01/e2.jpg)
//!
//! ```
//! Input: n = 6, edges = [[3,0],[3,1],[3,2],[3,4],[5,4]]
//! Output: [3,4]
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= n <= 2 * 10<sup>4</sup>`
//! * `edges.length == n - 1`
//! * `0 <= a<sub>i</sub>, b<sub>i</sub> < n`
//! * `a<sub>i</sub> != b<sub>i</sub>`
//! * All the pairs `(a<sub>i</sub>, b<sub>i</sub>)` are distinct.
//! * The given input is **guaranteed** to be a tree and there will be **no repeated** edges.
//!

use std::collections::{HashMap, HashSet, VecDeque};

// BFS, timeout.
//
// ```rust
// #[solution]
// impl Solution {
//     pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
//         let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
//         for edge in edges {
//             graph
//                 .entry(edge[0] as usize)
//                 .or_default()
//                 .push(edge[1] as usize);
//             graph
//                 .entry(edge[1] as usize)
//                 .or_default()
//                 .push(edge[0] as usize);
//         }
//
//         let mut min_height = i32::MAX;
//         let mut mht_roots: Vec<i32> = Vec::new();
//         for root in 0..n {
//             let height =
//                 Solution::get_min_tree_height(root as usize, n as usize, &graph, min_height);
//             if height < min_height {
//                 min_height = height;
//                 mht_roots.clear();
//             }
//
//             if height == min_height {
//                 mht_roots.push(root);
//             }
//         }
//
//         mht_roots
//     }
//
//     fn get_min_tree_height(
//         root: usize,
//         n: usize,
//         graph: &HashMap<usize, Vec<usize>>,
//         height_limit: i32,
//     ) -> i32 {
//         let mut visited = vec![false; n];
//         let mut max_height = 0;
//         let mut queue = VecDeque::new();
//         queue.push_back((root, 0));
//         visited[root] = true;
//
//         while let Some((node, height)) = queue.pop_front() {
//             if height > max_height {
//                 max_height = height;
//             }
//
//             if let Some(dest_nodes) = graph.get(&node) {
//                 for dest_node in dest_nodes {
//                     if !visited[*dest_node] {
//                         if height + 1 > height_limit {
//                             return i32::MAX;
//                         }
//                         queue.push_back((*dest_node, height + 1));
//                         visited[*dest_node] = true;
//                     }
//                 }
//             }
//         }
//
//         max_height
//     }
// }
// ```

#[solution]
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph: Vec<HashSet<usize>> = vec![HashSet::new(); n as usize];
        for edge in edges {
            graph[edge[0] as usize].insert(edge[1] as usize);
            graph[edge[1] as usize].insert(edge[0] as usize);
        }

        // Add all leaf nodes
        let mut queue = VecDeque::new();

        let mut remaining_nodes = n;
        for (node, dst_nodes) in graph.iter().enumerate() {
            if dst_nodes.len() <= 1 {
                queue.push_back(node);
            }
        }

        // Iterating and removing all leaf nodes
        while remaining_nodes > 2 {
            let mut new_queue = VecDeque::new();

            while let Some(removing_node) = queue.pop_front() {
                remaining_nodes -= 1;

                let mut dst_nodes = HashSet::new();
                std::mem::swap(&mut dst_nodes, &mut graph[removing_node]);

                for dst_node in dst_nodes {
                    graph[dst_node].remove(&removing_node);

                    if graph[dst_node].len() == 1 {
                        new_queue.push_back(dst_node);
                    }
                }
            }

            queue = new_queue;
        }

        queue.into_iter().map(|v| v as i32).collect::<Vec<i32>>()
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
    fn test_310() {
        assert_eq!(
            vec![1],
            Solution::find_min_height_trees(4, lc_vecvec![[1, 0], [1, 2], [1, 3]])
        );
    }
}
