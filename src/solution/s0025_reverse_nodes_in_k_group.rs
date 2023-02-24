//!
//! Problem #25: Reverse Nodes in k-Group
//!
//! - Link: <https://leetcode.com/problems/reverse-nodes-in-k-group/>
//! - Discussions: <https://leetcode.com/problems/reverse-nodes-in-k-group/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given the `head` of a linked list, reverse the nodes of the list `k` at a time, and return *the modified list*.
//!
//! `k` is a positive integer and is less than or equal to the length of the linked list. If the number of nodes is not a multiple of `k` then left-out nodes, in the end, should remain as it is.
//!
//! You may not alter the values in the list's nodes, only nodes themselves may be changed.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/10/03/reverse_ex1.jpg)
//!
//! ```
//! Input: head = [1,2,3,4,5], k = 2
//! Output: [2,1,4,3,5]
//!
//! ```
//!
//! **Example 2:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/10/03/reverse_ex2.jpg)
//!
//! ```
//! Input: head = [1,2,3,4,5], k = 3
//! Output: [3,2,1,4,5]
//!
//! ```
//!
//! **Constraints:**
//!
//! * The number of nodes in the list is `n`.
//! * `1 <= k <= n <= 5000`
//! * `0 <= Node.val <= 1000`
//!
//! **Follow-up:** Can you solve the problem in `O(1)` extra memory space?
//!

#[solution]
impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k < 2 {
            return head;
        }

        let mut head = ListNode { val: 0, next: head };

        let mut first_pointer = &mut head.next;
        let mut right = unsafe { &mut *(first_pointer as *mut Option<Box<ListNode>>) };

        let mut prev = &mut head;

        'outer: loop {
            if right.is_none() {
                break;
            }

            for _ in 0..(k - 1) {
                right = &mut right.as_mut().unwrap().as_mut().next;
                if right.is_none() {
                    break 'outer;
                }
            }

            let mut left = None;
            std::mem::swap(&mut left, &mut prev.next);

            let mut next_start = None;
            match right.as_mut() {
                Some(v) => std::mem::swap(&mut next_start, &mut v.as_mut().next),
                None => (),
            }

            let mut new_last: &mut ListNode =
                unsafe { &mut *(left.as_mut().unwrap().as_mut() as *mut ListNode) };
            let first = Solution::reverse_list(left, next_start);
            prev.next = first;

            prev = new_last;
            right = unsafe { &mut *(&mut prev.next as *mut Option<Box<ListNode>>) };
        }

        let ListNode { next: result, .. } = head;
        return result;
    }

    fn reverse_list<'a>(
        mut cur: Option<Box<ListNode>>,
        mut next_start: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut prev = next_start;
        let mut is_first_node = true;

        while cur.is_some() {
            let mut cur_node = cur.as_mut().unwrap();

            let mut next = None;
            std::mem::swap(&mut cur_node.as_mut().next, &mut next);
            cur_node.as_mut().next = prev;

            prev = cur;
            cur = next;
        }

        return prev;
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
    fn test_25() {
        // assert_eq!(lc_list![1], Solution::reverse_k_group(lc_list![1], 2));
        // assert_eq!(lc_list![2, 1], Solution::reverse_k_group(lc_list![1, 2], 2));

        // assert_eq!(
        //     lc_list![2, 1, 3],
        //     Solution::reverse_k_group(lc_list![1, 2, 3], 2)
        // );

        // assert_eq!(
        //     lc_list![1, 2, 3],
        //     Solution::reverse_k_group(lc_list![1, 2, 3], 5)
        // );

        // assert_eq!(
        //     lc_list![1, 2, 3, 4, 5],
        //     Solution::reverse_k_group(lc_list![1, 2, 3, 4, 5], 1)
        // );

        assert_eq!(
            lc_list![3, 2, 1, 4, 5],
            Solution::reverse_k_group(lc_list![1, 2, 3, 4, 5], 3)
        );
    }
}
