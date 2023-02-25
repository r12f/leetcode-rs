//!
//! Problem #127: Word Ladder
//!
//! - Link: <https://leetcode.com/problems/word-ladder/>
//! - Discussions: <https://leetcode.com/problems/word-ladder/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! A **transformation sequence** from word `beginWord` to word `endWord` using a dictionary `wordList` is a sequence of words `beginWord -> s<sub>1</sub> -> s<sub>2</sub> -> ... -> s<sub>k</sub>` such that:
//!
//! * Every adjacent pair of words differs by a single letter.
//! * Every `s<sub>i</sub>` for `1 <= i <= k` is in `wordList`. Note that `beginWord` does not need to be in `wordList`.
//! * `s<sub>k</sub> == endWord`
//!
//! Given two words, `beginWord` and `endWord`, and a dictionary `wordList`, return *the **number of words** in the **shortest transformation sequence** from* `beginWord` *to* `endWord`*, or* `0` *if no such sequence exists.*
//!
//! **Example 1:**
//!
//! ```
//! Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
//! Output: 5
//! Explanation: One shortest transformation sequence is "hit" -> "hot" -> "dot" -> "dog" -> cog", which is 5 words long.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]
//! Output: 0
//! Explanation: The endWord "cog" is not in wordList, therefore there is no valid transformation sequence.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= beginWord.length <= 10`
//! * `endWord.length == beginWord.length`
//! * `1 <= wordList.length <= 5000`
//! * `wordList[i].length == beginWord.length`
//! * `beginWord`, `endWord`, and `wordList[i]` consist of lowercase English letters.
//! * `beginWord != endWord`
//! * All the words in `wordList` are **unique**.
//!

use std::collections::{HashMap, HashSet, VecDeque};

#[solution]
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut word_map: HashMap<String, Vec<String>> = HashMap::new();

        for word in word_list.iter() {
            for i in 0..word.len() {
                let mut word_pattern = word.clone();
                word_pattern.replace_range(i..i + 1, "*");
                word_map.entry(word_pattern).or_default().push(word.clone());
            }
        }

        let mut visited: HashSet<String> = HashSet::new();
        let mut queue = VecDeque::<(String, i32)>::new();
        queue.push_back((begin_word, 1));

        while let Some((mut word, step)) = queue.pop_front() {
            if word == end_word {
                return step;
            }

            if visited.contains(&word) {
                continue;
            }
            visited.insert(word.clone());

            for i in 0..word.len() {
                let original_byte;
                {
                    let mut word_bytes = unsafe { word.as_bytes_mut() };
                    original_byte = word_bytes[i];
                    word_bytes[i] = '*' as u8;
                }

                if let Some(next_words) = word_map.get(&word) {
                    for next_word in next_words {
                        queue.push_back((next_word.clone(), step + 1));
                    }
                }

                {
                    let mut word_bytes = unsafe { word.as_bytes_mut() };
                    word_bytes[i] = original_byte;
                }
            }
        }

        return 0;
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
    fn test_127() {
        assert_eq!(
            5,
            Solution::ladder_length(
                "hit".into(),
                "cog".into(),
                lc_strvec!["hot", "dot", "dog", "lot", "log", "cog"]
            )
        );

        assert_eq!(
            0,
            Solution::ladder_length(
                "hit".into(),
                "cog".into(),
                lc_strvec!["hot", "dot", "dog", "lot", "log"]
            )
        );
    }
}
