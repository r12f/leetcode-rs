//!
//! Problem #24: Swap Nodes in Pairs
//!
//! - Link: <https://leetcode.com/problems/swap-nodes-in-pairs/>
//! - Discussions: <https://leetcode.com/problems/swap-nodes-in-pairs/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given a linked list, swap every two adjacent nodes and return its head. You must solve the problem without modifying the values in the list's nodes (i.e., only nodes themselves may be changed.)
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/10/03/swap_ex1.jpg)
//!
//! ```
//! Input: head = [1,2,3,4]
//! Output: [2,1,4,3]
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: head = []
//! Output: []
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: head = [1]
//! Output: [1]
//!
//! ```
//!
//! **Constraints:**
//!
//! * The number of nodes in the list is in the range `[0, 100]`.
//! * `0 <= Node.val <= 100`
//!

#[solution]
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut head = ListNode { val: 0, next: head };

        let mut prev = &mut head;
        while prev.next.is_some() {
            if prev.next.as_ref().unwrap().next.is_none() {
                break;
            }

            // Swap
            let mut temp = None;
            std::mem::swap(
                &mut temp,
                &mut prev.next.as_mut().unwrap().next.as_mut().unwrap().next,
            );
            std::mem::swap(&mut temp, &mut prev.next.as_mut().unwrap().next);
            std::mem::swap(&mut temp, &mut prev.next);
            prev.next.as_mut().unwrap().next = temp;

            // Move to next
            prev = prev.next.as_mut().unwrap().next.as_mut().unwrap().as_mut();
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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_24() {
        assert_eq!(
            lc_list![2, 1, 4, 3],
            Solution::swap_pairs(lc_list![1, 2, 3, 4])
        );
    }
}
