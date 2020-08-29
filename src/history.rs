use crate::math::cyclic_group::{Cyclic, CyclicIndex};

#[derive(Clone, Debug)]
pub struct Cycle<T> {
    values: Vec<T>,
    /// the pointer where the next record will be pushed in
    next: CyclicIndex,
    /// the pointer where the earlier record is
    /// None meaning no valid record was pushed in,
    start: Option<CyclicIndex>,
}

/// default() for value in Cycle for RAII,
/// However, the default() generated values are guaranteed not to be exposed
pub trait CycleDefault {
    fn default() -> Self;
}

impl<T: CycleDefault> Cycle<T>
    where T: Clone {
    pub fn new(capacity: usize) -> Self {
        Cycle {
            values: vec![T::default(); capacity],
            start: None,
            next: CyclicIndex::new(0, capacity),
        }
    }

    pub fn clear(&mut self) {
        self.start = None;
    }

    pub fn push(&mut self, value: T) {
        match self.start {
            Some(start) => {
                if start == self.next + 1 {
                    // full
                    self.start = Some(start + 1)
                }
            }
            None => self.start = Some(self.next)
        }
        self.values[self.next.value()] = value.clone();
        self.next += 1;
        // notes that, self.start is guaranteed not to be equal to self.next
    }
}

/// iterate from start to next - 1
pub struct CycleIntoIterator<'a, T>
    where T: Clone + CycleDefault {
    history: &'a Cycle<T>,
    current: Option<CyclicIndex>,
}

impl<'a, T> IntoIterator for &'a Cycle<T>
    where T: Clone + CycleDefault
{
    type Item = T;
    type IntoIter = CycleIntoIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        CycleIntoIterator {
            history: self,
            current: self.start,
        }
    }
}

impl<'a, T> Iterator for CycleIntoIterator<'a, T>
    where T: Clone + CycleDefault
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self.current {
            None => None,
            Some(current) => {
                if current == self.history.next {
                    None
                } else {
                    let item = self.history.values[current.value()].clone();
                    self.current = Some(current + 1);
                    Some(item)
                }
            }
        }
    }
}

