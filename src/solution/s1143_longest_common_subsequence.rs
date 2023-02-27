//!
//! Problem #1143: Longest Common Subsequence
//!
//! - Link: <https://leetcode.com/problems/longest-common-subsequence/>
//! - Discussions: <https://leetcode.com/problems/longest-common-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given two strings `text1` and `text2`, return *the length of their longest **common subsequence**.* If there is no **common subsequence**, return `0`.
//!
//! A **subsequence** of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.
//!
//! * For example, `"ace"` is a subsequence of `"abcde"`.
//!
//! A **common subsequence** of two strings is a subsequence that is common to both strings.
//!
//! **Example 1:**
//!
//! ```
//! Input: text1 = "abcde", text2 = "ace"
//! Output: 3  
//! Explanation: The longest common subsequence is "ace" and its length is 3.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: text1 = "abc", text2 = "abc"
//! Output: 3
//! Explanation: The longest common subsequence is "abc" and its length is 3.
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: text1 = "abc", text2 = "def"
//! Output: 0
//! Explanation: There is no such common subsequence, so the result is 0.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= text1.length, text2.length <= 1000`
//! * `text1` and `text2` consist of only lowercase English characters.
//!

#[solution]
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];

        let b1 = text1.as_bytes();
        let b2 = text2.as_bytes();
        for p1 in (0..b1.len()).rev() {
            for p2 in (0..b2.len()).rev() {
                if b1[p1] == b2[p2] {
                    dp[p1][p2] = 1 + dp[p1 + 1][p2 + 1];
                } else {
                    dp[p1][p2] = dp[p1 + 1][p2].max(dp[p1][p2 + 1]);
                }
            }
        }

        dp[0][0]
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
    fn test_1143() {}
}
