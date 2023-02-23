//!
//! Problem #5: Longest Palindromic Substring
//!
//! - Link: <https://leetcode.com/problems/longest-palindromic-substring/>
//! - Discussions: <https://leetcode.com/problems/longest-palindromic-substring/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given a string `s`, return *the longest* *palindromic* *substring* in `s`.
//!
//! **Example 1:**
//!
//! ```
//! Input: s = "babad"
//! Output: "bab"
//! Explanation: "aba" is also a valid answer.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: s = "cbbd"
//! Output: "bb"
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= s.length <= 1000`
//! * `s` consist of only digits and English letters.
//!

#[solution]
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 0 {
            return "".into();
        }

        let bytes = s.as_bytes();

        let mut dp = vec![vec![0; s.len()]; 3];
        for i in 0..s.len() {
            dp[1][i] = 1;
        }

        let mut max_start = 0;
        let mut max_len = 1;

        for step in 2..=s.len() {
            let dp_cur = step % 3;
            let dp_prev = (step - 1) % 3;
            let dp_prevprev = (step - 2) % 3;

            for start_index in 0..=(s.len() - step) {
                let end_index = start_index + step - 1;

                if bytes[start_index] != bytes[end_index] {
                    dp[dp_cur][start_index] = 0;
                    continue;
                }

                // If inner one is not palindrome, then outer one will not be one.
                if step != 2 && dp[dp_prevprev][start_index + 1] == 0 {
                    dp[dp_cur][start_index] = 0;
                    continue;
                }

                dp[dp_cur][start_index] = step;

                // [s + 1][e - 1]
                if dp[dp_prevprev][start_index + 1] + 2 > max_len {
                    max_len = step;
                    max_start = start_index;
                }
            }
        }

        let result_bytes = &s[max_start..(max_start + max_len)];
        String::from(result_bytes)
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
    fn test_5() {
        assert_eq!("bab", Solution::longest_palindrome("babad".into()));
        assert_eq!("bb", Solution::longest_palindrome("cbbd".into()));
        assert_eq!("bb", Solution::longest_palindrome("bb".into()));
        assert_eq!("aaaa", Solution::longest_palindrome("aaaa".into()));
        assert_eq!("", Solution::longest_palindrome("".into()));
        assert_eq!("aca", Solution::longest_palindrome("aacabdkacaa".into()));
    }
}
