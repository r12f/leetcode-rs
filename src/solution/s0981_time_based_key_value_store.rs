//!
//! Problem #981: Time Based Key-Value Store
//!
//! - Link: <https://leetcode.com/problems/time-based-key-value-store/>
//! - Discussions: <https://leetcode.com/problems/time-based-key-value-store/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Design a time-based key-value data structure that can store multiple values for the same key at different time stamps and retrieve the key's value at a certain timestamp.
//!
//! Implement the `TimeMap` class:
//!
//! * `TimeMap()` Initializes the object of the data structure.
//! * `void set(String key, String value, int timestamp)` Stores the key `key` with the value `value `at the given time `timestamp`.
//! * `String get(String key, int timestamp)` Returns a value such that `set` was called previously, with `timestamp_prev <= timestamp`. If there are multiple such values, it returns the value associated with the largest `timestamp_prev`. If there are no values, it returns `""`.
//!
//! **Example 1:**
//!
//! ```
//! Input
//! ["TimeMap", "set", "get", "get", "set", "get", "get"]
//! [[], ["foo", "bar", 1], ["foo", 1], ["foo", 3], ["foo", "bar2", 4], ["foo", 4], ["foo", 5]]
//! Output
//! [null, null, "bar", "bar", null, "bar2", "bar2"]
//!
//! Explanation
//! TimeMap timeMap = new TimeMap();
//! timeMap.set("foo", "bar", 1);  // store the key "foo" and value "bar" along with timestamp = 1.
//! timeMap.get("foo", 1);         // return "bar"
//! timeMap.get("foo", 3);         // return "bar", since there is no value corresponding to foo at timestamp 3 and timestamp 2, then the only value is at timestamp 1 is "bar".
//! timeMap.set("foo", "bar2", 4); // store the key "foo" and value "bar2" along with timestamp = 4.
//! timeMap.get("foo", 4);         // return "bar2"
//! timeMap.get("foo", 5);         // return "bar2"
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= key.length, value.length <= 100`
//! * `key` and `value` consist of lowercase English letters and digits.
//! * `1 <= timestamp <= 10<sup>7</sup>`
//! * All the timestamps `timestamp` of `set` are strictly increasing.
//! * At most `2 * 10<sup>5</sup>` calls will be made to `set` and `get`.
//!

use std::collections::{hash_map::Entry, BTreeMap, HashMap};

struct TimeMap {
    data: HashMap<String, BTreeMap<i32, String>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        TimeMap {
            data: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.data.entry(key).or_default().insert(timestamp, value);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        match self.data.get(&key) {
            None => return "".into(),
            Some(time_to_value_map) => {
                let mut it = time_to_value_map.range(..(timestamp + 1));
                let value = it.next_back();
                match value {
                    None => return "".into(),
                    Some((ts, v)) => return v.to_string(),
                }
            }
        }
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */

//
// Tests
//
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_981() {
        let mut timeMap = TimeMap::new();
        timeMap.set("foo".into(), "bar".into(), 1); // store the key "foo" and value "bar" along with timestamp = 1.
        assert_eq!("bar".to_string(), timeMap.get("foo".into(), 1));
        assert_eq!("bar".to_string(), timeMap.get("foo".into(), 3));

        timeMap.set("foo".into(), "bar2".into(), 4); // store the key "foo" and value "bar2" along with timestamp = 4.
        assert_eq!("bar2".to_string(), timeMap.get("foo".into(), 4));
        assert_eq!("bar2".to_string(), timeMap.get("foo".into(), 5));
    }
}
