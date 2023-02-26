//!
//! Problem #42: Trapping Rain Water
//!
//! - Link: <https://leetcode.com/problems/trapping-rain-water/>
//! - Discussions: <https://leetcode.com/problems/trapping-rain-water/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given `n` non-negative integers representing an elevation map where the width of each bar is `1`, compute how much water it can trap after raining.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2018/10/22/rainwatertrap.png)
//!
//! ```
//! Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
//! Output: 6
//! Explanation: The above elevation map (black section) is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: height = [4,2,0,3,2,5]
//! Output: 9
//!
//! ```
//!
//! **Constraints:**
//!
//! * `n == height.length`
//! * `1 <= n <= 2 * 10<sup>4</sup>`
//! * `0 <= height[i] <= 10<sup>5</sup>`
//!

struct WallState {
    index: usize,
    start_height: i32,
    height: i32,
}

impl WallState {
    pub fn new(index: usize, height: i32) -> Self {
        WallState {
            index,
            start_height: 0,
            height,
        }
    }

    pub fn trap_area(&self, other: &WallState) -> i32 {
        if other.height < self.start_height {}
        0
    }
}

#[solution]
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 2 {
            return 0;
        }

        let mut result = 0;

        let mut stack = Vec::<WallState>::new();
        stack.push(WallState::new(0, height[0]));

        for (i, h) in height[1..].iter().enumerate().map(|(i, v)| (i + 1, v)) {
            while !stack.is_empty() {
                // Previous wall is lower
                if stack.last().as_ref().unwrap().height <= *h {
                    let prev_wall_state = stack.pop().unwrap();
                    let area = (prev_wall_state.height - prev_wall_state.start_height)
                        * (i - prev_wall_state.index - 1) as i32;

                    result += area;
                }
                // Previous wall is higher
                else {
                    let prev_wall_state = stack.last_mut().unwrap();
                    let area =
                        (h - prev_wall_state.start_height) * (i - prev_wall_state.index - 1) as i32;
                    result += area;
                    prev_wall_state.start_height = *h;
                    break;
                }
            }

            stack.push(WallState::new(i, *h));
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
    fn test_42() {
        assert_eq!(6, Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
    }
}
