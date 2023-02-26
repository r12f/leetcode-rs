//!
//! Problem #739: Daily Temperatures
//!
//! - Link: <https://leetcode.com/problems/daily-temperatures/>
//! - Discussions: <https://leetcode.com/problems/daily-temperatures/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given an array of integers `temperatures` represents the daily temperatures, return *an array* `answer` *such that* `answer[i]` *is the number of days you have to wait after the* `i<sup>th</sup>` *day to get a warmer temperature*. If there is no future day for which this is possible, keep `answer[i] == 0` instead.
//!
//! **Example 1:**
//!
//! ```
//! Input: temperatures = [73,74,75,71,69,72,76,73]
//! Output: [1,1,4,2,1,1,0,0]
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: temperatures = [30,40,50,60]
//! Output: [1,1,1,0]
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: temperatures = [30,60,90]
//! Output: [1,1,0]
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= temperatures.length <= 10<sup>5</sup>`
//! * `30 <= temperatures[i] <= 100`
//!

#[solution]
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        assert!(!temperatures.is_empty());

        let mut result = vec![0; temperatures.len()];

        let mut stack = Vec::<(i32, usize)>::new();

        for (i, t) in temperatures.iter().enumerate() {
            while let Some((prev_temp, prev_index)) = stack.last() {
                if *prev_temp >= *t {
                    break;
                }

                result[*prev_index] = (i - prev_index) as i32;
                stack.pop();
            }

            stack.push((*t, i));
        }

        while let Some((_, i)) = stack.pop() {
            result[i] = 0;
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
    fn test_739() {
        assert_eq!(
            vec![1, 1, 4, 2, 1, 1, 0, 0],
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73])
        );
    }
}
