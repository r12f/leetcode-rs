//!
//! Problem #721: Accounts Merge
//!
//! - Link: <https://leetcode.com/problems/accounts-merge/>
//! - Discussions: <https://leetcode.com/problems/accounts-merge/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given a list of `accounts` where each element `accounts[i]` is a list of strings, where the first element `accounts[i][0]` is a name, and the rest of the elements are **emails** representing emails of the account.
//!
//! Now, we would like to merge these accounts. Two accounts definitely belong to the same person if there is some common email to both accounts. Note that even if two accounts have the same name, they may belong to different people as people could have the same name. A person can have any number of accounts initially, but all of their accounts definitely have the same name.
//!
//! After merging the accounts, return the accounts in the following format: the first element of each account is the name, and the rest of the elements are emails **in sorted order**. The accounts themselves can be returned in **any order**.
//!
//! **Example 1:**
//!
//! ```
//! Input: accounts = [["John","johnsmith@mail.com","john_newyork@mail.com"],["John","johnsmith@mail.com","john00@mail.com"],["Mary","mary@mail.com"],["John","johnnybravo@mail.com"]]
//! Output: [["John","john00@mail.com","john_newyork@mail.com","johnsmith@mail.com"],["Mary","mary@mail.com"],["John","johnnybravo@mail.com"]]
//! Explanation:
//! The first and second John's are the same person as they have the common email "johnsmith@mail.com".
//! The third John and Mary are different people as none of their email addresses are used by other accounts.
//! We could return these lists in any order, for example the answer [['Mary', 'mary@mail.com'], ['John', 'johnnybravo@mail.com'],
//! ['John', 'john00@mail.com', 'john_newyork@mail.com', 'johnsmith@mail.com']] would still be accepted.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: accounts = [["Gabe","Gabe0@m.co","Gabe3@m.co","Gabe1@m.co"],["Kevin","Kevin3@m.co","Kevin5@m.co","Kevin0@m.co"],["Ethan","Ethan5@m.co","Ethan4@m.co","Ethan0@m.co"],["Hanzo","Hanzo3@m.co","Hanzo1@m.co","Hanzo0@m.co"],["Fern","Fern5@m.co","Fern1@m.co","Fern0@m.co"]]
//! Output: [["Ethan","Ethan0@m.co","Ethan4@m.co","Ethan5@m.co"],["Gabe","Gabe0@m.co","Gabe1@m.co","Gabe3@m.co"],["Hanzo","Hanzo0@m.co","Hanzo1@m.co","Hanzo3@m.co"],["Kevin","Kevin0@m.co","Kevin3@m.co","Kevin5@m.co"],["Fern","Fern0@m.co","Fern1@m.co","Fern5@m.co"]]
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= accounts.length <= 1000`
//! * `2 <= accounts[i].length <= 10`
//! * `1 <= accounts[i][j].length <= 30`
//! * `accounts[i][0]` consists of English letters.
//! * `accounts[i][j] (for j > 0)` is a valid email.
//!

use std::collections::{btree_map::Entry, BTreeMap, BTreeSet, HashMap};

#[solution]
impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut account_parent: Vec<Option<usize>> = vec![None; accounts.len()];

        // Build union set
        let mut email_to_index_map = BTreeMap::<String, usize>::new();
        for (mut account_index, account) in accounts.iter().enumerate() {
            for email in account.iter().skip(1) {
                match email_to_index_map.entry(email.clone()) {
                    Entry::Occupied(entry) => {
                        let mut root = *entry.into_mut();
                        while let Some(v) = account_parent[root] {
                            if root == v {
                                break;
                            }
                            root = v;
                        }
                        account_parent[account_index] = Some(root);
                        account_index = root;
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(account_index);
                    }
                };
            }
        }

        // Craft result
        let mut result = BTreeMap::<usize, Vec<String>>::new();
        for (email, index) in email_to_index_map.into_iter() {
            let mut root = index;
            while let Some(v) = account_parent[root] {
                if root == v {
                    break;
                }
                root = v;
            }

            match result.entry(root) {
                Entry::Occupied(entry) => {
                    entry.into_mut().push(email);
                }
                Entry::Vacant(entry) => {
                    entry.insert(vec![accounts[root][0].clone()]).push(email);
                }
            }
        }

        result.into_values().collect::<Vec<Vec<String>>>()
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
    fn test_721() {
        assert_eq!(
            lc_vecvec![
                [
                    "John".to_string(),
                    "john00@mail.com".to_string(),
                    "john_newyork@mail.com".to_string(),
                    "johnsmith@mail.com".to_string(),
                ],
                ["Mary".to_string(), "mary@mail.com".to_string()],
                ["John".to_string(), "johnnybravo@mail.com".to_string()]
            ],
            Solution::accounts_merge(lc_vecvec![
                [
                    "John".to_string(),
                    "johnsmith@mail.com".to_string(),
                    "john_newyork@mail.com".to_string()
                ],
                [
                    "John".to_string(),
                    "johnsmith@mail.com".to_string(),
                    "john00@mail.com".to_string()
                ],
                ["Mary".to_string(), "mary@mail.com".to_string()],
                ["John".to_string(), "johnnybravo@mail.com".to_string()]
            ])
        );

        // assert_eq!(
        //     lc_vecvec![
        //         [
        //             "John".to_string(),
        //             "john00@mail.com".to_string(),
        //             "john_newyork@mail.com".to_string(),
        //             "johnsmith@mail.com".to_string(),
        //         ],
        //         ["Mary".to_string(), "mary@mail.com".to_string()],
        //         ["John".to_string(), "johnnybravo@mail.com".to_string()]
        //     ],
        //     Solution::accounts_merge(lc_vecvec![
        //         [
        //             "David".to_string(),
        //             "David0@m.co".to_string(),
        //             "David4@m.co".to_string(),
        //             "David3@m.co".to_string()
        //         ],
        //         [
        //             "David".to_string(),
        //             "David5@m.co".to_string(),
        //             "David5@m.co".to_string(),
        //             "David0@m.co".to_string()
        //         ],
        //         [
        //             "David".to_string(),
        //             "David1@m.co".to_string(),
        //             "David4@m.co".to_string(),
        //             "David0@m.co".to_string()
        //         ],
        //         [
        //             "David".to_string(),
        //             "David0@m.co".to_string(),
        //             "David1@m.co".to_string(),
        //             "David3@m.co".to_string()
        //         ],
        //         [
        //             "David".to_string(),
        //             "David4@m.co".to_string(),
        //             "David1@m.co".to_string(),
        //             "David3@m.co".to_string()
        //         ]
        //     ])
        // );
    }
}
