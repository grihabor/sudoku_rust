use crate::range::{Range, SameAs};
use std::ops::Add;
use std::{mem, ops};

pub const WIDTH: usize = 9;
pub const HEIGHT: usize = 9;
pub const BLOCK_SIZE: usize = 3;
pub const NUM_BLOCKS: usize = WIDTH / BLOCK_SIZE;

pub const COLUMNS: Range<GridColumn> = Range {
    start: GridColumn(0),
    end: GridColumn(WIDTH),
};
pub const ROWS: Range<GridRow> = Range {
    start: GridRow(0),
    end: GridRow(HEIGHT),
};

pub const BLOCK_COLUMNS: Range<BlockColumn> = Range {
    start: BlockColumn(0),
    end: BlockColumn(NUM_BLOCKS),
};
pub const BLOCK_ROWS: Range<BlockRow> = Range {
    start: BlockRow(0),
    end: BlockRow(NUM_BLOCKS),
};

#[derive(Copy, Clone)]
pub struct GridColumn(pub usize);

impl ops::Add<i32> for GridColumn {
    type Output = GridColumn;

    fn add(self, rhs: i32) -> Self::Output {
        GridColumn((self.0 as i32 + rhs) as usize)
    }
}

impl ops::AddAssign<i32> for GridColumn {
    fn add_assign(&mut self, rhs: i32) {
        *self = self.add(rhs)
    }
}

impl Into<usize> for GridColumn {
    fn into(self) -> usize {
        self.0
    }
}

impl From<usize> for GridColumn {
    fn from(u: usize) -> Self {
        Self(u)
    }
}

impl SameAs<usize> for GridColumn {}

#[derive(Copy, Clone)]
pub struct GridRow(pub usize);

impl ops::Add<usize> for GridRow {
    type Output = GridRow;

    fn add(self, rhs: usize) -> Self::Output {
        GridRow(self.0 + rhs)
    }
}

impl ops::AddAssign<usize> for GridRow {
    fn add_assign(&mut self, rhs: usize) {
        *self = self.add(rhs)
    }
}

impl Into<usize> for GridRow {
    fn into(self) -> usize {
        self.0
    }
}

impl From<usize> for GridRow {
    fn from(u: usize) -> Self {
        Self(u)
    }
}

impl SameAs<usize> for GridRow {}

#[derive(Copy, Clone)]
pub struct GridBlock {
    x: BlockColumn,
    y: BlockRow,
}

impl GridBlock {
    pub fn grid_point(&self, p: BlockPoint) -> GridPoint {
        GridPoint {
            x: GridColumn(self.x.0 * BLOCK_SIZE + p.x.0),
            y: GridRow(self.y.0 * BLOCK_SIZE + p.y.0),
        }
    }
}

#[derive(Copy, Clone)]
pub struct BlockColumn(pub usize);

impl Into<usize> for BlockColumn {
    fn into(self) -> usize {
        self.0
    }
}

impl From<usize> for BlockColumn {
    fn from(u: usize) -> Self {
        Self(u)
    }
}

impl SameAs<usize> for BlockColumn {}

#[derive(Copy, Clone)]
pub struct BlockRow(pub usize);

impl Into<usize> for BlockRow {
    fn into(self) -> usize {
        self.0
    }
}

impl From<usize> for BlockRow {
    fn from(u: usize) -> Self {
        Self(u)
    }
}

impl SameAs<usize> for BlockRow {}

pub struct BlockPoint {
    pub x: BlockColumn,
    pub y: BlockRow,
}

#[derive(Copy, Clone)]
pub struct GridPoint {
    pub x: GridColumn,
    pub y: GridRow,
}

impl GridPoint {
    pub fn column(&self) -> GridColumn {
        self.x
    }
    pub fn row(&self) -> GridRow {
        self.y
    }
    pub fn block(&self) -> GridBlock {
        GridBlock {
            x: BlockColumn(self.x.0 / BLOCK_SIZE),
            y: BlockRow(self.y.0 / BLOCK_SIZE),
        }
    }
}
