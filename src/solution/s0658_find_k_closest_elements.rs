//!
//! Problem #658: Find K Closest Elements
//!
//! - Link: <https://leetcode.com/problems/find-k-closest-elements/>
//! - Discussions: <https://leetcode.com/problems/find-k-closest-elements/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given a **sorted** integer array `arr`, two integers `k` and `x`, return the `k` closest integers to `x` in the array. The result should also be sorted in ascending order.
//!
//! An integer `a` is closer to `x` than an integer `b` if:
//!
//! * `|a - x| < |b - x|`, or
//! * `|a - x| == |b - x|` and `a < b`
//!
//! **Example 1:**
//!
//! ```
//! Input: arr = [1,2,3,4,5], k = 4, x = 3
//! Output: [1,2,3,4]
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: arr = [1,2,3,4,5], k = 4, x = -1
//! Output: [1,2,3,4]
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= k <= arr.length`
//! * `1 <= arr.length <= 10<sup>4</sup>`
//! * `arr` is sorted in **ascending** order.
//! * `-10<sup>4</sup> <= arr[i], x <= 10<sup>4</sup>`
//!

/// To find the closest k element, we are actually trying to find a lower bound `left`, where Distance(left, x) == Distance(x, left + k)
#[solution]
impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        assert!(k as usize <= arr.len());

        if k as usize == arr.len() {
            return arr;
        }

        let mut left = 0;
        let mut right = arr.len() - k as usize;

        while left < right {
            let mid = (left + right) / 2;
            if x - arr[mid] <= arr[mid + k as usize] - x {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        arr[left..(left + k as usize)].to_vec()
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
    fn test_658() {
        // assert_eq!(vec![1], Solution::find_closest_elements(vec![1], 1, 1));
        assert_eq!(
            vec![1, 2, 3, 4],
            Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3)
        );
    }
}
