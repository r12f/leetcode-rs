//!
//! Problem #408: Valid Word Abbreviation
//!
//! - Link: <https://leetcode.com/problems/valid-word-abbreviation/>
//! - Discussions: <https://leetcode.com/problems/valid-word-abbreviation/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! A string can be **abbreviated** by replacing any number of **non-adjacent**, **non-empty** substrings with their lengths. The lengths **should not** have leading zeros.
//!
//! For example, a string such as `"substitution"` could be abbreviated as (but not limited to):
//!
//! * `"s10n"` (`"s ubstitutio n"`)
//! * `"sub4u4"` (`"sub stit u tion"`)
//! * `"12"` (`"substitution"`)
//! * `"su3i1u2on"` (`"su bst i t u ti on"`)
//! * `"substitution"` (no substrings replaced)
//!
//! The following are **not valid** abbreviations:
//!
//! * `"s55n"` (`"s ubsti tutio n"`, the replaced substrings are adjacent)
//! * `"s010n"` (has leading zeros)
//! * `"s0ubstitution"` (replaces an empty substring)
//!
//! Given a string `word` and an abbreviation `abbr`, return *whether the string **matches** the given abbreviation*.
//!
//! A **substring** is a contiguous **non-empty** sequence of characters within a string.
//!
//! **Example 1:**
//!
//! ```
//! Input: word = "internationalization", abbr = "i12iz4n"
//! Output: true
//! Explanation: The word "internationalization" can be abbreviated as "i12iz4n" ("i nternational iz atio n").
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: word = "apple", abbr = "a2e"
//! Output: false
//! Explanation: The word "apple" cannot be abbreviated as "a2e".
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= word.length <= 20`
//! * `word` consists of only lowercase English letters.
//! * `1 <= abbr.length <= 10`
//! * `abbr` consists of lowercase English letters and digits.
//! * All the integers in `abbr` will fit in a 32-bit integer.
//!

#[solution]
impl Solution {
    pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
        let word_bytes = word.as_bytes();
        let abbr_bytes = abbr.as_bytes();

        let mut word_index = 0;
        let mut abbr_index = 0;

        while word_index < word_bytes.len() && abbr_index < abbr_bytes.len() {
            let mut abbr_char = abbr_bytes[abbr_index] as char;

            // Leading 0s.
            if abbr_char == '0' {
                return false;
            }

            if abbr_char >= '1' && abbr_char <= '9' {
                let mut num = 0;

                while abbr_char >= '0' && abbr_char <= '9' {
                    num = num * 10 + abbr_char as usize - '0' as usize;
                    abbr_index += 1;

                    if (abbr_index >= abbr_bytes.len()) {
                        break;
                    }

                    abbr_char = abbr_bytes[abbr_index] as char;
                }

                if word_index + num > word_bytes.len() {
                    return false;
                } else if word_index + num == word_bytes.len() {
                    if abbr_index == abbr_bytes.len() {
                        return true;
                    } else {
                        return false;
                    }
                } else if (abbr_index == abbr_bytes.len()) {
                    return false;
                }

                word_index += num;
            }

            // Test is char the same
            if word_bytes[word_index] != abbr_bytes[abbr_index] {
                return false;
            }

            word_index += 1;
            abbr_index += 1;
        }

        if word_index < word_bytes.len() || abbr_index < abbr_bytes.len() {
            return false;
        }

        true
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
    fn test_408() {
        assert!(Solution::valid_word_abbreviation(
            "internationalization".to_string(),
            "i12iz4n".to_string(),
        ));

        assert!(Solution::valid_word_abbreviation(
            "internationalization".to_string(),
            "i5a11o1".to_string(),
        ));

        assert!(!Solution::valid_word_abbreviation(
            "hi".to_string(),
            "2i".to_string(),
        ));

        assert!(!Solution::valid_word_abbreviation(
            "hi".to_string(),
            "1".to_string(),
        ));
    }
}
