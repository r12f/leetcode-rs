//!
//! Problem #2244: Minimum Rounds to Complete All Tasks
//!
//! - Link: <https://leetcode.com/problems/minimum-rounds-to-complete-all-tasks/>
//! - Discussions: <https://leetcode.com/problems/minimum-rounds-to-complete-all-tasks/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! You are given a **0-indexed** integer array `tasks`, where `tasks[i]` represents the difficulty level of a task. In each round, you can complete either 2 or 3 tasks of the **same difficulty level**.
//!
//! Return *the **minimum** rounds required to complete all the tasks, or* `-1` *if it is not possible to complete all the tasks.*
//!
//! **Example 1:**
//!
//! ```
//! Input: tasks = [2,2,3,3,2,4,4,4,4,4]
//! Output: 4
//! Explanation: To complete all the tasks, a possible plan is:
//! - In the first round, you complete 3 tasks of difficulty level 2.
//! - In the second round, you complete 2 tasks of difficulty level 3.
//! - In the third round, you complete 3 tasks of difficulty level 4.
//! - In the fourth round, you complete 2 tasks of difficulty level 4.  
//! It can be shown that all the tasks cannot be completed in fewer than 4 rounds, so the answer is 4.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: tasks = [2,3,3]
//! Output: -1
//! Explanation: There is only 1 task of difficulty level 2, but in each round, you can only complete either 2 or 3 tasks of the same difficulty level. Hence, you cannot complete all the tasks, and the answer is -1.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= tasks.length <= 10<sup>5</sup>`
//! * `1 <= tasks[i] <= 10<sup>9</sup>`
//!

#[solution]
impl Solution {
    pub fn minimum_rounds(mut tasks: Vec<i32>) -> i32 {
        tasks.sort();

        let mut results = 0;
        let mut task = tasks[0];
        let mut task_count = 0;
        for i in 0..tasks.len() {
            if tasks[i] != task {
                if task_count < 2 {
                    return -1;
                }

                results += task_count / 3;
                if task_count % 3 != 0 {
                    results += 1;
                }

                task = tasks[i];
                task_count = 1;
            } else {
                task_count += 1;
            }
        }

        if task_count < 2 {
            return -1;
        }

        results += task_count / 3;
        if task_count % 3 != 0 {
            results += 1;
        }

        results
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
    fn test_2244() {
        assert_eq!(
            4,
            Solution::minimum_rounds(vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4])
        );

        assert_eq!(-1, Solution::minimum_rounds(vec![2, 3, 3]));
    }
}
