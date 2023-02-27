//!
//! Problem #78: Subsets
//! 
//! - Link: <https://leetcode.com/problems/subsets/>
//! - Discussions: <https://leetcode.com/problems/subsets/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given an integer array `nums` of **unique** elements, return *all possible* *subsets* *(the power set)*.
//! 
//! The solution set **must not** contain duplicate subsets. Return the solution in **any order**.
//! 
//! **Example 1:**
//! 
//! ```
//! Input: nums = [1,2,3]
//! Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
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
//! * All the numbers of `nums` are **unique**.
//!

#[solution]
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        vec![]
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
    fn test_78() {
    }
}
