//!
//! Problem #23: Merge k Sorted Lists
//!
//! - Link: <https://leetcode.com/problems/merge-k-sorted-lists/>
//! - Discussions: <https://leetcode.com/problems/merge-k-sorted-lists/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! You are given an array of `k` linked-lists `lists`, each linked-list is sorted in ascending order.
//!
//! *Merge all the linked-lists into one sorted linked-list and return it.*
//!
//! **Example 1:**
//!
//! ```
//! Input: lists = [[1,4,5],[1,3,4],[2,6]]
//! Output: [1,1,2,3,4,4,5,6]
//! Explanation: The linked-lists are:
//! [
//!   1->4->5,
//!   1->3->4,
//!   2->6
//! ]
//! merging them into one sorted list:
//! 1->1->2->3->4->4->5->6
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: lists = []
//! Output: []
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: lists = [[]]
//! Output: []
//!
//! ```
//!
//! **Constraints:**
//!
//! * `k == lists.length`
//! * `0 <= k <= 10<sup>4</sup>`
//! * `0 <= lists[i].length <= 500`
//! * `-10<sup>4</sup> <= lists[i][j] <= 10<sup>4</sup>`
//! * `lists[i]` is sorted in **ascending order**.
//! * The sum of `lists[i].length` will not exceed `10<sup>4</sup>`.
//!

use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct PendingNode(Box<ListNode>, usize);

impl Ord for PendingNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.as_ref().val.cmp(&self.0.val)
    }
}

impl PartialOrd for PendingNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[solution]
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut pq = BinaryHeap::<PendingNode>::new();
        let mut list_heads = vec![ListNode::new(0); lists.len()];

        for (i, list) in lists.into_iter().enumerate() {
            list_heads[i].next = list;
        }

        let mut result = ListNode::new(0);
        for (i, list_head) in list_heads.iter_mut().enumerate() {
            Solution::push_next_node_to_queue_if_any(&mut pq, list_head, i);
        }

        let mut result_tail = &mut result;
        while let Some(PendingNode(node, list_index)) = pq.pop() {
            result_tail.next = Some(node);
            result_tail = result_tail.next.as_mut().unwrap().as_mut();
            Solution::push_next_node_to_queue_if_any(
                &mut pq,
                &mut list_heads[list_index],
                list_index,
            );
        }

        let ListNode { next: result, .. } = result;
        result
    }

    fn push_next_node_to_queue_if_any(
        pq: &mut BinaryHeap<PendingNode>,
        list_head: &mut ListNode,
        list_index: usize,
    ) {
        if (list_head.next.is_none()) {
            return;
        }

        let mut temp = None;
        std::mem::swap(&mut temp, &mut list_head.next.as_mut().unwrap().next);
        std::mem::swap(&mut temp, &mut list_head.next);

        if let Some(node) = temp {
            pq.push(PendingNode(node, list_index));
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
    fn test_23() {
        assert_eq!(
            lc_list![1, 1, 2, 3, 4, 4, 5, 6],
            Solution::merge_k_lists(lc_listvec![[1, 4, 5], [1, 3, 4], [2, 6]])
        );
    }
}
