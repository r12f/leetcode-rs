//!
//! Problem #460: LFU Cache
//!
//! - Link: <https://leetcode.com/problems/lfu-cache/>
//! - Discussions: <https://leetcode.com/problems/lfu-cache/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Design and implement a data structure for a [Least Frequently Used (LFU)](https://en.wikipedia.org/wiki/Least_frequently_used) cache.
//!
//! Implement the `LFUCache` class:
//!
//! * `LFUCache(int capacity)` Initializes the object with the `capacity` of the data structure.
//! * `int get(int key)` Gets the value of the `key` if the `key` exists in the cache. Otherwise, returns `-1`.
//! * `void put(int key, int value)` Update the value of the `key` if present, or inserts the `key` if not already present. When the cache reaches its `capacity`, it should invalidate and remove the **least frequently used** key before inserting a new item. For this problem, when there is a **tie** (i.e., two or more keys with the same frequency), the **least recently used** `key` would be invalidated.
//!
//! To determine the least frequently used key, a **use counter** is maintained for each key in the cache. The key with the smallest **use counter** is the least frequently used key.
//!
//! When a key is first inserted into the cache, its **use counter** is set to `1` (due to the `put` operation). The **use counter** for a key in the cache is incremented either a `get` or `put` operation is called on it.
//!
//! The functions `get` and `put` must each run in `O(1)` average time complexity.
//!
//! **Example 1:**
//!
//! ```
//! Input
//! ["LFUCache", "put", "put", "get", "put", "get", "get", "put", "get", "get", "get"]
//! [[2], [1, 1], [2, 2], [1], [3, 3], [2], [3], [4, 4], [1], [3], [4]]
//! Output
//! [null, null, null, 1, null, -1, 3, null, -1, 3, 4]
//!
//! Explanation
//! // cnt(x) = the use counter for key x
//! // cache=[] will show the last used order for tiebreakers (leftmost element is  most recent)
//! LFUCache lfu = new LFUCache(2);
//! lfu.put(1, 1);   // cache=[1,_], cnt(1)=1
//! lfu.put(2, 2);   // cache=[2,1], cnt(2)=1, cnt(1)=1
//! lfu.get(1);      // return 1
//!                  // cache=[1,2], cnt(2)=1, cnt(1)=2
//! lfu.put(3, 3);   // 2 is the LFU key because cnt(2)=1 is the smallest, invalidate 2.
//!                  // cache=[3,1], cnt(3)=1, cnt(1)=2
//! lfu.get(2);      // return -1 (not found)
//! lfu.get(3);      // return 3
//!                  // cache=[3,1], cnt(3)=2, cnt(1)=2
//! lfu.put(4, 4);   // Both 1 and 3 have the same cnt, but 1 is LRU, invalidate 1.
//!                  // cache=[4,3], cnt(4)=1, cnt(3)=2
//! lfu.get(1);      // return -1 (not found)
//! lfu.get(3);      // return 3
//!                  // cache=[3,4], cnt(4)=1, cnt(3)=3
//! lfu.get(4);      // return 4
//!                  // cache=[4,3], cnt(4)=2, cnt(3)=3
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= capacity <= 10<sup>4</sup>`
//! * `0 <= key <= 10<sup>5</sup>`
//! * `0 <= value <= 10<sup>9</sup>`
//! * At most `2 * 10<sup>5</sup>` calls will be made to `get` and `put`.
//!
//!
/*
```
use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap, VecDeque},
    rc::Rc,
};

type RcCacheNode = Rc<RefCell<CacheNode>>;

struct CacheNode {
    key: i32,
    val: i32,
    hit_count: i32,
    prev: Option<RcCacheNode>,
    next: Option<RcCacheNode>,
}

impl CacheNode {
    pub fn new(key: i32, val: i32) -> RcCacheNode {
        let mut node = Rc::new(RefCell::new(CacheNode {
            key,
            val,
            hit_count: 0,
            prev: None,
            next: None,
        }));

        node
    }

