//!
//! Problem #1570: Dot Product of Two Sparse Vectors
//!
//! - Link: <https://leetcode.com/problems/dot-product-of-two-sparse-vectors/>
//! - Discussions: <https://leetcode.com/problems/dot-product-of-two-sparse-vectors/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given two sparse vectors, compute their dot product.
//!
//! Implement class `SparseVector`:
//!
//! * `SparseVector(nums)` Initializes the object with the vector `nums`
//! * `dotProduct(vec)` Compute the dot product between the instance of *SparseVector* and `vec`
//!
//! A **sparse vector** is a vector that has mostly zero values, you should store the sparse vector **efficiently** and compute the dot product between two *SparseVector*.
//!
//! **Follow up:** What if only one of the vectors is sparse?
//!
//! **Example 1:**
//!
//! ```
//! Input: nums1 = [1,0,0,2,3], nums2 = [0,3,0,4,0]
//! Output: 8
//! Explanation: v1 = SparseVector(nums1) , v2 = SparseVector(nums2)
//! v1.dotProduct(v2) = 1*0 + 0*3 + 0*0 + 2*4 + 3*0 = 8
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: nums1 = [0,1,0,0,0], nums2 = [0,0,0,0,2]
//! Output: 0
//! Explanation: v1 = SparseVector(nums1) , v2 = SparseVector(nums2)
//! v1.dotProduct(v2) = 0*0 + 1*0 + 0*0 + 0*0 + 0*2 = 0
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: nums1 = [0,1,0,0,2,0,0], nums2 = [1,0,0,0,3,0,4]
//! Output: 6
//!
//! ```
//!
//! **Constraints:**
//!
//! * `n == nums1.length == nums2.length`
//! * `1 <= n <= 10^5`
//! * `0 <= nums1[i], nums2[i] <= 100`
//!

struct SparseVector {
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SparseVector {
    fn new(nums: Vec<i32>) -> Self {
        SparseVector { nums }
    }

    /*
     * cannot believe this beats 100%.
    fn dot_product(&self, other: SparseVector) -> i32 {
        let mut my_index = 0;
        let mut other_index = 0;

        let mut sum = 0;
        while my_index < self.nums.len() {
            if self.nums[my_index] == 0 {
                while my_index < self.nums.len() && self.nums[my_index] == 0 {
                    my_index += 1;
                    other_index += 1;
                }

                continue;
            } else if other.nums[other_index] == 0 {
                while my_index < self.nums.len() && other.nums[other_index] == 0 {
                    my_index += 1;
                    other_index += 1;
                }
            } else {
                sum += self.nums[my_index] * other.nums[other_index];
                my_index += 1;
                other_index += 1;
            }
        }

        sum
    }
    */

    /*
     * This beats 50%..... what is this median level problem asking??
     */
    fn dot_product(&self, other: SparseVector) -> i32 {
        let mut my_index = 0;
        let mut other_index = 0;

        let mut sum = 0;
        while my_index < self.nums.len() {
            if self.nums[my_index] != 0 && other.nums[other_index] != 0 {
                sum += self.nums[my_index] * other.nums[other_index];
            }

            my_index += 1;
            other_index += 1;
        }

        sum
    }
}

/**
 * Your SparseVector object will be instantiated and called as such:
 * let v1 = SparseVector::new(nums1);
 * let v2 = SparseVector::new(nums2);
 * let ans = v1.dot_product(v2);
 */

//
// Tests
//
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_1570() {
        assert_eq!(
            8,
            SparseVector::new(vec![1, 0, 0, 2, 3])
                .dot_product(SparseVector::new(vec![0, 3, 0, 4, 0]))
        );
    }
}
