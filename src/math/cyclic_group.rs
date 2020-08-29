//! define some type whose behavior can be abstracted as a cyclic group
//! while define a meta trait Cyclic as well

use std::convert;
use std::num;
use std::ops::{Add, AddAssign, Sub, SubAssign};

//TODO: Impl foreign trait for type bound by local trait is impossible currently,
// impl<T, C: Cyclic> Add<T> for C could not be done
/// A interface to manipulate an element of cyclic group
pub trait Cyclic: Sized + Copy
{
    type Value: Sized + Copy + PartialEq;
    fn value(&self) -> Self::Value;
    /// incrementing itself and then return its self
    fn increment_by(&mut self, num: Self::Value) -> Self;
    /// decrementing itself and then return its self
    fn decrement_by(&mut self, num: Self::Value) -> Self;
    /// incrementing itself by 1 and then return the result
    fn increment(&mut self) -> Self;
    /// decrementing itself by 1 and then return the result
    fn decrement(&mut self) -> Self;
}

#[derive(Clone, Debug, Default, Copy)]
pub struct CyclicIndex {
    pub index: usize,
    /// keep a clone of size to ensure locality at expense of double the memory use
    pub size: usize,
}

impl CyclicIndex {
    pub fn new(index: usize, size: usize) -> Self {
        CyclicIndex {
            index,
            size,
        }
    }
}

impl convert::From<CyclicIndex> for usize {
    fn from(cyclic_index: CyclicIndex) -> usize {
        cyclic_index.index.clone()
    }
}

impl Cyclic for CyclicIndex {
    type Value = usize;

    fn value(&self) -> usize {
        self.index
    }
    fn increment(&mut self) -> Self {
        self.increment_by(1)
    }
    fn decrement(&mut self) -> Self {
        self.decrement_by(1)
    }
    /// incrementing itself and then return the result
    #[inline]
    fn increment_by(&mut self, num: usize) -> Self {
        // It seems that this will hardly overflow but should be allow to overflow
        self.index = (num::Wrapping(self.index) + num::Wrapping(num)).0 % self.size;
        self.clone()
    }
    #[inline]
    /// decrementing itself and then return the result
    fn decrement_by(&mut self, num: usize) -> Self {
        // This will overflow and should be allow to overflow
        self.index = (num::Wrapping(self.index) - num::Wrapping(num)).0 % self.size;
        self.clone()
    }
}

///TODO: impl<T: Into<usize>> Add<T> for CyclicIndex was denied since
/// if a is a CyclicIndex,
/// a + 1 will failed to compile since compiler will interpret 1 as i32,
/// and i32 is not Into<usize>.
/// one has to explicitly says a + 1usize in order for this to work,
/// which defeats it purposes being T: Into<usize>
impl Add<usize> for CyclicIndex {
    type Output = CyclicIndex;

    fn add(self, rhs: usize) -> Self::Output {
        self.clone().increment_by(rhs.into())
    }
}

impl Sub<usize> for CyclicIndex
{
    type Output = CyclicIndex;

    fn sub(self, rhs: usize) -> Self::Output {
        self.clone().decrement_by(rhs.into())
    }
}

impl AddAssign<usize> for CyclicIndex
{
    fn add_assign(&mut self, rhs: usize) {
        self.increment_by(rhs.into());
    }
}

impl SubAssign<usize> for CyclicIndex
{
    fn sub_assign(&mut self, rhs: usize) {
        self.decrement_by(rhs.into());
    }
}

impl PartialEq for CyclicIndex {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! c8 {
        ($x:expr) => {
            CyclicIndex::new($x,8)
        };
    }
    #[test]
    fn cyclic_index_increment_test() {
        let mut a = c8!(0);
        a.increment();
        assert_eq!(a.value(), 1);
        let mut b = c8!(7);
        b.increment();
        assert_eq!(b.value(), 0);
    }

    #[test]
    fn cyclic_index_decrement_test() {
        let mut a = c8!(0);
        a.decrement();
        assert_eq!(a.value(), 7);
        let mut b = c8!(7);
        b.decrement();
        assert_eq!(b.value(), 6);
    }

    #[test]
    fn cyclic_index_increment_by_test() {
        let mut a = c8!(0);
        a.increment_by(7);
        assert_eq!(a.value(), 7);
        let mut b = c8!(0);
        b.increment_by(8);
        assert_eq!(b.value(), 0);
        let mut c = c8!(0);
        c.increment_by(16);
        assert_eq!(c.value(), 0);
    }

    #[test]
    fn cyclic_index_decrement_by_test() {
        let mut a = c8!(7);
        a.decrement_by(7);
        assert_eq!(a.value(), 0);
        let mut b = c8!(7);
        b.decrement_by(8);
        assert_eq!(b.value(), 7);
        let mut c = c8!(7);
        c.increment_by(16);
        assert_eq!(c.value(), 7);
    }

    #[test]
    fn cyclic_index_test_partial_eq_test() {
        let a = c8!(0);
        let b = c8!(0);
        assert_eq!(a, b);
        assert_eq!(a, a.clone());
    }

    #[test]
    fn cyclic_index_test_operator_overloading_test() {
        let mut a = c8!(0);
        {
            let mut b = a + 1;
            assert_eq!(a, c8!(0));
            assert_eq!(b, c8!(1));
        }
        {
            let mut b = a - 1;
            assert_eq!(a, c8!(0));
            assert_eq!(b, c8!(7));
        }
        {
            let mut b = a;
            b += 1;
            assert_eq!(a, c8!(0));
            assert_eq!(b, c8!(1));
        }
        {
            let mut b = a;
            b -= 1;
            assert_eq!(a, c8!(0));
            assert_eq!(b, c8!(7));
        }
    }
}
