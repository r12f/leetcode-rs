//!
//! Problem #1249: Minimum Remove to Make Valid Parentheses
//!
//! - Link: <https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/>
//! - Discussions: <https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given a string s of `'('` , `')'` and lowercase English characters.
//!
//! Your task is to remove the minimum number of parentheses ( `'('` or `')'`, in any positions ) so that the resulting *parentheses string* is valid and return **any** valid string.
//!
//! Formally, a *parentheses string* is valid if and only if:
//!
//! * It is the empty string, contains only lowercase characters, or
//! * It can be written as `AB` (`A` concatenated with `B`), where `A` and `B` are valid strings, or
//! * It can be written as `(A)`, where `A` is a valid string.
//!
//! **Example 1:**
//!
//! ```
//! Input: s = "lee(t(c)o)de)"
//! Output: "lee(t(c)o)de"
//! Explanation: "lee(t(co)de)" , "lee(t(c)ode)" would also be accepted.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: s = "a)b(c)d"
//! Output: "ab(c)d"
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: s = "))(("
//! Output: ""
//! Explanation: An empty string is also valid.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= s.length <= 10<sup>5</sup>`
//! * `s[i]` is either`'('` , `')'`, or lowercase English letter`.`
//!

#[solution]
impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut processed = "".to_string();

        let mut left_stack = vec![];
        for c in s.chars() {
            match c {
                '(' => {
                    left_stack.push(processed.len());
                    processed.push(c);
                }
                ')' => {
                    if !left_stack.is_empty() {
                        left_stack.pop();
                        processed.push(c);
                    }
                    // else drop
                }
                v => processed.push(v),
            }
        }

        if left_stack.is_empty() {
            return processed;
        }

        // If anything leftover in the stack, we need to get them removed.
        let mut result = String::new();
        let mut stack_index = 0;
        for (i, c) in processed.chars().enumerate() {
            if stack_index < left_stack.len() && i == left_stack[stack_index] {
                stack_index += 1;
            } else {
                result.push(c);
            }
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
    fn test_1249() {
        assert_eq!(
            "lee(t(c)o)de",
            Solution::min_remove_to_make_valid("lee(t(c)o)de)".into())
        );

        assert_eq!(
            "ab(c)d",
            Solution::min_remove_to_make_valid("a)b(c)d".into())
        );

        assert_eq!("", Solution::min_remove_to_make_valid("))((".into()));

        assert_eq!(
            "a(b(c)d)",
            Solution::min_remove_to_make_valid("(a(b(c)d)".into())
        );
    }
}
