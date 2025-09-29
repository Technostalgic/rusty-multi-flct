use std::fmt::Debug;

use crate::{
    destretch::{
        fft::{fft2d_with_handlers, ifft2d_inplace_with_handlers},
        offset,
    },
    vec2::Vec2,
};
use ndarray::{Array2, ArrayBase, Data, Dim, Ix2};
use ndrustfft::{Complex, FftHandler, FftNum, ndfft, ndifft};
use num_traits::{FloatConst, FromPrimitive, Signed};

// DEFINITIONS -----------------------------------------------------------------

#[derive(Debug)]
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
/// * `reference_kernels` - the kernels to calculate destretch offsets from
/// * `scene_kernels` - the kernels to calculate destretch offsets to
/// * `apod_factor` - width of the apod mask around the edges, in terms of
/// 	percentage of window size (0.0 to 1.0)
pub fn kernel_offsets<A, S>(
    reference_kernels: &Array2<ArrayBase<S, Ix2>>,
    scene_kernels: &Array2<ArrayBase<S, Ix2>>,
    apod_factor: f32,
) -> Result<Array2<Vec2<f32>>, OffsetErr>
where
    A: FloatConst + Copy + Send + Sync + Debug + PartialOrd + Signed + FromPrimitive + 'static,
    S: Data<Elem = A>,
{
    // ensure dimentsions of kernels and subwindows match
    if reference_kernels.dim() != scene_kernels.dim() {
        return OffsetErr::MismatchedKernels.into();
    }
    if reference_kernels.len() > 0 {
        if reference_kernels[[0, 0]].dim() != scene_kernels[[0, 0]].dim() {
            return OffsetErr::MismatchedKernelSizes.into();
        }
    } else {
        return Ok(Array2::<Vec2<f32>>::from_elem((0, 0), (0.0, 0.0).into()));
    }

    // create fft handlers outside loop so they don't get recreated every iteration
    let subwindow_dim = reference_kernels[[0, 0]].dim();
    let subwindow_size: Vec2<usize> = subwindow_dim.into();
    let mut handler_x = FftHandler::<A>::new(subwindow_size.x);
    let mut handler_y = FftHandler::<A>::new(subwindow_size.y);

    // create an empty array to hold the calculated offsets
    let mut offsets = Array2::<Vec2<f32>>::from_elem(reference_kernels.dim(), (0.0, 0.0).into());

    // iterate through each subwindow in the reference and scene kernels
    for ((idx, reference), (_, subscene)) in reference_kernels
        .indexed_iter()
        .zip(scene_kernels.indexed_iter())
    {
        // apply fft
        let fft_reference = fft2d_with_handlers(reference, &mut handler_x, &mut handler_y);
        let fft_subscene = fft2d_with_handlers(subscene, &mut handler_x, &mut handler_y);

        // get correlation map
        let correlation = ifft2d_inplace_with_handlers(
            fft_reference * fft_subscene,
            &mut handler_x,
            &mut handler_y,
        );

        // find and store the correlation peak
        let correlation_peak = correlation
            .indexed_iter()
            .max_by(|(_, a), (_, b)| a.re.partial_cmp(&b.re).unwrap())
            .unwrap()
            .0;
        let offset: Vec2<f32> = (correlation_peak.0 as f32, correlation_peak.1 as f32).into();
        // TODO interpolate between pixels
        offsets[idx] = offset;
    }

    Ok(offsets)
}

// UNIT TESTS ------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array2;

    #[test]
    fn kernel_offs() {
        let a = Array2::from_shape_fn((10, 10), |(x1, y1)| {
            Array2::<f32>::from_shape_fn((4, 4), |(x2, y2)| {
                x1 as f32 * 16.0 + y1 as f32 * 16.0 * 10.0 + x2 as f32 + y2 as f32 * 4.0
            })
        });
        let b = a.clone() + 1.0;

        let offs = kernel_offsets(&a, &b, 0.08);
        assert_eq!(a.shape(), offs.unwrap().shape())
    }
}
