//!
//! Problem #200: Number of Islands
//!
//! - Link: <https://leetcode.com/problems/number-of-islands/>
//! - Discussions: <https://leetcode.com/problems/number-of-islands/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! Given an `m x n` 2D binary grid `grid` which represents a map of `'1'`s (land) and `'0'`s (water), return *the number of islands*.
//!
//! An **island** is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.
//!
//! **Example 1:**
//!
//! ```
//! Input: grid = [
//!   ["1","1","1","1","0"],
//!   ["1","1","0","1","0"],
//!   ["1","1","0","0","0"],
//!   ["0","0","0","0","0"]
//! ]
//! Output: 1
//!
//! ```
//!
//! **Example 2:**
//!
//! ```
//! Input: grid = [
//!   ["1","1","0","0","0"],
//!   ["1","1","0","0","0"],
//!   ["0","0","1","0","0"],
//!   ["0","0","0","1","1"]
//! ]
//! Output: 3
//!
//! ```
//!
//! **Constraints:**
//!
//! * `m == grid.length`
//! * `n == grid[i].length`
//! * `1 <= m, n <= 300`
//! * `grid[i][j]` is `'0'` or `'1'`.
//!

#[solution]
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let mut count = 0;

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] == '1' && !visited[row][col] {
                    count += 1;
                    Self::flood(&grid, &mut visited, row as i32, col as i32);
                }
            }
        }

        count
    }

    fn flood(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, row: i32, col: i32) -> bool {
        let dx = vec![1, 0, -1, 0];
        let dy = vec![0, 1, 0, -1];

        if grid[row as usize][col as usize] == '0' {
            return false;
        }

        if visited[row as usize][col as usize] {
            return false;
        }
        visited[row as usize][col as usize] = true;

        for i in 0..dx.len() {
            let new_row = row + dx[i];
            let new_col = col + dy[i];
            if new_row < 0
                || new_row >= grid.len() as i32
                || new_col < 0
                || new_col >= grid[new_row as usize].len() as i32
            {
                continue;
            }

            Self::flood(grid, visited, new_row, new_col);
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
    fn test_200() {
        assert_eq!(
            3,
            Solution::num_islands(lc_vecvec![
                ['1', '1', '0', '0', '0'],
                ['1', '1', '0', '0', '0'],
                ['0', '0', '1', '0', '0'],
                ['0', '0', '0', '1', '1']
            ])
        );
    }
}
