//!
//! Problem #28: Find the Index of the First Occurrence in a String
//!
//! - Link: <https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/>
//! - Discussions: <https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given two strings `needle` and `haystack`, return the index of the first occurrence of `needle` in `haystack`, or `-1` if `needle` is not part of `haystack`.
//!
//! **Example 1:**
//!
//! ```
//! Input: haystack = "sadbutsad", needle = "sad"
//! Output: 0
//! Explanation: "sad" occurs at index 0 and 6.
//! The first occurrence is at index 0, so we return 0.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: haystack = "leetcode", needle = "leeto"
//! Output: -1
//! Explanation: "leeto" did not occur in "leetcode", so we return -1.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= haystack.length, needle.length <= 10<sup>4</sup>`
//! * `haystack` and `needle` consist of only lowercase English characters.
//!

#[solution]
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        // Seriously? I am not going to implement KMP during my interview session.
        match haystack.find(&needle) {
            None => -1,
            Some(v) => v as i32,
        }
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
    fn test_28() {}
}
