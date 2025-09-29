use std::fmt::Debug;

use ndarray::{Array2, ArrayBase, Data, DataMut, Ix2};
use ndrustfft::{Complex, FftHandler, ndfft, ndifft};
use num_traits::{FloatConst, FromPrimitive, Signed};

// IMPLEMENTATIONS -------------------------------------------------------------

/// Calculate the 2d fft of a given data array
///
/// # Arguments
/// * `spatial` - 2d data in the spatial domain we want to find the fft of
/// * `handler_x` - [`FftHandler`] used for this transformation along the x axis
/// * `handler_x` - [`FftHandler`] used for this transformation along the y axis
pub fn fft2d_with_handlers<A, S>(
    spatial: &ArrayBase<S, Ix2>,
    handler_x: &mut FftHandler<A>,
    handler_y: &mut FftHandler<A>,
) -> Array2<Complex<A>>
where
    A: FloatConst + Copy + Send + Sync + Debug + Signed + FromPrimitive + 'static,
    S: Data<Elem = A>,
{
    let mut fourier0 = spatial.mapv(|x| Complex::<A>::new(x, A::zero()));
    let mut fourier1 = Array2::<Complex<A>>::zeros(fourier0.dim());

    ndfft(&fourier0, &mut fourier1, handler_x, 0);
    ndfft(&fourier1, &mut fourier0, handler_y, 1);

    fourier0
}

/// Calculate the 2d inverse fft of a given complex number 2d array, essentially
/// moving it back into the spatial domain without removing the conjugate
///
/// # Arguments
/// * `fourier` - 2d complex number array in the fourier domain we want to convert back to
///     spatial domain
/// * `handler_x` - [`FftHandler`] used for this transformation along the x axis
/// * `handler_x` - [`FftHandler`] used for this transformation along the y axis
pub fn ifft2d_inplace_with_handlers<A, S>(
    mut fourier: ArrayBase<S, Ix2>,
    handler_x: &mut FftHandler<A>,
    handler_y: &mut FftHandler<A>,
) -> ArrayBase<S, Ix2>
where
    A: FloatConst + Copy + Send + Sync + Debug + Signed + FromPrimitive + 'static,
    S: DataMut<Elem = Complex<A>>,
{
    let mut spatial1 = Array2::<Complex<A>>::zeros(fourier.dim());

    ndifft(&fourier, &mut spatial1, handler_x, 0);
    ndifft(&spatial1, &mut fourier, handler_y, 1);

    fourier
}

// UNIT TESTS ------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fft() {
        let a = Array2::<f32>::from_shape_fn((16, 24), |(x, y)| x as f32 + y as f32 * 4.0);
        let mut handler_x = FftHandler::<f32>::new(a.dim().0);
        let mut handler_y = FftHandler::<f32>::new(a.dim().1);

        let fft = fft2d_with_handlers(&a, &mut handler_x, &mut handler_y);

        assert!(fft.shape() == a.shape());
    }

    #[test]
    fn test_ifft() {
        let a =
            Array2::<f32>::from_shape_fn((16, 24), |(x, y)| (x as f32 + y as f32 * 16.0).round());
        let mut handler_x = FftHandler::<f32>::new(a.dim().0);
        let mut handler_y = FftHandler::<f32>::new(a.dim().1);

        let fft = fft2d_with_handlers(&a, &mut handler_x, &mut handler_y);
        let ifft = ifft2d_inplace_with_handlers(fft, &mut handler_x, &mut handler_y);

        let b = ifft.mapv(|x| x.re.round());
        assert!(a == b);
    }
}
