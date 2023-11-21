use std::ops::Range;

pub trait Interval {
    fn contains(&self, item: f64) -> bool;
    fn surrounds(&self, item: f64) -> bool;
}

impl Interval for Range<f64> {
    fn contains(&self, item: f64) -> bool {
        self.start <= item && item <= self.end
    }

    fn surrounds(&self, item: f64) -> bool {
        self.start < item && item < self.end
    }
}
