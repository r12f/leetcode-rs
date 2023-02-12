//!
//! Problem #3: Longest Substring Without Repeating Characters
//!
//! - Link: <https://leetcode.com/problems/longest-substring-without-repeating-characters/>
//! - Discussions: <https://leetcode.com/problems/longest-substring-without-repeating-characters/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given a string `s`, find the length of the **longest** **substring** without repeating characters.
//!
//! **Example 1:**
//!
//! ```
//! Input: s = "abcabcbb"
//! Output: 3
//! Explanation: The answer is "abc", with the length of 3.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: s = "bbbbb"
//! Output: 1
//! Explanation: The answer is "b", with the length of 1.
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: s = "pwwkew"
//! Output: 3
//! Explanation: The answer is "wke", with the length of 3.
//! Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `0 <= s.length <= 5 * 10<sup>4</sup>`
//! * `s` consists of English letters, digits, symbols and spaces.
//!

#[solution]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut result = 0;

        let mut current_length = 0;
        let mut last_positions: Vec<i32> = vec![-1; 255];
        let str_bytes = s.as_bytes();

        for i in 0..str_bytes.len() {
            let c = str_bytes[i];
            current_length += 1;

            let last_pos = last_positions[c as usize];
            if last_pos != -1 {
                let start_pos = (i + 1 - current_length) as usize;
                for prev_char_index in start_pos..=last_pos as usize {
                    last_positions[str_bytes[prev_char_index] as usize] = -1;
                }
                current_length = i - last_pos as usize;
            }

            if current_length > result {
                result = current_length;
            }

            last_positions[c as usize] = i as i32;
        }

        return result as i32;
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
    fn test_3() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring("abcabccbb".to_string())
        );

        assert_eq!(
            5,
            Solution::length_of_longest_substring("tmmzuxt".to_string())
        );
    }
}