    pub fn insert_before(self_: &RcCacheNode, node: RcCacheNode) {
        node.borrow_mut().next = Some(self_.clone());
        node.borrow_mut().prev = self_.borrow().prev.clone();

        // Self and prev can be the same node (list head), so we need to avoid the if let scope holding the reference here.
        let prev = self_.borrow().prev.clone();
        if let Some(prev) = prev {
            prev.borrow_mut().next = Some(node.clone());
        }

        self_.borrow_mut().prev = Some(node);
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

struct LFUCache {
    capacity: usize,
    data: RcCacheNode,
    key_to_node_map: HashMap<i32, RcCacheNode>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    fn new(capacity: i32) -> Self {
        let head_node = CacheNode::new(i32::MIN, i32::MIN);
        head_node.borrow_mut().hit_count = i32::MAX;
        head_node.borrow_mut().next = Some(head_node.clone());
        head_node.borrow_mut().prev = Some(head_node.clone());

        LFUCache {
            capacity: capacity as usize,
            data: head_node,
            key_to_node_map: HashMap::new(),
        }
    }

    fn get(&self, key: i32) -> i32 {
        match self.key_to_node_map.get(&key) {
            None => -1,
            Some(node) => {
                let result = node.borrow().val;
                LFUCache::move_up(&node);
                result
            }
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        // Found, then update.
        if let Some(node) = self.key_to_node_map.get(&key) {
            node.borrow_mut().val = value;
            LFUCache::move_up(&node);
            return;
        }

        // Not found, create a new one.
        self.evict_tail_if_full();

        let node = CacheNode::new(key, value);
        self.key_to_node_map.insert(key, node.clone());

        CacheNode::insert_before(&self.data, node.clone());
        LFUCache::move_up(&node);
    }

    fn move_up(node: &RcCacheNode) {
        node.borrow_mut().hit_count += 1;

        let mut prev_node = node.borrow().prev.clone().unwrap();
        if prev_node.borrow().hit_count > node.borrow().hit_count {
            return;
        }

        // Remove the current node and find the right place to insert.
        CacheNode::remove(node);

        loop {
            let prev_prev_node = prev_node.borrow().prev.clone().unwrap();
            if (prev_prev_node.borrow().hit_count > node.borrow().hit_count) {
                break;
            }

            prev_node = prev_prev_node;
        }

        CacheNode::insert_before(&prev_node, node.clone());
    }

    fn evict_tail_if_full(&mut self) {
        if self.key_to_node_map.len() < self.capacity {
            return;
        }

        let tail = self.data.borrow().prev.clone().unwrap();
        if Rc::ptr_eq(&self.data, &tail) {
            return;
        }

        // println!("Evicting ({}, {})", tail.borrow().key, tail.borrow().val);
        CacheNode::remove(&tail);
        self.key_to_node_map.remove(&tail.borrow().key);
    }
}
```
*/

use std::collections::{BTreeMap, HashMap, HashSet};

struct CacheNode {
    value: i32,
    freq: i32,
}

struct LFUCache {
    capacity: usize,
    data: HashMap<i32, CacheNode>,
    frequency_to_key_map: BTreeMap<i32, HashSet<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    fn new(capacity: i32) -> Self {
        LFUCache {
            capacity: capacity as usize,
            data: HashMap::new(),
            frequency_to_key_map: BTreeMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let node = match self.data.get_mut(&key) {
            None => return -1,
            Some(v) => v,
        };

        LFUCache::update_node_freq(key, node, &mut self.frequency_to_key_map);

        return node.value;
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.data.get_mut(&key) {
            node.value = value;
            LFUCache::update_node_freq(key, node, &mut self.frequency_to_key_map);
            return;
        }

        // Evict
        if self.data.len() >= self.capacity {
            let mut lfe = self.frequency_to_key_map.first_entry().unwrap();
            let lf_key = *lfe.get().iter().next().unwrap();
            lfe.get_mut().remove(&lf_key);
            self.data.remove(&lf_key);
            println!("Evict key: {}", lf_key);
        }

        // Create
        self.data.insert(key, CacheNode { value, freq: 1 });
        self.frequency_to_key_map.entry(1).or_default().insert(key);
    }

    fn update_node_freq(
        key: i32,
        node: &mut CacheNode,
        frequency_to_key_map: &mut BTreeMap<i32, HashSet<i32>>,
    ) {
        let same_freq_keys = frequency_to_key_map.get_mut(&node.freq).unwrap();
        same_freq_keys.remove(&key);

        if same_freq_keys.is_empty() {
            drop(same_freq_keys);
            frequency_to_key_map.remove(&node.freq);
        }

        node.freq += 1;

        frequency_to_key_map
            .entry(node.freq)
            .or_default()
            .insert(key);
    }
}

/**
 * Your LFUCache object will be instantiated and called as such:
 * let obj = LFUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

/**
 * Your LFUCache object will be instantiated and called as such:
 * let obj = LFUCache::new(capacity);
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
    fn test_460() {
        let mut cache = LFUCache::new(2);

        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(1, cache.get(1));

        cache.put(3, 3);
        assert_eq!(-1, cache.get(2));
        assert_eq!(3, cache.get(3));

        cache.put(4, 4);
        assert_eq!(-1, cache.get(1));
        assert_eq!(3, cache.get(3));
        assert_eq!(4, cache.get(4));
    }
}
