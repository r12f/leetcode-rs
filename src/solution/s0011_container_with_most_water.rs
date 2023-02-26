//!
//! Problem #11: Container With Most Water
//!
//! - Link: <https://leetcode.com/problems/container-with-most-water/>
//! - Discussions: <https://leetcode.com/problems/container-with-most-water/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! You are given an integer array `height` of length `n`. There are `n` vertical lines drawn such that the two endpoints of the `i<sup>th</sup>` line are `(i, 0)` and `(i, height[i])`.
//!
//! Find two lines that together with the x-axis form a container, such that the container contains the most water.
//!
//! Return *the maximum amount of water a container can store*.
//!
//! **Notice** that you may not slant the container.
//!
//! **Example 1:**
//!
//! ![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/07/17/question_11.jpg)
//!
//! ```
//! Input: height = [1,8,6,2,5,4,8,3,7]
//! Output: 49
//! Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the container can contain is 49.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: height = [1,1]
//! Output: 1
//!
//! ```
//!
//! **Constraints:**
//!
//! * `n == height.length`
//! * `2 <= n <= 10<sup>5</sup>`
//! * `0 <= height[i] <= 10<sup>4</sup>`
//!

#[solution]
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut left = 0usize;
        let mut right = height.len() - 1;
        while left < right {
            let area = height[left].min(height[right]) * (right - left) as i32;
            if max_area < area {
                max_area = area;
            }

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_area
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
    fn test_11() {
        assert_eq!(49, Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    }
}
