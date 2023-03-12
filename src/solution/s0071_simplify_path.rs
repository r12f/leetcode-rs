//!
//! Problem #71: Simplify Path
//!
//! - Link: <https://leetcode.com/problems/simplify-path/>
//! - Discussions: <https://leetcode.com/problems/simplify-path/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given a string `path`, which is an **absolute path** (starting with a slash `'/'`) to a file or directory in a Unix-style file system, convert it to the simplified **canonical path**.
//!
//! In a Unix-style file system, a period `'.'` refers to the current directory, a double period `'..'` refers to the directory up a level, and any multiple consecutive slashes (i.e. `'//'`) are treated as a single slash `'/'`. For this problem, any other format of periods such as `'...'` are treated as file/directory names.
//!
//! The **canonical path** should have the following format:
//!
//! * The path starts with a single slash `'/'`.
//! * Any two directories are separated by a single slash `'/'`.
//! * The path does not end with a trailing `'/'`.
//! * The path only contains the directories on the path from the root directory to the target file or directory (i.e., no period `'.'` or double period `'..'`)
//!
//! Return *the simplified **canonical path***.
//!
//! **Example 1:**
//!
//! ```
//! Input: path = "/home/"
//! Output: "/home"
//! Explanation: Note that there is no trailing slash after the last directory name.
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: path = "/../"
//! Output: "/"
//! Explanation: Going one level up from the root directory is a no-op, as the root level is the highest level you can go.
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: path = "/home//foo/"
//! Output: "/home/foo"
//! Explanation: In the canonical path, multiple consecutive slashes are replaced by a single one.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `1 <= path.length <= 3000`
//! * `path` consists of English letters, digits, period `'.'`, slash `'/'` or `'_'`.
//! * `path` is a valid absolute Unix path.
//!

#[solution]
impl Solution {
    pub fn simplify_path(path: String) -> String {
        assert!(!path.is_empty());

        let mut path_segs = Vec::new();
        let mut seg = String::new();
        for c in path.chars() {
            match c {
                '/' => {
                    Solution::update_path_segs(&mut path_segs, &seg);
                    seg.clear();
                }

                v => seg.push(c),
            }
        }

        Solution::update_path_segs(&mut path_segs, &seg);

        if path_segs.is_empty() {
            return "/".into();
        }

        let mut result = String::new();
        for s in path_segs {
            result.push('/');
            result.push_str(&s);
        }

        result
    }

    fn update_path_segs(path_segs: &mut Vec<String>, path_seg: &str) {
        match path_seg {
            "" => return,
            "." => return,
            ".." => {
                path_segs.pop();
            }
            v => {
                path_segs.push(v.to_string());
            }
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
    fn test_71() {
        assert_eq!("/", Solution::simplify_path("/".into()));
        assert_eq!("/", Solution::simplify_path("//".into()));
        assert_eq!("/", Solution::simplify_path("/..".into()));
        assert_eq!("/", Solution::simplify_path("/../".into()));
        assert_eq!("/home", Solution::simplify_path("/home".into()));
        assert_eq!("/home", Solution::simplify_path("/home/".into()));
        assert_eq!("/home/foo", Solution::simplify_path("/home//foo".into()));
    }
}
