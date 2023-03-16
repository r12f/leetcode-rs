//!
//! Problem #670: Maximum Swap
//!
//! - Link: <https://leetcode.com/problems/maximum-swap/>
//! - Discussions: <https://leetcode.com/problems/maximum-swap/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! You are given an integer `num`. You can swap two digits at most once to get the maximum valued number.
//!
//! Return *the maximum valued number you can get*.
//!
//! **Example 1:**
//!
//! ```
//! Input: num = 2736
//! Output: 7236
//! Explanation: Swap the number 2 and the number 7.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: num = 9973
//! Output: 9973
//! Explanation: No swap.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `0 <= num <= 10<sup>8</sup>`
//!

#[solution]
impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut num_string = num.to_string();
        let num_string_bytes = num_string.as_bytes();

        let mut min_digit_stack = Vec::<(u8, usize)>::new();
        for i in 0..num_string_bytes.len() {
            let digit = num_string_bytes[i];
            if min_digit_stack.is_empty() || min_digit_stack.last().unwrap().0 > digit {
                min_digit_stack.push((digit, i));
            }
        }

        let mut max_digit_stack = Vec::<(u8, usize)>::new();
        for i in (0..num_string_bytes.len()).rev() {
            let digit = num_string_bytes[i];
            if max_digit_stack.is_empty() || max_digit_stack.last().unwrap().0 < digit {
                max_digit_stack.push((digit, i));
            }
        }
        drop(num_string_bytes);

        let mut min_stack_index = 0;
        let mut max_stack_index = max_digit_stack.len() - 1;
        while min_stack_index < min_digit_stack.len() && max_stack_index < max_digit_stack.len() {
            if min_digit_stack[min_stack_index].1 >= max_digit_stack[max_stack_index].1 {
                min_stack_index = 0;
                max_stack_index -= 1;
            } else if min_digit_stack[min_stack_index].0 >= max_digit_stack[max_stack_index].0 {
                min_stack_index += 1;
                if min_stack_index >= min_digit_stack.len() {
                    min_stack_index = 0;
                    max_stack_index -= 1;
                }
            } else {
                let num_string_bytes = unsafe { num_string.as_bytes_mut() };
                num_string_bytes.swap(
                    min_digit_stack[min_stack_index].1,
                    max_digit_stack[max_stack_index].1,
                );
                drop(num_string_bytes);
                return num_string.parse::<i32>().unwrap();
            }
        }

        num
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
    fn test_670() {
        assert_eq!(7236, Solution::maximum_swap(2736));
        assert_eq!(75341, Solution::maximum_swap(74351));
        assert_eq!(98863, Solution::maximum_swap(98368));
    }
}
