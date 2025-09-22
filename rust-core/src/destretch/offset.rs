use crate::vec2::Numeric;
use ndarray::{Array2, ArrayView2};

// DEFINITIONS -----------------------------------------------------------------

pub enum OffsetErr {
    Unknown,
    MismatchedKernels,
    MismatchedKernelSizes,
}

// IMPLEMENTATIONS -------------------------------------------------------------

impl<S> Into<Result<S, OffsetErr>> for OffsetErr {
    fn into(self) -> Result<S, OffsetErr> {
        Err(self)
    }
}

/// Calculate offsets for pre-split kernel arrays
///
/// # Arguments
/// * `kernels_last` - the kernels to calculate destretch offsets from
/// * `kernels_next` - the kernels to calculate destretch offsets to
/// * `apod_factor` - width of the apod mask around the edges, in terms of
/// 	percentage of window size (0.0 to 1.0)
pub fn kernel_offsets<A>(
    kernels_last: Array2<ArrayView2<A>>,
    kernels_next: Array2<ArrayView2<A>>,
    apod_factor: f32,
) -> Result<Array2<A>, OffsetErr>
where
    A: Numeric,
{
    if kernels_last.dim() != kernels_next.dim() {
        return OffsetErr::MismatchedKernels.into();
    }
    if kernels_last.len() > 0 {
        if kernels_last[[0, 0]].dim() != kernels_next[[0, 0]].dim() {
            return OffsetErr::MismatchedKernelSizes.into();
        }
    }

    for (last, next) in kernels_last.into_iter().zip(kernels_next.into_iter()) {
        todo!()
    }

    OffsetErr::Unknown.into()
}

// UNIT TESTS ------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array2;

    #[test]
    fn exists() {}

    fn kernel_offs() {
        let a = Array2::from_shape_fn((10, 10), |(x1, y1)| {
            Array2::from_shape_fn((4, 4), |(x2, y2)| x1 * 16 + y1 * 16 * 10 + x2 + y2 * 4)
        });
        let b = a.clone() + 1;

        kernel_offsets(a, b, 0.08);
    }
}
