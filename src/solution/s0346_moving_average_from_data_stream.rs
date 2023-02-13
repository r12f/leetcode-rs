//!
//! Problem #346: Moving Average from Data Stream
//!
//! - Link: <https://leetcode.com/problems/moving-average-from-data-stream/>
//! - Discussions: <https://leetcode.com/problems/moving-average-from-data-stream/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given a stream of integers and a window size, calculate the moving average of all integers in the sliding window.
//!
//! Implement the `MovingAverage` class:
//!
//! * `MovingAverage(int size)` Initializes the object with the size of the window `size`.
//! * `double next(int val)` Returns the moving average of the last `size` values of the stream.
//!
//! **Example 1:**
//!
//! ```
//! Input
//! ["MovingAverage", "next", "next", "next", "next"]
//! [[3], [1], [10], [3], [5]]
//! Output
//! [null, 1.0, 5.5, 4.66667, 6.0]
//!
//! Explanation
//! MovingAverage movingAverage = new MovingAverage(3);
//! movingAverage.next(1); // return 1.0 = 1 / 1
//! movingAverage.next(10); // return 5.5 = (1 + 10) / 2
//! movingAverage.next(3); // return 4.66667 = (1 + 10 + 3) / 3
//! movingAverage.next(5); // return 6.0 = (10 + 3 + 5) / 3
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= size <= 1000`
//! * `-10<sup>5</sup> <= val <= 10<sup>5</sup>`
//! * At most `10<sup>4</sup>` calls will be made to `next`.
//!

use std::collections::VecDeque;

struct MovingAverage {
    sum: i64,
    nums: VecDeque<i32>,
    size: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {
    fn new(size: i32) -> Self {
        MovingAverage {
            sum: 0,
            nums: VecDeque::new(),
            size: size as usize,
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        if self.size == self.nums.len() {
            let old_val = self.nums.pop_front().unwrap();
            self.sum -= old_val as i64;
        }

        self.nums.push_back(val);
        self.sum += val as i64;

        return self.sum as f64 / self.nums.len() as f64;
    }
}

/**
 * Your MovingAverage object will be instantiated and called as such:
 * let obj = MovingAverage::new(size);
 * let ret_1: f64 = obj.next(val);
 */

//
// Tests
//
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_346() {
        let mut ma = MovingAverage::new(3);
        assert_eq!(1.0, ma.next(1));
        assert_eq!(5.5, ma.next(10));
        assert_eq!(4.666666666666667, ma.next(3));
        assert_eq!(6.0, ma.next(5));
    }
}
