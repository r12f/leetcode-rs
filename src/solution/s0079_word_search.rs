//!
//! Problem #79: Word Search
//!
//! - Link: <https://leetcode.com/problems/word-search/>
//! - Discussions: <https://leetcode.com/problems/word-search/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given an `m x n` grid of characters `board` and a string `word`, return `true` *if* `word` *exists in the grid*.
//!
//! The word can be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once.
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/11/04/word2.jpg)
//!
//! ```
//! Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
//! Output: true
//!
//! ```
//!
//! **Example 2:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/11/04/word-1.jpg)
//!
//! ```
//! Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
//! Output: true
//!
//! ```
//!
//! **Example 3:**
//!
//! ![](https://assets.leetcode.com/uploads/2020/10/15/word3.jpg)
//!
//! ```
//! Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
//! Output: false
//!
//! ```
//!
//! **Constraints:**
//!
//! * `m == board.length`
//! * `n = board[i].length`
//! * `1 <= m, n <= 6`
//! * `1 <= word.length <= 15`
//! * `board` and `word` consists of only lowercase and uppercase English letters.
//!
//! **Follow up:** Could you use search pruning to make your solution faster with a larger `board`?
//!

#[solution]
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut visited = vec![vec![false; board[0].len()]; board.len()];
        for row in 0..board.len() {
            for col in 0..board[row].len() {
                if Solution::dfs(&board, row as i32, col as i32, &word, 0, &mut visited) {
                    return true;
                }
            }
        }

        false
    }

    fn dfs(
        board: &Vec<Vec<char>>,
        row: i32,
        col: i32,
        word: &str,
        char_index: usize,
        visited: &mut Vec<Vec<bool>>,
    ) -> bool {
        if board[row as usize][col as usize] as u8 != word.as_bytes()[char_index] {
            return false;
        }

        if char_index + 1 == word.len() {
            return true;
        }

        visited[row as usize][col as usize] = true;

        const DX: &[i32] = &[1, 0, -1, 0];
        const DY: &[i32] = &[0, 1, 0, -1];

        for i in 0..DX.len() {
            let new_row = row + DX[i];
            let new_col = col + DY[i];

            if new_row < 0
                || new_row >= board.len() as i32
                || new_col < 0
                || new_col >= board[0].len() as i32
            {
                continue;
            }

            if visited[new_row as usize][new_col as usize] {
                continue;
            }

            if Solution::dfs(board, new_row, new_col, word, char_index + 1, visited) {
                return true;
            }
        }

        visited[row as usize][col as usize] = false;

        false
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
    fn test_79() {
        assert_eq!(
            true,
            Solution::exist(
                lc_vecvec![
                    ['A', 'B', 'C', 'E'],
                    ['S', 'F', 'C', 'S'],
                    ['A', 'D', 'E', 'E']
                ],
                "ABCCED".into()
            )
        );
    }
}
