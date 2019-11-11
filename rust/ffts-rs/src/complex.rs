use libc::c_void;

///32-bit aligned complex number (see
///[ffts.h](https://github.com/sevagh/ffts/blob/master/include/ffts.h#L68))
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(align(4))] //32-bit alignment for ffts
pub struct FFTSComplex {
    pub re: f32,
    pub im: f32,
}

impl FFTSComplex {
    ///Convert a real [f32] to a [FFTSComplex] with the imaginary values set to 0
    pub fn vec_from_real(real: &[f32]) -> Vec<Self> {
        let mut ret: Vec<FFTSComplex> = Vec::with_capacity(real.len());

        for x in real.iter() {
            ret.push(FFTSComplex { re: *x, im: 0.0 });
        }

        ret
    }

    ///Get a real [f32] from the real values of a [FFTSComplex]
    pub fn vec_to_real(complex: &[Self]) -> Vec<f32> {
        let mut ret: Vec<f32> = Vec::with_capacity(complex.len());

        for x in complex.iter() {
            ret.push(x.re);
        }

        ret
    }

    pub(in crate) fn to_aligned_c_ptr(complex: &mut [Self]) -> *mut c_void {
        complex.as_mut_ptr() as *mut c_void
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_mem_alignment() {
        assert_eq!(mem::align_of::<FFTSComplex>(), 4);
        println!("alignof FFTSComplex: {:#?}", mem::align_of::<FFTSComplex>());
        println!("sizeof FFTSComplex: {:#?}", mem::size_of::<FFTSComplex>());
        println!("sizeof f32: {:#?}", mem::size_of::<f32>());
    }
}
