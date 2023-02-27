//!
//! Problem #55: Jump Game
//!
//! - Link: <https://leetcode.com/problems/jump-game/>
//! - Discussions: <https://leetcode.com/problems/jump-game/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! You are given an integer array `nums`. You are initially positioned at the array's **first index**, and each element in the array represents your maximum jump length at that position.
//!
//! Return `true` *if you can reach the last index, or* `false` *otherwise*.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [2,3,1,1,4]
//! Output: true
//! Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [3,2,1,0,4]
//! Output: false
//! Explanation: You will always arrive at index 3 no matter what. Its maximum jump length is 0, which makes it impossible to reach the last index.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= nums.length <= 10<sup>4</sup>`
//! * `0 <= nums[i] <= 10<sup>5</sup>`
//!

#[solution]
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        // dp[i] = dp[i - k] && n[k] > (i - k)
        let mut dp = vec![false; nums.len()];
        dp[0] = true;

        for i in 1..nums.len() {
            for k in 0..i {
                dp[i] = dp[k] && (nums[k] as usize >= (i - k));
                if dp[i] {
                    break;
                }
            }
        }

        dp[nums.len() - 1]
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
    fn test_55() {
        assert_eq!(true, Solution::can_jump(vec![2, 3, 1, 1, 4]));
        assert_eq!(false, Solution::can_jump(vec![3, 2, 1, 0, 4]));
    }
}
