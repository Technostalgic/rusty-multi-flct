use std::fmt::Debug;

use crate::{
    destretch::offset,
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
        // convert from non-complex data to complex number arrays to work with fft
        let mut fft_reference_0 = reference.mapv(|x| Complex::<A>::new(x, A::zero()));
        let mut fft_subscene_0 = subscene.mapv(|x| Complex::<A>::new(x, A::zero()));
        let mut fft_reference_1 = Array2::<Complex<A>>::zeros(subwindow_dim);
        let mut fft_subscene_1 = Array2::<Complex<A>>::zeros(subwindow_dim);

        // apply fft along x and y axes
        ndfft(
            &fft_reference_0.view(),
            &mut fft_reference_1.view_mut(),
            &mut handler_x,
            0,
        );
        ndfft(
            &fft_reference_1.view(),
            &mut fft_reference_0.view_mut(),
            &mut handler_y,
            1,
        );
        ndfft(
            &fft_subscene_0.view(),
            &mut fft_subscene_1.view_mut(),
            &mut handler_x,
            0,
        );
        ndfft(
            &fft_subscene_1.view(),
            &mut fft_subscene_0.view_mut(),
            &mut handler_y,
            1,
        );

        // get correlation map in fourier space
        let mut correlation_0 = fft_reference_0 * fft_subscene_0;
        let mut correlation_1 = Array2::<Complex<A>>::zeros(subwindow_dim);

        // convert correlation to linear space
        ndifft(
            &correlation_0.view(),
            &mut correlation_1.view_mut(),
            &handler_x,
            0,
        );
        ndifft(
            &correlation_1.view(),
            &mut correlation_0.view_mut(),
            &handler_x,
            0,
        );

        // find and store the correlation peak
        let correlation_peak = correlation_0
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
