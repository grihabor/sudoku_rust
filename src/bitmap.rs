use crate::digit::Digit;
use crate::grid::{BlockPoint, GridBlock, GridColumn, GridPoint, GridRow, HEIGHT, WIDTH};
use crate::variants::{Variants, NUM_BITS};
use crate::{grid, variants};
use std::vec::IntoIter;
use std::{fmt, mem, ops, usize};
use std::convert::{TryInto, TryFrom};
use std::fmt::{Formatter, write};

type Ty = u64; // Bitmap uses Ty to store bits

const GRID_SIZE_BITS: usize = NUM_BITS * grid::WIDTH * grid::HEIGHT;
const MASK: Ty = variants::MASK as Ty;

const GRID_SIZE_TY: usize =
    GRID_SIZE_BITS / (8*mem::size_of::<Ty>()) + ((GRID_SIZE_BITS % (8*mem::size_of::<Ty>()) > 0) as usize);

pub struct Bitmap {
    data: [Ty; GRID_SIZE_TY],
}

impl fmt::Debug for Bitmap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.data, f)
    }
}

impl Bitmap {
    pub fn new() -> Bitmap {
        Bitmap {
            data: [Ty::MAX; GRID_SIZE_TY],
        }
    }

    fn clear_bit(&mut self, idx: Index) {
        self.data[idx.i()] &= !(1 << idx.j());
    }

    fn set_bit(&mut self, idx: Index) {
        self.data[idx.i()] |= 1 << idx.j();
    }

    pub fn set_variants(&mut self, p: GridPoint, variants: Variants) {
        let ops::Range { start, end } = IndexRange::new(p).0;

        let other_bits = self.data[start.i()] & !(MASK << start.j());
        let digit_bits = Ty::from(variants) << start.j();
        self.data[start.i()] = other_bits | digit_bits;

        if end.i() <= start.i() {
            return;
        }
        let other_bits = self.data[end.i()] & !(MASK >> (NUM_BITS - end.j()));
        let digit_bits = Ty::from(variants) >> (NUM_BITS - end.j());
        self.data[end.i()] = other_bits | digit_bits;
    }

    pub fn set_digit(&mut self, p: GridPoint, digit: Digit) {
        self.set_variants(p, digit.into())
    }

    pub fn get_variants(&self, p: GridPoint) -> Variants {
        let ops::Range { start, end } = IndexRange::new(p).0;

        let mut result = (self.data[start.i()] >> start.j()) & MASK;
        if end.i() > start.i() {
            let part2 = (self.data[end.i()] << (NUM_BITS - end.j())) & MASK;
            result |= part2;
        }
        result.try_into().unwrap()
    }

    pub fn get_digit(&self, p: GridPoint) -> Result<Digit, &'static str> {
        self.get_variants(p).try_into()
    }

    fn clear_column(&mut self, x: GridColumn, d: Digit) {
        for y in grid::ROWS {
            self.clear_bit(Index::new(GridPoint { x, y }, d));
        }
    }

    fn clear_row(&mut self, y: GridRow, d: Digit) {
        for x in grid::COLUMNS {
            self.clear_bit(Index::new(GridPoint { x, y }, d));
        }
    }

    fn clear_block(&mut self, b: GridBlock, d: Digit) {
        for y in grid::BLOCK_ROWS {
            for x in grid::BLOCK_COLUMNS {
                let point = b.grid_point(BlockPoint { x, y });
                self.clear_bit(Index::new(point, d));
            }
        }
    }

    pub fn set_known_digit(&mut self, p: GridPoint, digit: Digit) {
        self.clear_column(p.column(), digit);
        self.clear_row(p.row(), digit);
        self.clear_block(p.block(), digit);
        self.set_digit(p, digit);
    }

    pub fn iter(&self) -> BitmapIterator {
        BitmapIterator {
            bitmap: self,
            current: GridRow(0),
        }
    }
}

pub struct Index(usize);

impl fmt::Debug for Index {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(idx={},i={},j={})", self.0, self.i(), self.j())
    }
}

