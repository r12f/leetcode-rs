//!
//! Problem #210: Course Schedule II
//!
//! - Link: <https://leetcode.com/problems/course-schedule-ii/>
//! - Discussions: <https://leetcode.com/problems/course-schedule-ii/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! There are a total of `numCourses` courses you have to take, labeled from `0` to `numCourses - 1`. You are given an array `prerequisites` where `prerequisites[i] = [a<sub>i</sub>, b<sub>i</sub>]` indicates that you **must** take course `b<sub>i</sub>` first if you want to take course `a<sub>i</sub>`.
//!
//! * For example, the pair `[0, 1]`, indicates that to take course `0` you have to first take course `1`.
//!
//! Return *the ordering of courses you should take to finish all courses*. If there are many valid answers, return **any** of them. If it is impossible to finish all courses, return **an empty array**.
//!
//! **Example 1:**
//!
//! ```
//! Input: numCourses = 2, prerequisites = [[1,0]]
//! Output: [0,1]
//! Explanation: There are a total of 2 courses to take. To take course 1 you should have finished course 0. So the correct course order is [0,1].
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: numCourses = 4, prerequisites = [[1,0],[2,0],[3,1],[3,2]]
//! Output: [0,2,1,3]
//! Explanation: There are a total of 4 courses to take. To take course 3 you should have finished both courses 1 and 2. Both courses 1 and 2 should be taken after you finished course 0.
//! So one correct course order is [0,1,2,3]. Another correct ordering is [0,2,1,3].
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: numCourses = 1, prerequisites = []
//! Output: [0]
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= numCourses <= 2000`
//! * `0 <= prerequisites.length <= numCourses * (numCourses - 1)`
//! * `prerequisites[i].length == 2`
//! * `0 <= a<sub>i</sub>, b<sub>i</sub> < numCourses`
//! * `a<sub>i</sub> != b<sub>i</sub>`
//! * All the pairs `[a<sub>i</sub>, b<sub>i</sub>]` are **distinct**.
//!

use std::collections::HashSet;

#[solution]
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut visited = vec![false; num_courses as usize];
        let mut ordered_courses = Vec::<i32>::new();
        let mut parent_courses = HashSet::<i32>::new();

        let mut pre_courses = vec![vec![]; num_courses as usize];
        for prerequisite in prerequisites.iter() {
            pre_courses[prerequisite[0] as usize].push(prerequisite[1]);
        }

        for id in 0..num_courses {
            assert!(parent_courses.is_empty());
            if !Self::dfs(
                &mut ordered_courses,
                id,
                &pre_courses,
                &mut visited,
                &mut parent_courses,
            ) {
                return vec![];
            }
        }

        ordered_courses
    }

    fn dfs(
        ordered_courses: &mut Vec<i32>,
        current_course: i32,
        pre_courses: &Vec<Vec<i32>>,
        visited: &mut Vec<bool>,
        parent_courses: &mut HashSet<i32>,
    ) -> bool {
        if !parent_courses.insert(current_course) {
            // Loop is found.
            return false;
        }

        // If current course is not selected yet, we check all its prerequite courses and select this course
        if !visited[current_course as usize] {
            for pre_course in pre_courses[current_course as usize].iter() {
                if !Self::dfs(
                    ordered_courses,
                    *pre_course,
                    pre_courses,
                    visited,
                    parent_courses,
                ) {
                    return false;
                }
            }

            ordered_courses.push(current_course);
            visited[current_course as usize] = true;
        }

        parent_courses.remove(&current_course);

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
    fn test_210() {
        assert_eq!(vec![0], Solution::find_order(1, lc_vecvec![]));

        assert_eq!(vec![0, 1], Solution::find_order(2, lc_vecvec![[1, 0]]));

        assert_eq!(
            vec![0, 1, 2, 3],
            Solution::find_order(4, lc_vecvec![[1, 0], [2, 0], [3, 1], [3, 2]])
        );
    }
}
