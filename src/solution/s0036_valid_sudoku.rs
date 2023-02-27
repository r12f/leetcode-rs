//!
//! Problem #36: Valid Sudoku
//!
//! - Link: <https://leetcode.com/problems/valid-sudoku/>
//! - Discussions: <https://leetcode.com/problems/valid-sudoku/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Determine if a `9 x 9` Sudoku board is valid. Only the filled cells need to be validated **according to the following rules**:
//!
//! 1. Each row must contain the digits `1-9` without repetition.
//! 2. Each column must contain the digits `1-9` without repetition.
//! 3. Each of the nine `3 x 3` sub-boxes of the grid must contain the digits `1-9` without repetition.
//!
//! **Note:**
//!
//! * A Sudoku board (partially filled) could be valid but is not necessarily solvable.
//! * Only the filled cells need to be validated according to the mentioned rules.
//!
//! **Example 1:**
//!
//! ![](https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png)
//!
//! ```
//! Input: board =
//! [["5","3",".",".","7",".",".",".","."]
//! ,["6",".",".","1","9","5",".",".","."]
//! ,[".","9","8",".",".",".",".","6","."]
//! ,["8",".",".",".","6",".",".",".","3"]
//! ,["4",".",".","8",".","3",".",".","1"]
//! ,["7",".",".",".","2",".",".",".","6"]
//! ,[".","6",".",".",".",".","2","8","."]
//! ,[".",".",".","4","1","9",".",".","5"]
//! ,[".",".",".",".","8",".",".","7","9"]]
//! Output: true
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: board =
//! [["8","3",".",".","7",".",".",".","."]
//! ,["6",".",".","1","9","5",".",".","."]
//! ,[".","9","8",".",".",".",".","6","."]
//! ,["8",".",".",".","6",".",".",".","3"]
//! ,["4",".",".","8",".","3",".",".","1"]
//! ,["7",".",".",".","2",".",".",".","6"]
//! ,[".","6",".",".",".",".","2","8","."]
//! ,[".",".",".","4","1","9",".",".","5"]
//! ,[".",".",".",".","8",".",".","7","9"]]
//! Output: false
//! Explanation: Same as Example 1, except with the 5 in the top left corner being modified to 8. Since there are two 8's in the top left 3x3 sub-box, it is invalid.
//!
//! ```
//!
//! **Constraints:**
//!
//! * `board.length == 9`
//! * `board[i].length == 9`
//! * `board[i][j]` is a digit `1-9` or `'.'`.
//!

#[solution]
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row_nums = vec![vec![false; 10]; 9];
        let mut col_nums = vec![vec![false; 10]; 9];
        let mut block_nums = vec![vec![false; 10]; 9];

        for row in 0..board.len() {
            for col in 0..board[row].len() {
                if board[row][col] == '.' {
                    continue;
                }

                let n = board[row][col] as usize - '0' as usize;
                if row_nums[row][n] {
                    return false;
                }
                row_nums[row][n] = true;

                if col_nums[col][n] {
                    return false;
                }
                col_nums[col][n] = true;

                let block = (row / 3) * 3 + col / 3;
                if block_nums[block][n] {
                    return false;
                }
                block_nums[block][n] = true;
            }
        }

        true
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
    fn test_36() {
        assert_eq!(
            true,
            Solution::is_valid_sudoku(lc_vecvec![
                ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                ['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ])
        );
    }
}
