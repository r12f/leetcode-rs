//!
//! Problem #787: Cheapest Flights Within K Stops
//!
//! - Link: <https://leetcode.com/problems/cheapest-flights-within-k-stops/>
//! - Discussions: <https://leetcode.com/problems/cheapest-flights-within-k-stops/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! There are `n` cities connected by some number of flights. You are given an array `flights` where `flights[i] = [from<sub>i</sub>, to<sub>i</sub>, price<sub>i</sub>]` indicates that there is a flight from city `from<sub>i</sub>` to city `to<sub>i</sub>` with cost `price<sub>i</sub>`.
//!
//! You are also given three integers `src`, `dst`, and `k`, return ***the cheapest price** from* `src` *to* `dst` *with at most* `k` *stops.* If there is no such route, return `-1`.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2022/03/18/cheapest-flights-within-k-stops-3drawio.png)
//!
//! ```
//! Input: n = 4, flights = [[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]], src = 0, dst = 3, k = 1
//! Output: 700
//! Explanation:
//! The graph is shown above.
//! The optimal path with at most 1 stop from city 0 to 3 is marked in red and has cost 100 + 600 = 700.
//! Note that the path through cities [0,1,2,3] is cheaper but is invalid because it uses 2 stops.
//!
//! ```
//!
//! **Example 2:**
//!
//! ![](https://assets.leetcode.com/uploads/2022/03/18/cheapest-flights-within-k-stops-1drawio.png)
//!
//! ```
//! Input: n = 3, flights = [[0,1,100],[1,2,100],[0,2,500]], src = 0, dst = 2, k = 1
//! Output: 200
//! Explanation:
//! The graph is shown above.
//! The optimal path with at most 1 stop from city 0 to 2 is marked in red and has cost 100 + 100 = 200.
//!
//! ```
//!
//! **Example 3:**
//!
//! ![](https://assets.leetcode.com/uploads/2022/03/18/cheapest-flights-within-k-stops-2drawio.png)
//!
//! ```
//! Input: n = 3, flights = [[0,1,100],[1,2,100],[0,2,500]], src = 0, dst = 2, k = 0
//! Output: 500
//! Explanation:
//! The graph is shown above.
//! The optimal path with no stops from city 0 to 2 is marked in red and has cost 500.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= n <= 100`
//! * `0 <= flights.length <= (n * (n - 1) / 2)`
//! * `flights[i].length == 3`
//! * `0 <= from<sub>i</sub>, to<sub>i</sub> < n`
//! * `from<sub>i</sub> != to<sub>i</sub>`
//! * `1 <= price<sub>i</sub> <= 10<sup>4</sup>`
//! * There will not be any multiple flights between two cities.
//! * `0 <= src, dst, k < n`
//! * `src != dst`
//!

use std::collections::{BinaryHeap, HashMap};

#[derive(PartialEq, Eq)]
struct SearchState {
    cost: i32,
    node: i32,
    step: i32,
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[solution]
impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut graph: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();
        for flight in flights {
            graph.entry(flight[0]).or_default().push(flight);
        }

        let mut steps = vec![i32::MAX; n as usize];
        let mut queue = BinaryHeap::<SearchState>::new();
        queue.push(SearchState {
            cost: 0,
            node: src,
            step: -1,
        });

        while let Some(SearchState { cost, node, step }) = queue.pop() {
            if step > k {
                continue;
            }

            // Already visited with smaller steps and cost, ignore.
            if step >= steps[node as usize] {
                continue;
            }
            steps[node as usize] = step;

            if node == dst {
                return cost;
            }

            if let Some(adj_nodes) = graph.get(&node) {
                for adj_node in adj_nodes {
                    queue.push(SearchState {
                        cost: cost + adj_node[2],
                        node: adj_node[1],
                        step: step + 1,
                    });
                }
            }
        }

        -1
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
    fn test_787() {
        assert_eq!(
            700,
            Solution::find_cheapest_price(
                4,
                lc_vecvec![
                    [0, 1, 100],
                    [1, 2, 100],
                    [2, 0, 100],
                    [1, 3, 600],
                    [2, 3, 200]
                ],
                0,
                3,
                1
            )
        )
    }
}
