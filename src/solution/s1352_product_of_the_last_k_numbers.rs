//!
//! Problem #1352: Product of the Last K Numbers
//!
//! - Link: <https://leetcode.com/problems/product-of-the-last-k-numbers/>
//! - Discussions: <https://leetcode.com/problems/product-of-the-last-k-numbers/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Design an algorithm that accepts a stream of integers and retrieves the product of the last `k` integers of the stream.
//!
//! Implement the `ProductOfNumbers` class:
//!
//! * `ProductOfNumbers()` Initializes the object with an empty stream.
//! * `void add(int num)` Appends the integer `num` to the stream.
//! * `int getProduct(int k)` Returns the product of the last `k` numbers in the current list. You can assume that always the current list has at least `k` numbers.
//!
//! The test cases are generated so that, at any time, the product of any contiguous sequence of numbers will fit into a single 32-bit integer without overflowing.
//!
//! **Example:**
//!
//! ```
//! Input
//! ["ProductOfNumbers","add","add","add","add","add","getProduct","getProduct","getProduct","add","getProduct"]
//! [[],[3],[0],[2],[5],[4],[2],[3],[4],[8],[2]]
//!
//! Output
//! [null,null,null,null,null,null,20,40,0,null,32]
//!
//! Explanation
//! ProductOfNumbers productOfNumbers = new ProductOfNumbers();
//! productOfNumbers.add(3);        // [3]
//! productOfNumbers.add(0);        // [3,0]
//! productOfNumbers.add(2);        // [3,0,2]
//! productOfNumbers.add(5);        // [3,0,2,5]
//! productOfNumbers.add(4);        // [3,0,2,5,4]
//! productOfNumbers.getProduct(2); // return 20. The product of the last 2 numbers is 5 * 4 = 20
//! productOfNumbers.getProduct(3); // return 40. The product of the last 3 numbers is 2 * 5 * 4 = 40
//! productOfNumbers.getProduct(4); // return 0. The product of the last 4 numbers is 0 * 2 * 5 * 4 = 0
//! productOfNumbers.add(8);        // [3,0,2,5,4,8]
//! productOfNumbers.getProduct(2); // return 32. The product of the last 2 numbers is 4 * 8 = 32
//!
//! ```
//!
//! **Constraints:**
//!
//! * `0 <= num <= 100`
//! * `1 <= k <= 4 * 10<sup>4</sup>`
//! * At most `4 * 10<sup>4</sup>` calls will be made to `add` and `getProduct`.
//! * The product of the stream at any point in time will fit in a **32-bit** integer.
//!

struct ProductOfNumbers {
    products: Vec<i32>,
    last_zero_index: Option<usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {
    fn new() -> Self {
        ProductOfNumbers {
            products: vec![1],
            last_zero_index: None,
        }
    }

    fn add(&mut self, num: i32) {
        if num == 0 {
            self.last_zero_index = Some(self.products.len());
            self.products.push(1);
            return;
        }

        let product = self.products.last().unwrap() * num;
        self.products.push(product);
    }

    fn get_product(&self, k: i32) -> i32 {
        let first_index = self.products.len() - k as usize;
        if self.has_zero(first_index) {
            return 0;
        }

        let all_product = *self.products.last().unwrap();
        let prev_product = self.products[first_index - 1];
        all_product / prev_product
    }

    fn has_zero(&self, first_index: usize) -> bool {
        match self.last_zero_index {
            None => false,
            Some(index) => first_index <= index,
        }
    }
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */

//
// Tests
//
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_1352() {}
}
