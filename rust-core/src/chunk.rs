use ndarray::{s, Array2, ArrayBase, ArrayView2, Data, Ix2};

pub fn slice_kernels<A, S>(
    arr: &ArrayBase<S, Ix2>, 
    size: usize
) -> Array2<ArrayView2<'_, A>> 
where S: Data<Elem = A> {

    let (cols, rows) = arr.dim();
    let kernels_x = cols / size;
    let kernels_y = rows / size;

    Array2::from_shape_fn((kernels_x, kernels_y), 
        |(x, y)| {
            let x0 = x * size;
            let y0 = y * size;
            arr.slice(s![x0..(x0 + size), y0..(y0 + size)])
        })
}

// UNIT TESTS ------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use super::*;
    use ndarray::Array2;

    #[test]
    fn exists() {}

    #[test]
    fn chunk_32() {
        // TODO
        // create an ndarray and chunk it into sub arrays of size with
        // width/height of 32
        let arr = Array2::from_shape_fn((256, 128), |(x, y)| x + y * 256);
        assert_eq!(arr.shape(), &[256, 128]);
        let kernels = slice_kernels(&arr, 32);
        assert_eq!(kernels.shape(), &[256 / 32, 128 / 32]);
    }
}
