//!
//! Problem #159: Longest Substring with At Most Two Distinct Characters
//!
//! - Link: <https://leetcode.com/problems/longest-substring-with-at-most-two-distinct-characters/>
//! - Discussions: <https://leetcode.com/problems/longest-substring-with-at-most-two-distinct-characters/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given a string `s`, return *the length of the longest* *substring* *that contains at most **two distinct characters***.
//!
//! **Example 1:**
//!
//! ```
//! Input: s = "eceba"
//! Output: 3
//! Explanation: The substring is "ece" which its length is 3.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: s = "ccaabbb"
//! Output: 5
//! Explanation: The substring is "aabbb" which its length is 5.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= s.length <= 10<sup>5</sup>`
//! * `s` consists of English letters.
//!

use std::collections::HashMap;

#[solution]
impl Solution {
    pub fn length_of_longest_substring_two_distinct(s: String) -> i32 {
        let mut last_char_index = HashMap::<char, usize>::new();

        let mut max = 0;
        let mut current = 0;
        for (i, c) in s.chars().enumerate() {
            if last_char_index.contains_key(&c) {
                current += 1;
                *last_char_index.get_mut(&c).unwrap() = i;
            } else if last_char_index.len() < 2 {
                current += 1;
                last_char_index.insert(c, i);
            } else {
                // Remove the smaller one.
                let (smaller_char, smaller_index) =
                    last_char_index.iter().min_by_key(|v| v.1).unwrap();

                let smaller_char = *smaller_char;
                let smaller_index = *smaller_index;

                current = current - smaller_index;
                last_char_index.remove(&smaller_char);
                last_char_index.insert(c, i);
            }

            if current > max {
                max = current
            }
        }

        max as i32
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
    fn test_159() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring_two_distinct("eceba".into())
        );
    }
}
