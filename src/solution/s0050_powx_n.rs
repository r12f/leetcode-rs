//!
//! Problem #50: Pow(x, n)
//!
//! - Link: <https://leetcode.com/problems/powx-n/>
//! - Discussions: <https://leetcode.com/problems/powx-n/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Implement [pow(x, n)](http://www.cplusplus.com/reference/valarray/pow/), which calculates `x` raised to the power `n` (i.e., `x<sup>n</sup>`).
//!
//! **Example 1:**
//!
//! ```
//! Input: x = 2.00000, n = 10
//! Output: 1024.00000
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: x = 2.10000, n = 3
//! Output: 9.26100
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: x = 2.00000, n = -2
//! Output: 0.25000
//! Explanation: 2-2 = 1/22 = 1/4 = 0.25
//!
//! ```
//!
//! **Constraints:**
//!
//! * `-100.0 < x < 100.0`
//! * `-2<sup>31</sup> <= n <= 2<sup>31</sup>-1`
//! * `n` is an integer.
//! * `-10<sup>4</sup> <= x<sup>n</sup> <= 10<sup>4</sup>`
//!

#[solution]
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let result = Self::my_pow_inner(x, n);
        (result * 100000.0).round() / 100000.0
    }

    fn my_pow_inner(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }

        if n % 2 != 0 {
            if n > 0 {
                let child = Self::my_pow_inner(x, n - 1);
                child * x
            } else {
                let child = Self::my_pow_inner(x, n + 1);
                child / x
            }
        } else {
            let child = Self::my_pow_inner(x, n / 2);
            child * child
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
    fn test_50() {
        assert_eq!(1024.0, Solution::my_pow(2.0, 10));
        assert_eq!(9.261, Solution::my_pow(2.1, 3));
        assert_eq!(0.25, Solution::my_pow(2.0, -2));
    }
}
