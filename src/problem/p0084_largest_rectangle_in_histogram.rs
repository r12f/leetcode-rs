//!
//! Problem #84: Largest Rectangle in Histogram
//! 
//! - Link: <https://leetcode.com/problems/largest-rectangle-in-histogram/>
//! - Discussions: <https://leetcode.com/problems/largest-rectangle-in-histogram/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given an array of integers `heights` representing the histogram's bar height where the width of each bar is `1`, return *the area of the largest rectangle in the histogram*.
//! 
//! **Example 1:**
//! 
//! ![](https://assets.leetcode.com/uploads/2021/01/04/histogram.jpg)
//! 
//! ```
//! Input: heights = [2,1,5,6,2,3]
//! Output: 10
//! Explanation: The above is a histogram where width of each bar is 1.
//! The largest rectangle is shown in the red area, which has an area = 10 units.
//! 
//! ```
//! 
//! **Example 2:**
//! 
//! ![](https://assets.leetcode.com/uploads/2021/01/04/histogram-1.jpg)
//! 
//! ```
//! Input: heights = [2,4]
//! Output: 4
//! 
//! ```
//! 
//! **Constraints:**
//! 
//! * `1 <= heights.length <= 10<sup>5</sup>`
//! * `0 <= heights[i] <= 10<sup>4</sup>`
//!

#[solution]
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        0
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
    fn test_84() {
    }
}
