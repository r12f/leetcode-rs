//!
//! Problem #490: The Maze
//!
//! - Link: <https://leetcode.com/problems/the-maze/>
//! - Discussions: <https://leetcode.com/problems/the-maze/discuss/?currentPage=1&orderBy=most_votes&query=>
//!
//! There is a ball in a `maze` with empty spaces (represented as `0`) and walls (represented as `1`). The ball can go through the empty spaces by rolling **up, down, left or right**, but it won't stop rolling until hitting a wall. When the ball stops, it could choose the next direction.
//!
//! Given the `m x n` `maze`, the ball's `start` position and the `destination`, where `start = [start<sub>row</sub>, start<sub>col</sub>]` and `destination = [destination<sub>row</sub>, destination<sub>col</sub>]`, return `true` if the ball can stop at the destination, otherwise return `false`.
//!
//! You may assume that **the borders of the maze are all walls** (see examples).
//!
//! **Example 1:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/03/31/maze1-1-grid.jpg)
//!
//! ```
//! Input: maze = [[0,0,1,0,0],[0,0,0,0,0],[0,0,0,1,0],[1,1,0,1,1],[0,0,0,0,0]], start = [0,4], destination = [4,4]
//! Output: true
//! Explanation: One possible way is : left -> down -> left -> down -> right -> down -> right.
//!
//! ```
//!
//! **Example 2:**
//!
//! ![](https://assets.leetcode.com/uploads/2021/03/31/maze1-2-grid.jpg)
//!
//! ```
//! Input: maze = [[0,0,1,0,0],[0,0,0,0,0],[0,0,0,1,0],[1,1,0,1,1],[0,0,0,0,0]], start = [0,4], destination = [3,2]
//! Output: false
//! Explanation: There is no way for the ball to stop at the destination. Notice that you can pass through the destination but you cannot stop there.
//!
//! ```
//!
//! **Example 3:**
//!
//! ```
//! Input: maze = [[0,0,0,0,0],[1,1,0,0,1],[0,0,0,0,0],[0,1,0,0,1],[0,1,0,0,0]], start = [4,3], destination = [0,1]
//! Output: false
//!
//! ```
//!
//! **Constraints:**
//!
//! * `m == maze.length`
//! * `n == maze[i].length`
//! * `1 <= m, n <= 100`
//! * `maze[i][j]` is `0` or `1`.
//! * `start.length == 2`
//! * `destination.length == 2`
//! * `0 <= start<sub>row</sub>, destination<sub>row</sub> <= m`
//! * `0 <= start<sub>col</sub>, destination<sub>col</sub> <= n`
//! * Both the ball and the destination exist in an empty space, and they will not be in the same position initially.
//! * The maze contains **at least 2 empty spaces**.
//!

use std::collections::VecDeque;

#[solution]
impl Solution {
    pub fn has_path(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> bool {
        let dx = vec![0, 1, 0, -1];
        let dy = vec![1, 0, -1, 0];

        let mut visited = vec![vec![false; maze[0].len()]; maze.len()];

        let mut queue = VecDeque::<(i32, i32)>::new();
        queue.push_back((start[0], start[1]));
        visited[start[0] as usize][start[1] as usize] = true;

        while let Some((row, col)) = queue.pop_front() {
            if row == destination[0] && col == destination[1] {
                return true;
            }

            for i in 0..dx.len() {
                let mut new_row = row;
                let mut new_col = col;

                let mut next_row = row;
                let mut next_col = col;
                while next_row >= 0
                    && next_row < maze.len() as i32
                    && next_col >= 0
                    && next_col < maze[0].len() as i32
                    && maze[next_row as usize][next_col as usize] == 0
                {
                    new_row = next_row;
                    new_col = next_col;

                    next_row += dx[i];
                    next_col += dy[i];
                }

                if visited[new_row as usize][new_col as usize] {
                    continue;
                }
                visited[new_row as usize][new_col as usize] = true;

                queue.push_back((new_row, new_col));
            }
        }

        return false;
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
    fn test_490() {
        assert_eq!(
            true,
            Solution::has_path(
                lc_vecvec![
                    [0, 0, 1, 0, 0],
                    [0, 0, 0, 0, 0],
                    [0, 0, 0, 1, 0],
                    [1, 1, 0, 1, 1],
                    [0, 0, 0, 0, 0]
                ],
                vec![0, 4],
                vec![4, 4]
            )
        );
    }
}
