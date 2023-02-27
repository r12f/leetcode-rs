//!
//! Problem #139: Word Break
//!
//! - Link: <https://leetcode.com/problems/word-break/>
//! - Discussions: <https://leetcode.com/problems/word-break/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given a string `s` and a dictionary of strings `wordDict`, return `true` if `s` can be segmented into a space-separated sequence of one or more dictionary words.
//!
//! **Note** that the same word in the dictionary may be reused multiple times in the segmentation.
//!
//! **Example 1:**
//!
//! ```
//! Input: s = "leetcode", wordDict = ["leet","code"]
//! Output: true
//! Explanation: Return true because "leetcode" can be segmented as "leet code".
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: s = "applepenapple", wordDict = ["apple","pen"]
//! Output: true
//! Explanation: Return true because "applepenapple" can be segmented as "apple pen apple".
//! Note that you are allowed to reuse a dictionary word.
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
//! Output: false
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= s.length <= 300`
//! * `1 <= wordDict.length <= 1000`
//! * `1 <= wordDict[i].length <= 20`
//! * `s` and `wordDict[i]` consist of only lowercase English letters.
//! * All the strings of `wordDict` are **unique**.
//!

use std::collections::HashSet;

#[solution]
impl Solution {
    /*
    ```
    /// DP: dp[i][j] = dp[i][n] && dp[n + 1][j]
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let words = word_dict.into_iter().collect::<HashSet<String>>();

        let mut dp = vec![vec![false; s.len()]; s.len()];
        for i in 0..s.len() {
            dp[i][i] = words.contains(&s[i..=i]);
        }

        for l in 2..=s.len() {
            for left_start in 0..=(s.len() - l) {
                for n in 0..=(l - 1) {
                    let left_range = left_start..=(left_start + n);
                    let left = &s[left_range.clone()];
                    let left_ok =
                        dp[*left_range.start()][*left_range.end()] || words.contains(left);

                    let right_range = (left_start + n + 1)..=(left_start + l - 1);
                    let right = &s[right_range.clone()];
                    let right_ok = if left_start + n + 1 <= left_start + l - 1 {
                        dp[*right_range.start()][*right_range.end()] || words.contains(right)
                    } else {
                        true
                    };

                    dp[*left_range.start()][*right_range.end()] =
                        dp[*left_range.start()][*right_range.end()] || (left_ok && right_ok);

                    if dp[left_start][left_start + l - 1] {
                        break;
                    }
                }
            }
        }

        dp[0][s.len() - 1]
    }
    ```
    */

    /// DP: dp[l] = dp[n] && dict.contains(s[n..])
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let words = word_dict.into_iter().collect::<HashSet<String>>();

        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;

        for l in 1..=s.len() {
            for n in 0..=(l - 1) {
                let result = dp[n] && words.contains(&s[n..l]);
                // println!("Checking {} - {}: {}", &s[0..n], &s[n..l], result);

                if result {
                    dp[l] = true;
                    break;
                }
            }
        }

        dp[s.len()]
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
    fn test_139() {
        assert_eq!(true, Solution::word_break("a".into(), lc_strvec!["a"]));
        assert_eq!(true, Solution::word_break("abc".into(), lc_strvec!["abc"]));

        assert_eq!(
            true,
            Solution::word_break("leetcode".into(), lc_strvec!["leet", "code"])
        );

        assert_eq!(
            true,
            Solution::word_break("applepenapple".into(), lc_strvec!["apple", "pen"])
        );
    }
}
