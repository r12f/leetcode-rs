//!
//! Problem #1762: Buildings With an Ocean View
//!
//! - Link: <https://leetcode.com/problems/buildings-with-an-ocean-view/>
//! - Discussions: <https://leetcode.com/problems/buildings-with-an-ocean-view/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! There are `n` buildings in a line. You are given an integer array `heights` of size `n` that represents the heights of the buildings in the line.
//!
//! The ocean is to the right of the buildings. A building has an ocean view if the building can see the ocean without obstructions. Formally, a building has an ocean view if all the buildings to its right have a **smaller** height.
//!
//! Return a list of indices **(0-indexed)** of buildings that have an ocean view, sorted in increasing order.
//!
//! **Example 1:**
//!
//! ```
//! Input: heights = [4,2,3,1]
//! Output: [0,2,3]
//! Explanation: Building 1 (0-indexed) does not have an ocean view because building 2 is taller.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: heights = [4,3,2,1]
//! Output: [0,1,2,3]
//! Explanation: All the buildings have an ocean view.
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: heights = [1,3,2,4]
//! Output: [3]
//! Explanation: Only building 3 has an ocean view.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= heights.length <= 10<sup>5</sup>`
//! * `1 <= heights[i] <= 10<sup>9</sup>`
//!

#[solution]
impl Solution {
    pub fn find_buildings(heights: Vec<i32>) -> Vec<i32> {
        let mut index = (heights.len() - 1) as i32;
        let mut max_height = 0;

        let mut result = Vec::new();
        while index >= 0 {
            let height = heights[index as usize];
            if height > max_height {
                result.push(index);
                max_height = height;
            }

            index -= 1;
        }

        result.reverse();
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
    fn test_1762() {
        assert_eq!(vec![0, 2, 3], Solution::find_buildings(vec![4, 2, 3, 1]));
    }
}
