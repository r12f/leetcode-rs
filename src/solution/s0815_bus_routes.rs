//!
//! Problem #815: Bus Routes
//!
//! - Link: <https://leetcode.com/problems/bus-routes/>
//! - Discussions: <https://leetcode.com/problems/bus-routes/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! You are given an array `routes` representing bus routes where `routes[i]` is a bus route that the `i<sup>th</sup>` bus repeats forever.
//!
//! * For example, if `routes[0] = [1, 5, 7]`, this means that the `0<sup>th</sup>` bus travels in the sequence `1 -> 5 -> 7 -> 1 -> 5 -> 7 -> 1 -> ...` forever.
//!
//! You will start at the bus stop `source` (You are not on any bus initially), and you want to go to the bus stop `target`. You can travel between bus stops by buses only.
//!
//! Return *the least number of buses you must take to travel from* `source` *to* `target`. Return `-1` if it is not possible.
//!
//! **Example 1:**
//!
//! ```
//! Input: routes = [[1,2,7],[3,6,7]], source = 1, target = 6
//! Output: 2
//! Explanation: The best strategy is take the first bus to the bus stop 7, then take the second bus to the bus stop 6.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: routes = [[7,12],[4,5,15],[6],[15,19],[9,12,13]], source = 15, target = 12
//! Output: -1
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= routes.length <= 500`.
//! * `1 <= routes[i].length <= 10<sup>5</sup>`
//! * All the values of `routes[i]` are **unique**.
//! * `sum(routes[i].length) <= 10<sup>5</sup>`
//! * `0 <= routes[i][j] < 10<sup>6</sup>`
//! * `0 <= source, target < 10<sup>6</sup>`
//!

use std::collections::{HashMap, HashSet, VecDeque};

#[solution]
impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }

        // Instead of building stop graph, we build bus graph here to save search time.
        let mut stop_to_routes_map = HashMap::<i32, Vec<i32>>::new();
        let mut routes_to_stop_map = vec![HashSet::<i32>::new(); routes.len()];
        let mut routes_map = vec![HashSet::<i32>::new(); routes.len()];
        for (route_index, route) in routes.iter().enumerate() {
            for stop in route {
                let stop_entry = stop_to_routes_map.entry(*stop).or_default();
                for route_of_stop in stop_entry.iter() {
                    // Undirectional map
                    routes_map[route_index].insert(*route_of_stop as i32);
                    routes_map[*route_of_stop as usize].insert(route_index as i32);
                }
                stop_entry.push(route_index as i32);

                routes_to_stop_map[route_index].insert(*stop);
            }
        }

        let mut queue = VecDeque::<(i32, i32)>::new();
        for init_route in stop_to_routes_map.get(&source).unwrap().iter() {
            queue.push_back((1, *init_route));
        }

        let mut visited = vec![false; routes.len()];
        while let Some((bus_count, route)) = queue.pop_front() {
            if visited[route as usize] {
                continue;
            }
            visited[route as usize] = true;

            if routes_to_stop_map[route as usize].contains(&target) {
                return bus_count;
            }

            for next_route in routes_map[route as usize].iter() {
                queue.push_back((bus_count + 1, *next_route));
            }
        }

        return -1;
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
    fn test_815() {
        assert_eq!(
            2,
            Solution::num_buses_to_destination(lc_vecvec![[1, 2, 7], [3, 6, 7]], 1, 6)
        );

        assert_eq!(
            -1,
            Solution::num_buses_to_destination(
                lc_vecvec![[7, 12], [4, 5, 15], [6], [15, 19], [9, 12, 13]],
                15,
                12
            )
        );
    }
}
