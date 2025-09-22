use ndarray::{Array2, ArrayBase, ArrayView2, Data, IntoDimension, Ix2, s};
use std::ops::{Add, Div, Mul, Sub};

// DEFINITIONS -----------------------------------------------------------------

trait Floaty: Clone + Copy + PartialEq + Add + Sub + Mul + Div {}
impl<T: Clone + Copy + PartialEq + Add + Sub + Mul + Div> Floaty for T {}

trait Inty: Clone + Copy + PartialEq + Eq + Add + Sub + Mul + Div + TryInto<usize> {}
impl<T: Clone + Copy + PartialEq + Eq + Add + Sub + Mul + Div + TryInto<usize>> Inty for T {}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Vec2f<F = f32>
where
    F: Floaty,
{
    x: F,
    y: F,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Vec2i<I = i32>
where
    I: Inty,
{
    x: I,
    y: I,
}

// IMPLEMENTATIONS -------------------------------------------------------------

impl<F> Add for Vec2f<F>
where
    F: Floaty + Add<Output = F>,
{
    type Output = Self;

    fn add(self, rhs: Vec2f<F>) -> Self::Output {
        Vec2f::<F> {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<I> From<Ix2> for Vec2i<I>
where
    I: Inty + From<usize>,
{
    fn from(value: Ix2) -> Self {
        let dim = value.into_dimension();
        Vec2i::<I> {
            x: dim[0].into(),
            y: dim[1].into(),
        }
    }
}

/// Slice data into array view kernels of specified size and spacing.
///
/// # Arguments
/// * `data` - input image data.
/// * `kernel_size` - subwindow size in pixels.
/// * `spacing_ratio` - the ratio of spacing between subwindow pixels in terms
///         of a `kernel_size` ratio.
pub fn slice_kernels<A, S>(
    data: &ArrayBase<S, Ix2>,
    kernel_size: usize,
    spacing_ratio: f32,
) -> Array2<ArrayView2<'_, A>>
where
    S: Data<Elem = A>,
{
    let spacing = (kernel_size as f32 * spacing_ratio).round() as usize;
    let spacing = if spacing <= 0 { 1 } else { spacing };

    let size = data.dim();
    let centered_width = size.0 - kernel_size;
    let centered_height = size.1 - kernel_size;

    let kernels_x = centered_width / spacing;
    let kernels_y = centered_height / spacing;
    let off_x = centered_width % spacing / 2;
    let off_y = centered_height % spacing / 2;

    Array2::from_shape_fn((kernels_x, kernels_y), |(x, y)| {
        let x0 = x * spacing + off_x;
        let y0 = y * spacing + off_y;
        data.slice(s![x0..(x0 + kernel_size), y0..(y0 + kernel_size)])
    })
}

// UNIT TESTS ------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use super::*;
    use ndarray::Array2;

    #[test]
    fn chunk_32() {

        // create an array and chunk it into kernels of size 32
        let arr = Array2::from_shape_fn((256, 128), |(x, y)| x + y * 256);
        assert_eq!(arr.shape(), &[256, 128]);
        let kernels = slice_kernels(&arr, 32 as usize, 0.5);

        // assert that the right number of chunks are created
        assert_eq!(
            [(256 - 32) / (32 / 2), (128 - 32) / (32 / 2)],
            *kernels.shape(),
        );
    }
}
