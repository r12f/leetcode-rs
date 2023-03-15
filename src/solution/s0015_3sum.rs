//!
//! Problem #15: 3Sum
//!
//! - Link: <https://leetcode.com/problems/3sum/>
//! - Discussions: <https://leetcode.com/problems/3sum/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given an integer array nums, return all the triplets `[nums[i], nums[j], nums[k]]` such that `i != j`, `i != k`, and `j != k`, and `nums[i] + nums[j] + nums[k] == 0`.
//!
//! Notice that the solution set must not contain duplicate triplets.
//!
//! **Example 1:**
//!
//! ```
//! Input: nums = [-1,0,1,2,-1,-4]
//! Output: [[-1,-1,2],[-1,0,1]]
//! Explanation:
//! nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
//! nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
//! nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
//! The distinct triplets are [-1,0,1] and [-1,-1,2].
//! Notice that the order of the output and the order of the triplets does not matter.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums = [0,1,1]
//! Output: []
//! Explanation: The only possible triplet does not sum up to 0.
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: nums = [0,0,0]
//! Output: [[0,0,0]]
//! Explanation: The only possible triplet sums up to 0.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `3 <= nums.length <= 3000`
//! * `-10<sup>5</sup> <= nums[i] <= 10<sup>5</sup>`
//!

#[solution]
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();

        let mut result = Vec::<Vec<i32>>::new();
        let mut pivot = 0;
        for pivot in 0..(nums.len() - 2) {
            if pivot > 0 && nums[pivot] == nums[pivot - 1] {
                continue;
            }

            let mut left = pivot + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = nums[pivot] + nums[left] + nums[right];
                if sum == 0 {
                    result.push(vec![nums[pivot], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;

                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                } else if sum > 0 {
                    right -= 1;
                } else {
                    left += 1;
                }
            }
        }

        result
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
    fn test_15() {
        assert_eq!(
            lc_vecvec![[-1, -1, 2], [-1, 0, 1]],
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4])
        );
    }
}
