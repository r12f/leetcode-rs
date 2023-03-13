//!
//! Problem #227: Basic Calculator II
//!
//! - Link: <https://leetcode.com/problems/basic-calculator-ii/>
//! - Discussions: <https://leetcode.com/problems/basic-calculator-ii/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given a string `s` which represents an expression, *evaluate this expression and return its value*.
//!
//! The integer division should truncate toward zero.
//!
//! You may assume that the given expression is always valid. All intermediate results will be in the range of `[-2<sup>31</sup>, 2<sup>31</sup> - 1]`.
//!
//! **Note:** You are not allowed to use any built-in function which evaluates strings as mathematical expressions, such as `eval()`.
//!
//! **Example 1:**
//!
//! ```
//! Input: s = "3+2*2"
//! Output: 7
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: s = " 3/2 "
//! Output: 1
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: s = " 3+5 / 2 "
//! Output: 5
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= s.length <= 3 * 10<sup>5</sup>`
//! * `s` consists of integers and operators `('+', '-', '*', '/')` separated by some number of spaces.
//! * `s` represents **a valid expression**.
//! * All the integers in the expression are non-negative integers in the range `[0, 2<sup>31</sup> - 1]`.
//! * The answer is **guaranteed** to fit in a **32-bit integer**.
//!

use std::collections::HashMap;

#[solution]
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let ops_priority: HashMap<char, i32> = vec![('+', 1), ('-', 1), ('*', 2), ('/', 2)]
            .into_iter()
            .collect();

        let mut opends_stack = Vec::<i32>::new();
        let mut ops_stack = Vec::<char>::new();
        let mut opend: Option<i32> = None;

        for c in s.chars() {
            match c {
                ' ' => continue,
                '+' | '-' | '*' | '/' => {
                    if let Some(opend) = opend {
                        opends_stack.push(opend);
                    }
                    opend = None;

                    Self::push_op(&mut opends_stack, &mut ops_stack, &ops_priority, c);
                }
                '(' => {
                    ops_stack.push(c);
                }
                ')' => {
                    opends_stack.push(opend.unwrap());
                    opend = None;

                    Self::consume_parentheses(&mut opends_stack, &mut ops_stack, &ops_priority);
                }
                v => opend = Some(opend.unwrap_or(0) * 10 + (v as i32 - '0' as i32)),
            }
        }

        if let Some(opend) = opend {
            opends_stack.push(opend);
        }

        while !ops_stack.is_empty() {
            Self::consume_op(&mut opends_stack, &mut ops_stack);
        }

        *opends_stack.first().unwrap()
    }

    fn push_op(
        opends_stack: &mut Vec<i32>,
        ops_stack: &mut Vec<char>,
        ops_priority: &HashMap<char, i32>,
        op: char,
    ) {
        let op_priority = *ops_priority.get(&op).unwrap();
        Self::consume_higher_or_equal_priority_op(
            opends_stack,
            ops_stack,
            ops_priority,
            op_priority,
        );
        ops_stack.push(op);
    }

    fn consume_parentheses(
        opends_stack: &mut Vec<i32>,
        ops_stack: &mut Vec<char>,
        ops_priority: &HashMap<char, i32>,
    ) {
        Self::consume_higher_or_equal_priority_op(opends_stack, ops_stack, ops_priority, 0);
        let op = ops_stack.pop().unwrap();
        assert_eq!(op, '(');
    }

    fn consume_higher_or_equal_priority_op(
        opends_stack: &mut Vec<i32>,
        ops_stack: &mut Vec<char>,
        ops_priority: &HashMap<char, i32>,
        op_priority: i32,
    ) {
        while !ops_stack.is_empty() {
            let top_op = *ops_stack.last().unwrap();
            if top_op == '(' {
                break;
            }

            // If the last op has lower priority, then we don't know if we could execute it or not, so we have to wait.
            let top_op_priority = *ops_priority.get(&top_op).unwrap();
            if top_op_priority < op_priority {
                break;
            }

            Self::consume_op(opends_stack, ops_stack);
        }
    }

    fn consume_op(opends_stack: &mut Vec<i32>, ops_stack: &mut Vec<char>) {
        let opend2 = opends_stack.pop().unwrap();
        let opend1 = opends_stack.pop().unwrap();
        let top_op = ops_stack.pop().unwrap();

        let op_result = match top_op {
            '+' => opend1 + opend2,
            '-' => opend1 - opend2,
            '*' => opend1 * opend2,
            '/' => opend1 / opend2,
            _ => unreachable!(),
        };

        opends_stack.push(op_result);
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
    fn test_227() {
        assert_eq!(7, Solution::calculate("3+2*2".into()));
        assert_eq!(34, Solution::calculate("3+2*2+(4+5)*3".into()));
    }
}
