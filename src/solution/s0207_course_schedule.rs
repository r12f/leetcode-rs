//!
//! Problem #207: Course Schedule
//!
//! - Link: <https://leetcode.com/problems/course-schedule/>
//! - Discussions: <https://leetcode.com/problems/course-schedule/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! There are a total of `numCourses` courses you have to take, labeled from `0` to `numCourses - 1`. You are given an array `prerequisites` where `prerequisites[i] = [a<sub>i</sub>, b<sub>i</sub>]` indicates that you **must** take course `b<sub>i</sub>` first if you want to take course `a<sub>i</sub>`.
//!
//! * For example, the pair `[0, 1]`, indicates that to take course `0` you have to first take course `1`.
//!
//! Return `true` if you can finish all courses. Otherwise, return `false`.
//!
//! **Example 1:**
//!
//! ```
//! Input: numCourses = 2, prerequisites = [[1,0]]
//! Output: true
//! Explanation: There are a total of 2 courses to take.
//! To take course 1 you should have finished course 0. So it is possible.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: numCourses = 2, prerequisites = [[1,0],[0,1]]
//! Output: false
//! Explanation: There are a total of 2 courses to take.
//! To take course 1 you should have finished course 0, and to take course 0 you should also have finished course 1. So it is impossible.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= numCourses <= 2000`
//! * `0 <= prerequisites.length <= 5000`
//! * `prerequisites[i].length == 2`
//! * `0 <= a<sub>i</sub>, b<sub>i</sub> < numCourses`
//! * All the pairs prerequisites[i] are **unique**.
//!

use std::collections::HashSet;

#[solution]
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut src_nodes = Vec::<Vec<usize>>::new();
        src_nodes.resize(num_courses as usize, vec![]);

        for prerequisite in &prerequisites {
            src_nodes[prerequisite[0] as usize].push(prerequisite[1] as usize);
        }

        let mut parents: HashSet<usize> = HashSet::new();
        let mut visited = vec![false; num_courses as usize];
        for i in 0..(num_courses as usize) {
            if !Self::dfs(&src_nodes, i, &mut visited, &mut parents) {
                return false;
            }
        }

        return true;
    }

    fn dfs(
        src_nodes: &Vec<Vec<usize>>,
        i: usize,
        visited: &mut Vec<bool>,
        parents: &mut HashSet<usize>,
    ) -> bool {
        if parents.contains(&i) {
            return false;
        }

        if visited[i] {
            return true;
        }
        visited[i] = true;

        parents.insert(i);

        for node in src_nodes[i].iter() {
            if !Self::dfs(src_nodes, *node, visited, parents) {
                return false;
            }
        }

        parents.remove(&i);

        return true;
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
    fn test_207() {}
}
