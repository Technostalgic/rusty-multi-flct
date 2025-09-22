use ndarray::{IntoDimension, Ix2};
use std::ops::{Add, Div, Mul, Sub};

// DEFINITIONS -----------------------------------------------------------------

pub trait FitsElement: Clone {}
impl FitsElement for f32 {}
impl FitsElement for f64 {}
impl FitsElement for u8 {}
impl FitsElement for i8 {}
impl FitsElement for u16 {}
impl FitsElement for i16 {}
impl FitsElement for u32 {}
impl FitsElement for i32 {}
impl FitsElement for u64 {}
impl FitsElement for i64 {}

pub trait Numeric: Clone + Copy + Sized + PartialEq + Add + Sub + Mul + Div {}
impl<T: Clone + Copy + PartialEq + Add + Sub + Mul + Div> Numeric for T {}

trait Inty: Clone + Copy + Sized + PartialEq + Eq + Add + Sub + Mul + Div + TryInto<usize> {}
impl<T: Clone + Copy + PartialEq + Eq + Add + Sub + Mul + Div + TryInto<usize>> Inty for T {}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec2<F = f64>
where
    F: Numeric,
{
    x: F,
    y: F,
}

// IMPLEMENTATIONS -------------------------------------------------------------

impl<F> Vec2<F>
where
    F: Numeric,
{
    pub fn new(x: F, y: F) -> Self {
        Vec2 { x: x, y: y }
    }
}

impl<I> From<Ix2> for Vec2<I>
where
    I: Inty + From<usize>,
{
    fn from(value: Ix2) -> Self {
        let dim = value.into_dimension();
        Vec2::<I> {
            x: dim[0].into(),
            y: dim[1].into(),
        }
    }
}

impl<F> From<(F, F)> for Vec2<F>
where
    F: Numeric,
{
    fn from(value: (F, F)) -> Self {
        Vec2::new(value.0, value.1)
    }
}

impl<F> Add for Vec2<F>
where
    F: Numeric + Add<Output = F>,
{
    type Output = Self;
    fn add(self, rhs: Vec2<F>) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<F> Sub for Vec2<F>
where
    F: Numeric + Sub<Output = F>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<F> Mul for Vec2<F>
where
    F: Numeric + Mul<Output = F>,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<F> Mul<F> for Vec2<F>
where
    F: Numeric + Mul<Output = F>,
{
    type Output = Self;
    fn mul(self, rhs: F) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl<F> Div for Vec2<F>
where
    F: Numeric + Div<Output = F>,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl<F> Div<F> for Vec2<F>
where
    F: Numeric + Div<Output = F>,
{
    type Output = Self;
    fn div(self, rhs: F) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

// UNIT TESTS ------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_add() {
        assert_eq!(Vec2::new(4, 15), Vec2::new(3, 8) + Vec2::new(1, 7));
        assert_eq!(
            Vec2::new(4.2, 15.2),
            Vec2::new(3.1, 8.1) + Vec2::new(1.1, 7.1)
        );
    }

    #[test]
    fn vec_sub() {
        assert_eq!(
            Vec2::new(2.0, 1.0),
            Vec2::new(3.1, 8.1) - Vec2::new(1.1, 7.1)
        );
    }

    #[test]
    fn vec_mul() {
        assert_eq!(Vec2::new(10.0, 25.0), Vec2::new(1.0, 2.5) * 10.0);
        assert_eq!(
            Vec2::new(4.0, 7.5),
            Vec2::new(2.0, 2.5) * Vec2::new(2.0, 3.0)
        );
    }

    #[test]
    fn vec_div() {
        assert_eq!(Vec2::new(0.1, 2.5), Vec2::new(1.0, 25.0) / 10.0);
        assert_eq!(
            Vec2::new(0.5, 7.5 / 3.0),
            Vec2::new(2.0, 7.5) / Vec2::new(4.0, 3.0)
        );
    }
}
