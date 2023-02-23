//!
//! Problem #49: Group Anagrams
//!
//! - Link: <https://leetcode.com/problems/group-anagrams/>
//! - Discussions: <https://leetcode.com/problems/group-anagrams/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given an array of strings `strs`, group **the anagrams** together. You can return the answer in **any order**.
//!
//! An **Anagram** is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
//!
//! **Example 1:**
//!
//! ```
//! Input: strs = ["eat","tea","tan","ate","nat","bat"]
//! Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: strs = [""]
//! Output: [[""]]
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: strs = ["a"]
//! Output: [["a"]]
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= strs.length <= 10<sup>4</sup>`
//! * `0 <= strs[i].length <= 100`
//! * `strs[i]` consists of lowercase English letters.
//!

use std::collections::{BTreeMap, HashMap};

#[solution]
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut id_to_str_map = BTreeMap::<Vec<i32>, Vec<String>>::new();
        //let mut id_to_str_map = HashMap::<Vec<i32>, Vec<String>>::new();

        for s in strs {
            let id = Solution::get_id(&s);
            id_to_str_map.entry(id).or_default().push(s);
        }

        id_to_str_map.into_values().collect::<Vec<Vec<String>>>()
    }

    fn get_id(s: &str) -> Vec<i32> {
        let mut id = vec![0; 26];

        for c in s.chars() {
            let n = (c as usize - 'a' as usize) as usize;
            id[n] += 1;
        }

        id
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
    fn test_49() {
        assert_eq!(
            lc_vecvec![["tan", "nat"], ["eat", "tea", "ate"], ["bat"]],
            Solution::group_anagrams(lc_strvec!["eat", "tea", "tan", "ate", "nat", "bat"])
        );
    }
}
