//!
//! Problem #8: String to Integer (atoi)
//!
//! - Link: <https://leetcode.com/problems/string-to-integer-atoi/>
//! - Discussions: <https://leetcode.com/problems/string-to-integer-atoi/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Implement the `myAtoi(string s)` function, which converts a string to a 32-bit signed integer (similar to C/C++'s `atoi` function).
//!
//! The algorithm for `myAtoi(string s)` is as follows:
//!
//! 1. Read in and ignore any leading whitespace.
//! 2. Check if the next character (if not already at the end of the string) is `'-'` or `'+'`. Read this character in if it is either. This determines if the final result is negative or positive respectively. Assume the result is positive if neither is present.
//! 3. Read in next the characters until the next non-digit character or the end of the input is reached. The rest of the string is ignored.
//! 4. Convert these digits into an integer (i.e. `"123" -> 123`, `"0032" -> 32`). If no digits were read, then the integer is `0`. Change the sign as necessary (from step 2).
//! 5. If the integer is out of the 32-bit signed integer range `[-2<sup>31</sup>, 2<sup>31</sup> - 1]`, then clamp the integer so that it remains in the range. Specifically, integers less than `-2<sup>31</sup>` should be clamped to `-2<sup>31</sup>`, and integers greater than `2<sup>31</sup> - 1` should be clamped to `2<sup>31</sup> - 1`.
//! 6. Return the integer as the final result.
//!
//! **Note:**
//!
//! * Only the space character `' '` is considered a whitespace character.
//! * **Do not ignore** any characters other than the leading whitespace or the rest of the string after the digits.
//!
//! **Example 1:**
//!
//! ```
//! Input: s = "42"
//! Output: 42
//! Explanation: The underlined characters are what is read in, the caret is the current reader position.
//! Step 1: "42" (no characters read because there is no leading whitespace)
//!          ^
//! Step 2: "42" (no characters read because there is neither a '-' nor '+')
//!          ^
//! Step 3: "42" ("42" is read in)
//!            ^
//! The parsed integer is 42.
//! Since 42 is in the range [-231, 231 - 1], the final result is 42.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: s = "   -42"
//! Output: -42
//! Explanation:
//! Step 1: "   -42" (leading whitespace is read and ignored)
//!             ^
//! Step 2: "   -42" ('-' is read, so the result should be negative)
//!              ^
//! Step 3: "   -42" ("42" is read in)
//!                ^
//! The parsed integer is -42.
//! Since -42 is in the range [-231, 231 - 1], the final result is -42.
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: s = "4193 with words"
//! Output: 4193
//! Explanation:
//! Step 1: "4193 with words" (no characters read because there is no leading whitespace)
//!          ^
//! Step 2: "4193 with words" (no characters read because there is neither a '-' nor '+')
//!          ^
//! Step 3: "4193 with words" ("4193" is read in; reading stops because the next character is a non-digit)
//!              ^
//! The parsed integer is 4193.
//! Since 4193 is in the range [-231, 231 - 1], the final result is 4193.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `0 <= s.length <= 200`
//! * `s` consists of English letters (lower-case and upper-case), digits (`0-9`), `' '`, `'+'`, `'-'`, and `'.'`.
//!

#[derive(PartialEq, PartialOrd)]
enum ParsingState {
    ParseLeadingSpace,
    ParseLeadingZero,
    ParseValue,
}

#[solution]
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut parsing_state = ParsingState::ParseLeadingSpace;
        let mut sign: i32 = -1;
        let mut result: i32 = 0;

        for c in s.chars() {
            match c {
                ' ' => {
                    if parsing_state != ParsingState::ParseLeadingSpace {
                        break;
                    }
                }
                '+' | '-' => {
                    if parsing_state != ParsingState::ParseLeadingSpace {
                        break;
                    }

                    if c == '-' {
                        sign = 1;
                    }

                    parsing_state = ParsingState::ParseLeadingZero;
                }
                '0'..='9' => {
                    if parsing_state == ParsingState::ParseLeadingSpace {
                        parsing_state = ParsingState::ParseLeadingZero;
                    }

                    let n = c as i32 - '0' as i32;
                    let mut added: Option<i32> = None;
                    if n != 0 {
                        parsing_state = ParsingState::ParseValue;
                        added = result.checked_mul(10).and_then(|v| v.checked_add(-n));
                    } else {
                        // Ignore leading 0s.
                        if parsing_state == ParsingState::ParseValue {
                            added = result.checked_mul(10);
                        } else {
                            continue;
                        }
                    }

                    result = match added {
                        Some(v) => {
                            if sign == -1 {
                                // overflow.
                                if v == i32::MIN {
                                    return i32::MAX;
                                }
                            }

                            v
                        }
                        None => {
                            if sign == -1 {
                                return i32::MAX;
                            } else {
                                return i32::MIN;
                            }
                        }
                    }
                }
                _ => break,
            }
        }

        result * sign
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
    fn test_8() {
        assert_eq!(42, Solution::my_atoi("42".into()));
        assert_eq!(42, Solution::my_atoi("+42".into()));
        assert_eq!(-42, Solution::my_atoi("-42".into()));
        assert_eq!(-42, Solution::my_atoi("   -42".into()));
        assert_eq!(42, Solution::my_atoi("  +42".into()));
        assert_eq!(0, Solution::my_atoi("   --42".into()));
        assert_eq!(1337, Solution::my_atoi("  1337 words".into()));
        assert_eq!(0, Solution::my_atoi("words".into()));
        assert_eq!(0, Solution::my_atoi("-words".into()));
        assert_eq!(0, Solution::my_atoi("- words".into()));
        assert_eq!(-1337, Solution::my_atoi("-1337words".into()));
        assert_eq!(-2147483648, Solution::my_atoi("-91283472332".into()));
        assert_eq!(2147483647, Solution::my_atoi("2147483648".into()));
    }
}
