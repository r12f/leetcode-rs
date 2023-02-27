//!
//! Problem #295: Find Median from Data Stream
//!
//! - Link: <https://leetcode.com/problems/find-median-from-data-stream/>
//! - Discussions: <https://leetcode.com/problems/find-median-from-data-stream/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! The **median** is the middle value in an ordered integer list. If the size of the list is even, there is no middle value, and the median is the mean of the two middle values.
//!
//! * For example, for `arr = [2,3,4]`, the median is `3`.
//! * For example, for `arr = [2,3]`, the median is `(2 + 3) / 2 = 2.5`.
//!
//! Implement the MedianFinder class:
//!
//! * `MedianFinder()` initializes the `MedianFinder` object.
//! * `void addNum(int num)` adds the integer `num` from the data stream to the data structure.
//! * `double findMedian()` returns the median of all elements so far. Answers within `10<sup>-5</sup>` of the actual answer will be accepted.
//!
//! **Example 1:**
//!
//! ```
//! Input
//! ["MedianFinder", "addNum", "addNum", "findMedian", "addNum", "findMedian"]
//! [[], [1], [2], [], [3], []]
//! Output
//! [null, null, null, 1.5, null, 2.0]
//!
//! Explanation
//! MedianFinder medianFinder = new MedianFinder();
//! medianFinder.addNum(1);    // arr = [1]
//! medianFinder.addNum(2);    // arr = [1, 2]
//! medianFinder.findMedian(); // return 1.5 (i.e., (1 + 2) / 2)
//! medianFinder.addNum(3);    // arr[1, 2, 3]
//! medianFinder.findMedian(); // return 2.0
//!
//! ```
//!
//! **Constraints:**
//!
//! * `-10<sup>5</sup> <= num <= 10<sup>5</sup>`
//! * There will be at least one element in the data structure before calling `findMedian`.
//! * At most `5 * 10<sup>4</sup>` calls will be made to `addNum` and `findMedian`.
//!
//! **Follow up:**
//!
//! * If all integer numbers from the stream are in the range `[0, 100]`, how would you optimize your solution?
//! * If `99%` of all integer numbers from the stream are in the range `[0, 100]`, how would you optimize your solution?
//!

use std::{cmp::Reverse, collections::BinaryHeap};

struct MedianFinder {
    smaller: BinaryHeap<i32>,
    larger: BinaryHeap<std::cmp::Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            smaller: BinaryHeap::new(),
            larger: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        // If odd number, median is always tracked as top of larger side.
        // If even number, we peak both and average them.
        let smaller_top = self.smaller.peek().unwrap_or(&i32::MAX).clone();
        let larger_top = self.larger.peek().unwrap_or(&Reverse(i32::MIN)).0;

        if num >= larger_top {
            self.larger.push(Reverse(num));
        } else {
            self.smaller.push(num);
        }

        while self.larger.len() < self.smaller.len() {
            let temp = self.smaller.pop().unwrap();
            self.larger.push(Reverse(temp));
        }

        while self.larger.len() > self.smaller.len() + 1 {
            let temp = self.larger.pop().unwrap();
            self.smaller.push(temp.0);
        }
    }

    fn find_median(&self) -> f64 {
        if self.larger.is_empty() {
            return 0.0;
        }

        if self.larger.len() > self.smaller.len() {
            return self.larger.peek().unwrap().0 as f64;
        }

        (self.smaller.peek().unwrap() + self.larger.peek().unwrap().0) as f64 / 2.0
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

//
// Tests
//
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_295() {
        let mut medianFinder = MedianFinder::new();
        medianFinder.add_num(1);
        medianFinder.add_num(2);
        assert!((medianFinder.find_median() - 1.5).abs() < 1E5f64);
        medianFinder.add_num(3);
        assert!((medianFinder.find_median() - 2.0).abs() < 1E5f64);
    }

    #[test]
    fn test_295_2() {
        let mut medianFinder = MedianFinder::new();
        medianFinder.add_num(-1);
        assert!((medianFinder.find_median() - -1.0).abs() < 1E5f64);
        medianFinder.add_num(-2);
        assert!((medianFinder.find_median() - -1.5).abs() < 1E5f64);
        medianFinder.add_num(-3);
        assert!((medianFinder.find_median() - -2.0).abs() < 1E5f64);
        medianFinder.add_num(-4);
        assert!((medianFinder.find_median() - -2.5).abs() < 1E5f64);
        medianFinder.add_num(-5);
        assert!((medianFinder.find_median() - -3.0).abs() < 1E5f64);
    }
}
