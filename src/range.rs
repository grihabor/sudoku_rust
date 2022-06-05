use std::convert;

pub trait SameAs<T>: convert::Into<T> + convert::From<T> {}

#[derive(Copy, Clone)]
pub struct Range<T> {
    pub start: T,
    pub end: T,
}

impl<T> Range<T>
where
    T: SameAs<usize>,
{
    pub fn new(start: T, end: T) -> Range<T> {
        Range { start, end }
    }
}

pub struct RangeIterator<T>
where
    T: SameAs<usize>,
{
    range: Range<T>,
    i: usize,
}

impl<T> Iterator for RangeIterator<T>
where
    T: Copy + SameAs<usize>,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.range.end.into() {
            None
        } else {
            let row = self.i.into();
            self.i += 1;
            Some(row)
        }
    }
}

impl<T> IntoIterator for Range<T>
where
    T: Copy + SameAs<usize>,
{
    type Item = T;
    type IntoIter = RangeIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        RangeIterator {
            range: self,
            i: self.start.into(),
        }
    }
}