impl Index {
    fn new(p: GridPoint, d: Digit) -> Index {
        Index((p.y.0 * grid::WIDTH + p.x.0) * NUM_BITS + usize::from(d))
    }
    pub fn i(&self) -> usize {
        self.0 / (8*mem::size_of::<Ty>())
    }
    pub fn j(&self) -> usize {
        self.0 % (8*mem::size_of::<Ty>())
    }
}

struct IndexRange(ops::Range<Index>);

impl IndexRange {
    fn new(p: GridPoint) -> IndexRange {
        let start = Index((p.y.0 * grid::WIDTH + p.x.0) * NUM_BITS);
        let end = Index(start.0 + NUM_BITS);
        IndexRange(start..end)
    }
}

pub struct BitmapIterator<'a> {
    bitmap: &'a Bitmap,
    current: GridRow,
}

impl<'a> Iterator for BitmapIterator<'a> {
    type Item = BitmapRow<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.0 >= HEIGHT {
            None
        } else {
            let row = self.current;
            self.current += 1;
            Some(BitmapRow {
                bitmap: self.bitmap,
                row,
            })
        }
    }
}

pub struct BitmapRow<'a> {
    bitmap: &'a Bitmap,
    row: GridRow,
}

impl<'bitmap, 'row> BitmapRow<'bitmap> {
    pub(crate) fn iter(&'row self) -> BitmapRowIterator<'bitmap, 'row> {
        BitmapRowIterator {
            row: self,
            current: GridColumn(0),
        }
    }
}

pub struct BitmapRowIterator<'bitmap, 'row> {
    row: &'row BitmapRow<'bitmap>,
    current: GridColumn,
}

impl Iterator for BitmapRowIterator<'_, '_> {
    type Item = Variants;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.0 >= WIDTH {
            None
        } else {
            let column = self.current;
            self.current += 1;
            Some(self.row.bitmap.get_variants(GridPoint {
                y: self.row.row,
                x: column,
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bitmap::{Bitmap, Index};
    use crate::digit::Digit;
    use crate::grid::{GridColumn, GridPoint, GridRow};
    use crate::solution::{for_each_point, for_each_point_mut, pretty, Solution};
    use crate::{digit, grid, variants};
    use std::env::var;
    use std::fs;
    use std::fs::File;

    #[test]
    fn test_set_get_digit() {
        let input: Vec<Vec<char>> = {
            let in_ = File::open("input.txt").unwrap();
            serde_json::from_reader(in_).unwrap()
        };

        let mut board: Vec<Vec<char>> = input.clone();
        let mut bitmap = Box::new(Bitmap::new());
        for_each_point(&board, |point, ch| {
            if let Ok(digit) = Digit::try_from(*ch) {
                bitmap.set_digit(point, digit)
            }
        });
        for_each_point_mut(&mut board, |point, ch| {
            *ch = match bitmap.get_digit(point) {
                Ok(d) => d.into(),
                Err(_) => '.',
            }
        });

        assert_eq!(board, input, "\n input:\n{}", pretty(&input))
    }

    #[test]
    fn test_row_iterator() {
        let mut bitmap = Bitmap::new();
        for row in bitmap.iter() {
            for vars in row.iter() {
                assert_eq!(vars, variants::ANY);
            }
        }
    }

    #[test]
    fn test_clear_row() {
        let mut bitmap = Bitmap::new();
        let target_row = GridRow(1);
        let target_digit = digit::FIVE;

        bitmap.clear_row(target_row, target_digit);

        for (y, row) in bitmap.iter().enumerate() {
            let expected = if GridRow(y) == target_row {
                variants::ANY ^ target_digit
            } else {
                variants::ANY
            };
            for (x, vars) in row.iter().enumerate() {
                assert_eq!(expected, vars, "y: {}, x: {}", y, x);
            }
        }
    }

    #[test]
    fn test_clear_column() {
        let mut bitmap = Bitmap::new();
        let target_row = GridColumn(3);
        let target_digit = digit::EIGHT;

        bitmap.clear_column(target_row, target_digit);

        for (y, row) in bitmap.iter().enumerate() {
            for (x, vars) in row.iter().enumerate() {
                let expected = if GridColumn(x) == target_row {
                    variants::ANY ^ target_digit
                } else {
                    variants::ANY
                };
                assert_eq!(expected, vars, "y: {}, x: {}", y, x);
            }
        }
    }

}
