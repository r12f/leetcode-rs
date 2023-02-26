//!
//! Problem #146: LRU Cache
//!
//! - Link: <https://leetcode.com/problems/lru-cache/>
//! - Discussions: <https://leetcode.com/problems/lru-cache/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Design a data structure that follows the constraints of a **[Least Recently Used (LRU) cache](https://en.wikipedia.org/wiki/Cache_replacement_policies#LRU)**.
//!
//! Implement the `LRUCache` class:
//!
//! * `LRUCache(int capacity)` Initialize the LRU cache with **positive** size `capacity`.
//! * `int get(int key)` Return the value of the `key` if the key exists, otherwise return `-1`.
//! * `void put(int key, int value)` Update the value of the `key` if the `key` exists. Otherwise, add the `key-value` pair to the cache. If the number of keys exceeds the `capacity` from this operation, **evict** the least recently used key.
//!
//! The functions `get` and `put` must each run in `O(1)` average time complexity.
//!
//! **Example 1:**
//!
//! ```
//! Input
//! ["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]
//! [[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
//! Output
//! [null, null, null, 1, null, -1, null, -1, 3, 4]
//!
//! Explanation
//! LRUCache lRUCache = new LRUCache(2);
//! lRUCache.put(1, 1); // cache is {1=1}
//! lRUCache.put(2, 2); // cache is {1=1, 2=2}
//! lRUCache.get(1);    // return 1
//! lRUCache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
//! lRUCache.get(2);    // returns -1 (not found)
//! lRUCache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
//! lRUCache.get(1);    // return -1 (not found)
//! lRUCache.get(3);    // return 3
//! lRUCache.get(4);    // return 4
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= capacity <= 3000`
//! * `0 <= key <= 10<sup>4</sup>`
//! * `0 <= value <= 10<sup>5</sup>`
//! * At most `2 * 10<sup>5</sup>` calls will be made to `get` and `put`.
//!

use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap, VecDeque},
    rc::Rc,
};

type RcCacheNode = Rc<RefCell<CacheNode>>;

struct CacheNode {
    key: i32,
    val: i32,
    prev: Option<RcCacheNode>,
    next: Option<RcCacheNode>,
}

impl CacheNode {
    pub fn new(key: i32, val: i32) -> RcCacheNode {
        let mut node = Rc::new(RefCell::new(CacheNode {
            key,
            val,
            prev: None,
            next: None,
        }));

        node
    }

    pub fn insert_after(self_: &RcCacheNode, node: RcCacheNode) {
        node.borrow_mut().prev = Some(self_.clone());
        node.borrow_mut().next = self_.borrow().next.clone();

        // Self and next can be the same node (list head), so we need to avoid the if let scope holding the reference here.
        let next = self_.borrow().next.clone();
        if let Some(next) = next {
            next.borrow_mut().prev = Some(node.clone());
        }

        self_.borrow_mut().next = Some(node);
    }

    pub fn remove(self_: &RcCacheNode) {
        if let Some(next) = self_.borrow().next.clone() {
            next.borrow_mut().prev = self_.borrow().prev.clone();
        }

        if let Some(prev) = self_.borrow().prev.clone() {
            prev.borrow_mut().next = self_.borrow().next.clone();
        }

        self_.borrow_mut().next = None;
        self_.borrow_mut().prev = None;
    }
}

struct LRUCache {
    capacity: usize,
    data: RcCacheNode,
    key_to_index_map: HashMap<i32, RcCacheNode>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        let head_node = CacheNode::new(i32::MIN, i32::MIN);
        head_node.borrow_mut().next = Some(head_node.clone());
        head_node.borrow_mut().prev = Some(head_node.clone());

        LRUCache {
            capacity: capacity as usize,
            data: head_node,
            key_to_index_map: HashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let p = match self.key_to_index_map.get(&key) {
            None => return -1,
            Some(v) => v.clone(),
        };

        let result = p.borrow().val;
        CacheNode::remove(&p);
        CacheNode::insert_after(&self.data, p);
        return result;
    }

    fn put(&mut self, key: i32, value: i32) {
        match self.key_to_index_map.entry(key) {
            Entry::Vacant(e) => {
                let node = CacheNode::new(key, value);
                CacheNode::insert_after(&self.data, node.clone());
                e.insert(node);
            }

            Entry::Occupied(e) => {
                let node = e.into_mut();
                node.borrow_mut().val = value;
                CacheNode::remove(&node);
                CacheNode::insert_after(&self.data, node.to_owned());
            }
        };

        if self.key_to_index_map.len() > self.capacity {
            let tail = self.data.borrow().prev.clone().unwrap();
            CacheNode::remove(&tail);
            self.key_to_index_map.remove(&tail.borrow().key);
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

//
// Tests
//
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_146() {
        let mut lRUCache = LRUCache::new(2);

        lRUCache.put(1, 1); // cache is {1=1}
        lRUCache.put(2, 2); // cache is {1=1, 2=2}
        assert_eq!(1, lRUCache.get(1)); // return 1

        lRUCache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
        assert_eq!(-1, lRUCache.get(2)); // returns -1 (not found)

        lRUCache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
        assert_eq!(-1, lRUCache.get(1)); // return -1 (not found)
        assert_eq!(3, lRUCache.get(3)); // return 3
        assert_eq!(4, lRUCache.get(4)); // return 4
    }

    #[test]
    fn test_146_2() {
        let mut lRUCache = LRUCache::new(2);

        lRUCache.put(1, 0);
        lRUCache.put(2, 2);
        assert_eq!(0, lRUCache.get(1));

        lRUCache.put(3, 3);
        assert_eq!(-1, lRUCache.get(2));

        lRUCache.put(4, 4);
        assert_eq!(-1, lRUCache.get(1));
        assert_eq!(3, lRUCache.get(3));
        assert_eq!(4, lRUCache.get(4));
    }
}
