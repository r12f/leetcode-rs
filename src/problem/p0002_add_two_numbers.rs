//!
//! Problem #2: Add Two Numbers
//!
//! - Link: <https://leetcode.com/problems/add-two-numbers/>
//! - Discussions: <https://leetcode.com/problems/add-two-numbers/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! You are given two **non-empty** linked lists representing two non-negative integers. The digits are stored in **reverse order**, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
//!
//! You may assume the two numbers do not contain any leading zero, except the number 0 itself.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/10/02/addtwonumber1.jpg)
//!
//! ```
//! Input: l1 = [2,4,3], l2 = [5,6,4]
//! Output: [7,0,8]
//! Explanation: 342 + 465 = 807.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: l1 = [0], l2 = [0]
//! Output: [0]
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
//! Output: [8,9,9,9,0,0,0,1]
//!
//! ```
//!
//! **Constraints:**
//!
//! * The number of nodes in each linked list is in the range `[1, 100]`.
//! * `0 <= Node.val <= 9`
//! * It is guaranteed that the list represents a number that does not have leading zeros.
//!

#[solution]
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut node = &mut head;

        let mut l1 = &l1;
        let mut l2 = &l2;

        let mut carry_over = 0;
        while (l1.is_some() || l2.is_some()) {
            let mut l1_val = 0;
            let mut l2_val = 0;

            if let Some(l1_node) = l1 {
                l1_val = l1_node.val;
                l1 = &l1_node.next;
            }

            if let Some(l2_node) = l2 {
                l2_val = l2_node.val;
                l2 = &l2_node.next;
            }

            let mut sum = l1_val + l2_val + carry_over;
            if sum >= 10 {
                carry_over = 1;
                sum -= 10;
            } else {
                carry_over = 0;
            }

            let mut digit_node = Some(Box::new(ListNode::new(sum)));
            node.next = digit_node;
            node = node.next.as_mut().unwrap().as_mut();
        }

        if carry_over > 0 {
            node.next = Some(Box::new(ListNode::new(carry_over)));
        }

        let ListNode { next: result, .. } = head;
        result
    }
}

//
// Tests
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        lc_list_assert_eq!(Solution::add_two_numbers(lc_list!(), lc_list!()), ());
        lc_list_assert_eq!(Solution::add_two_numbers(lc_list!(1), lc_list!()), (1));
        lc_list_assert_eq!(Solution::add_two_numbers(lc_list!(1), lc_list!(1)), (2));
        lc_list_assert_eq!(
            Solution::add_two_numbers(lc_list!(2, 4, 3), lc_list!(5, 6, 4)),
            (7, 0, 8)
        );

        lc_list_assert_eq!(
            Solution::add_two_numbers(lc_list!(9, 9, 9, 9, 9, 9, 9), lc_list!(9, 9, 9, 9)),
            (8, 9, 9, 9, 0, 0, 0, 1)
        );
    }
}
