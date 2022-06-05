use crate::bitmap::Bitmap;
use crate::digit::Digit;
use crate::grid::{GridBlock, GridColumn, GridPoint, GridRow};
use crate::range::Range;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::iter::{
    Chain, Cloned, Copied, Cycle, Enumerate, Filter, FilterMap, FlatMap, Flatten, Fuse, Inspect,
    Map, MapWhile, Peekable, Product, Rev, Scan, Skip, SkipWhile, StepBy, Sum, Take, TakeWhile,
    Zip,
};
use std::ops;
use std::{convert, iter, mem};

pub struct Solution {}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut bitmap = Box::new(Bitmap::new());
        for_each_point(board, |point, ch| {
            if let Ok(digit) = Digit::try_from(*ch) {
                bitmap.set_known_digit(point, digit)
            }
        });
        for_each_point_mut(board, |point, ch| {
            *ch = match bitmap.get_digit(point) {
                Ok(d) => d.into(),
                Err(_) => '.',
            }
        });
    }
}

pub fn for_each_point(board: &Vec<Vec<char>>, mut f: impl FnMut(GridPoint, &char) -> ()) {
    for (y, row) in board.iter().enumerate() {
        let y = GridRow(y);
        for (x, ch) in row.iter().enumerate() {
            let point = GridPoint {
                y,
                x: GridColumn(x),
            };
            f(point, ch)
        }
    }
}

pub fn for_each_point_mut(board: &mut Vec<Vec<char>>, f: impl Fn(GridPoint, &mut char) -> ()) {
    for (y, row) in &mut board.iter_mut().enumerate() {
        let y = GridRow(y);
        for (x, ch) in &mut row.iter_mut().enumerate() {
            let point = GridPoint {
                y,
                x: GridColumn(x),
            };
            f(point, ch)
        }
    }
}

pub fn pretty(input: &Vec<Vec<char>>) -> String {
    input
        .iter()
        .map(|row| serde_json::to_string(row).unwrap())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use crate::solution::{pretty, Solution};
    use std::fs;
    use std::fs::File;

    #[test]
    fn test_solve() {
        let input: Vec<Vec<char>> = {
            let in_ = File::open("input.txt").unwrap();
            serde_json::from_reader(in_).unwrap()
        };
        let mut board: Vec<Vec<char>> = input.clone();

        Solution::solve_sudoku(&mut board);

        let out = File::open("output.txt").unwrap();
        let expected: Vec<Vec<char>> = serde_json::from_reader(out).unwrap();

        assert_eq!(board, expected, "\n input:\n{}", pretty(&input))
    }
}
