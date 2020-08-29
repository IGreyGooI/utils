use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
#[repr(transparent)]
pub struct Probability<R: Copy + Debug + PartialOrd + PartialEq> {
    inner: R
}

// A (+) B = A + B - A * B = R
// R (-) A = B = (R - A) / (1 - A)
// (-) A = 0 (-) A = (0 - A) / (1 - A) = A / (A - 1)
// R (+) (-) A = R + A / (A - 1) - R * A / (A - 1)
//             = (R * A - R + A - R * A) / (A - 1)
//             = (A - R) / (A - 1)
//             = (R - A) / (1 - A)
// A (+) (-) A = (A - A) / (1 - A) = 0

impl<R: Copy + Debug + PartialOrd + PartialEq> Add for Probability<R>
    where R: Add<Output=R> + Mul<Output=R> + Sub<Output=R>,
{
    type Output = Probability<R>;

    fn add(self, rhs: Self) -> Self::Output {
        Probability {
            inner: self.inner + rhs.inner - self.inner * rhs.inner
        }
    }
}

impl<R: Copy + Debug + PartialOrd + PartialEq> Sub for Probability<R>
    where R: Div<Output=R> + Sub<Output=R> + From<f64>,
{
    type Output = Probability<R>;

    fn sub(self, rhs: Self) -> Self::Output {
        Probability {
            inner: (self.inner - rhs.inner) / (R::from(1.0) - rhs.inner.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! p {
        ($x:expr) => {
            Probability {inner: $x}
        };
    }
    #[test]
    fn probability_macro() {
        let a = p!(0.5);
        let b = Probability {
            inner: 0.5
        };

        assert_eq!(a, b)
    }

    #[test]
    fn probability_conversion() {
        let a: Probability<f32> = p!(0.5f32);
        let b: Probability<f64> = p!(a.inner.into());
    }

    #[test]
    fn probability_adding() {
        assert_eq!(p!(0.75), p!(0.5) + p!(0.5))
    }

    #[test]
    fn probability_subtracting() {
        assert_eq!(p!(0.5), p!(0.75) - p!(0.5))
    }

    #[test]
    fn probability_associativity() {
        let a = p!(0.1);
        let b = p!(0.2);
        let c = p!(0.3);
        assert_eq!((a + b) + c, a + (b + c));
    }

    #[test]
    fn probability_commutativity() {
        let a = p!(0.1);
        let b = p!(0.2);
        assert_eq!(a + b, b + a);
    }
}

