extern crate sudoku_rust;

use sudoku_rust::solution;


impl Solution {
    fn solve_sudoku(data: &mut Vec<Vec<char>>) {
        solution::Solution::solve_sudoku(data);
    }
}

struct Solution {}

fn main() {
    todo!()
}
