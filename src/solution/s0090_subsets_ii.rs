//!
//! Problem #90: Subsets II
//!
//! - Link: <https://leetcode.com/problems/subsets-ii/>
//! - Discussions: <https://leetcode.com/problems/subsets-ii/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given an integer array `nums` that may contain duplicates, return *all possible* *subsets* *(the power set)*.
//!
//! The solution set **must not** contain duplicate subsets. Return the solution in **any order**.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [1,2,2]
//! Output: [[],[1],[1,2],[1,2,2],[2],[2,2]]
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [0]
//! Output: [[],[0]]
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= nums.length <= 10`
//! * `-10 <= nums[i] <= 10`
//!

use std::collections::HashSet;

#[solution]
impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();

        let mut results = HashSet::<Vec<i32>>::new();

        let mut result = Vec::new();
        results.insert(result.clone());

        Solution::dfs(&mut results, &mut result, &nums, 0);

        results.into_iter().collect::<Vec<Vec<i32>>>()
    }

    fn dfs(
        results: &mut HashSet<Vec<i32>>,
        result: &mut Vec<i32>,
        nums: &Vec<i32>,
        start_index: i32,
    ) {
        for i in (start_index as usize)..nums.len() {
            result.push(nums[i]);

            if results.insert(result.clone()) {
                Solution::dfs(results, result, nums, (i + 1) as i32);
            }

            result.pop();
        }
    }
}

/* MLE */
/*
use std::collections::{btree_map::IterMut, BTreeMap};

#[solution]
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut num_counts = BTreeMap::<i32, i32>::new();
        nums.iter()
            .for_each(|x| *num_counts.entry(*x).or_default() += 1);

        let mut results = Vec::new();

        let mut result = Vec::new();
        results.push(result.clone());

        let v = *num_counts.iter().next().unwrap().0;
        Solution::dfs(&mut results, &mut result, &mut num_counts, v);

        results
    }

    fn dfs(
        results: &mut Vec<Vec<i32>>,
        result: &mut Vec<i32>,
        num_counts: &mut BTreeMap<i32, i32>,
        start_element: i32,
    ) {
        let mut n = start_element;
        loop {
            let mut count = num_counts.get_mut(&n).unwrap();

            if *count == 0 {
                let mut next_element_it = num_counts.range(start_element..);
                next_element_it.next();

                // All numbers are picked up, return. Otherwise, pick the next available number.
                n = match next_element_it.next() {
                    Some(x) => *x.0,
                    None => return,
                };

                count = num_counts.get_mut(&n).unwrap();
            }

            *count -= 1;
            drop(count);

            result.push(n);
            results.push(result.clone());
            Solution::dfs(results, result, num_counts, n);

            let mut count = num_counts.get_mut(&n).unwrap();
            result.pop();
            *count += 1;
        }
    }
}
 */

//
// Tests
//
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_90() {
        assert_eq!(
            lc_vecvec![[], [1], [1, 2], [1, 2, 2], [2], [2, 2]],
            Solution::subsets_with_dup(vec![1, 2, 2])
        );

        assert_eq!(lc_vecvec![[], [0]], Solution::subsets_with_dup(vec![0]));
    }
}
