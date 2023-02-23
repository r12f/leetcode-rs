//!
//! Problem #1584: Min Cost to Connect All Points
//!
//! - Link: <https://leetcode.com/problems/min-cost-to-connect-all-points/>
//! - Discussions: <https://leetcode.com/problems/min-cost-to-connect-all-points/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! You are given an array `points` representing integer coordinates of some points on a 2D-plane, where `points[i] = [x<sub>i</sub>, y<sub>i</sub>]`.
//!
//! The cost of connecting two points `[x<sub>i</sub>, y<sub>i</sub>]` and `[x<sub>j</sub>, y<sub>j</sub>]` is the **manhattan distance** between them: `|x<sub>i</sub> - x<sub>j</sub>| + |y<sub>i</sub> - y<sub>j</sub>|`, where `|val|` denotes the absolute value of `val`.
//!
//! Return *the minimum cost to make all points connected.* All points are connected if there is **exactly one** simple path between any two points.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/08/26/d.png)
//!
//! ```
//! Input: points = [[0,0],[2,2],[3,10],[5,2],[7,0]]
//! Output: 20
//! Explanation:
//!
//! We can connect the points as shown above to get the minimum cost of 20.
//! Notice that there is a unique path between every pair of points.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: points = [[3,12],[-2,5],[-4,1]]
//! Output: 18
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= points.length <= 1000`
//! * `-10<sup>6</sup> <= x<sub>i</sub>, y<sub>i</sub> <= 10<sup>6</sup>`
//! * All pairs `(x<sub>i</sub>, y<sub>i</sub>)` are distinct.
//!

use std::{cmp::Ordering, collections::BinaryHeap, num};

#[derive(PartialEq, Eq)]
struct Edge {
    cost: i32,
    dst: usize,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[solution]
impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut cost = 0;

        let mut visited = vec![false; points.len()];
        visited[0] = true;

        let mut visited_node_count = 1;

        let mut queue = BinaryHeap::<Edge>::new();
        Solution::push_connected_nodes(&mut queue, &points, 0, &visited);

        while let Some(edge) = queue.pop() {
            if visited[edge.dst] {
                continue;
            }
            visited[edge.dst] = true;
            visited_node_count += 1;

            cost += edge.cost;

            if visited_node_count == points.len() {
                break;
            }

            Solution::push_connected_nodes(&mut queue, &points, edge.dst, &visited);
        }

        cost
    }

    fn push_connected_nodes(
        queue: &mut BinaryHeap<Edge>,
        points: &Vec<Vec<i32>>,
        start_point_index: usize,
        visited: &Vec<bool>,
    ) {
        let start_point = &points[start_point_index];
        for i in 0..points.len() {
            if visited[i] {
                continue;
            }

            let dst_point = &points[i];

            queue.push(Edge {
                cost: (dst_point[1] - start_point[1]).abs() + (dst_point[0] - start_point[0]).abs(),
                dst: i,
            })
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
    fn test_1584() {
        assert_eq!(
            20,
            Solution::min_cost_connect_points(lc_vecvec![[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]])
        );
    }
}
