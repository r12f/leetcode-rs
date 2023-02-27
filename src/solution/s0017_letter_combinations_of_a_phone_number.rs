//!
//! Problem #17: Letter Combinations of a Phone Number
//!
//! - Link: <https://leetcode.com/problems/letter-combinations-of-a-phone-number/>
//! - Discussions: <https://leetcode.com/problems/letter-combinations-of-a-phone-number/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given a string containing digits from `2-9` inclusive, return all possible letter combinations that the number could represent. Return the answer in **any order**.
//!
//! A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
//!
//! ![](https://assets.leetcode.com/uploads/2022/03/15/1200px-telephone-keypad2svg.png)
//!
//! **Example 1:**
//!
//! ```
//! Input: digits = "23"
//! Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: digits = ""
//! Output: []
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: digits = "2"
//! Output: ["a","b","c"]
//!
//! ```
//!
//! **Constraints:**
//!
//! * `0 <= digits.length <= 4`
//! * `digits[i]` is a digit in the range `['2', '9']`.
//!

#[solution]
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut results = Vec::new();
        if digits.is_empty() {
            return results;
        }

        let mappings: Vec<Vec<char>> = vec![
            vec![],
            vec![],
            "abc".chars().collect(),
            "def".chars().collect(),
            "ghi".chars().collect(),
            "jkl".chars().collect(),
            "mno".chars().collect(),
            "pqrs".chars().collect(),
            "tuv".chars().collect(),
            "wxyz".chars().collect(),
        ];

        let mut result = String::new();
        Solution::dfs(&mut results, &mut result, &mappings, &digits, 0);

        results
    }

    fn dfs(
        results: &mut Vec<String>,
        result: &mut String,
        mappings: &Vec<Vec<char>>,
        digits: &str,
        n: usize,
    ) {
        if n == digits.len() {
            results.push(result.clone());
            return;
        }

        let digit = digits.as_bytes()[n] - ('0' as u8);
        for c in &mappings[digit as usize] {
            result.push(*c);
            Solution::dfs(results, result, mappings, digits, n + 1);
            result.pop();
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
    fn test_17() {
        assert_eq!(
            lc_strvec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"],
            Solution::letter_combinations("23".into())
        );
    }
}
