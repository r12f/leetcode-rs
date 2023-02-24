//!
//! Problem #61: Rotate List
//!
//! - Link: <https://leetcode.com/problems/rotate-list/>
//! - Discussions: <https://leetcode.com/problems/rotate-list/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given the `head` of a linked list, rotate the list to the right by `k` places.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/11/13/rotate1.jpg)
//!
//! ```
//! Input: head = [1,2,3,4,5], k = 2
//! Output: [4,5,1,2,3]
//!
//! ```
//!
//! **Example 2:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/11/13/roate2.jpg)
//!
//! ```
//! Input: head = [0,1,2], k = 4
//! Output: [2,0,1]
//!
//! ```
//!
//! **Constraints:**
//!
//! * The number of nodes in the list is in the range `[0, 500]`.
//! * `-100 <= Node.val <= 100`
//! * `0 <= k <= 2 * 10<sup>9</sup>`
//!

#[solution]
impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut head = ListNode { val: 0, next: head };
        let mut head_pointer = &mut head;

        let mut fast = unsafe { &mut *(head_pointer as *mut ListNode) };
        let mut slow = unsafe { &mut *(head_pointer as *mut ListNode) };

        // k can be really large, so need to fix the number k first.
        let mut len = 0;
        while fast.next.is_some() {
            fast = fast.next.as_mut().unwrap().as_mut();
            len += 1;
        }
        k = k % len;
        if k == 0 {
            let ListNode { next: result, .. } = head;
            return result;
        }

        // Move fast pointer
        fast = unsafe { &mut *(head_pointer as *mut ListNode) };
        drop(head_pointer);
        for _ in 0..k {
            fast = fast.next.as_mut().unwrap().as_mut();
        }

        // Move both pointer, until fast reaches end
        while fast.next.is_some() {
            fast = fast.next.as_mut().unwrap().as_mut();
            slow = slow.next.as_mut().unwrap().as_mut();
        }

        let mut temp = None;
        std::mem::swap(&mut temp, &mut slow.next);
        std::mem::swap(&mut temp, &mut head.next);
        std::mem::swap(&mut temp, &mut fast.next);

        let ListNode { next: result, .. } = head;
        return result;
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
    fn test_61() {
        assert_eq!(lc_list![], Solution::rotate_right(lc_list![], 0));
        assert_eq!(lc_list![], Solution::rotate_right(lc_list![], 2));
        assert_eq!(lc_list![1], Solution::rotate_right(lc_list![1], 2));

        assert_eq!(
            lc_list![4, 5, 1, 2, 3],
            Solution::rotate_right(lc_list![1, 2, 3, 4, 5], 2)
        );

        assert_eq!(
            lc_list![4, 5, 1, 2, 3],
            Solution::rotate_right(lc_list![1, 2, 3, 4, 5], 7)
        );
    }
}
