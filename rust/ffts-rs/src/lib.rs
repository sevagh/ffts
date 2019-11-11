pub mod complex;

pub use crate::complex::FFTSComplex;
use ffts_sys::*;
use std::{convert::TryInto, error, fmt, result};

type FFTSResult<T> = result::Result<T, FFTSError>;

///For when the underlying ffts C library returns nullptrs
#[derive(Debug)]
pub struct FFTSError;

impl fmt::Display for FFTSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ffts nullptr error")
    }
}

impl error::Error for FFTSError {
    fn description(&self) -> &str {
        "ffts nullptr error"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

///Direction of the FFT, passed to the constructors
pub enum FFTSDirection {
    Forward,
    Backward,
}

///Struct wrapping `ffts_plan_t`
#[derive(Debug)]
pub struct FFTSPlan {
    plan: *mut ffts_plan_t,
}

impl Drop for FFTSPlan {
    fn drop(&mut self) {
        unsafe {
            ffts_free(self.plan);
        }
    }
}

impl FFTSPlan {
    ///Initialize a 1D FFT with the given dimension and direction
    pub fn new_1d(n: usize, direction: FFTSDirection) -> FFTSResult<Self> {
        let plan: *mut ffts_plan_t;

        unsafe {
            plan = match direction {
                FFTSDirection::Forward => ffts_init_1d(n, FFTS_FORWARD as i32),
                FFTSDirection::Backward => ffts_init_1d(n, FFTS_BACKWARD as i32),
            };
        }

        if plan.is_null() {
            return Err(FFTSError);
        }

        Ok(FFTSPlan { plan })
    }

    ///Initialize a 2D FFT with the given dimensions and direction
    pub fn new_2d(n1: usize, n2: usize, direction: FFTSDirection) -> FFTSResult<Self> {
        let plan: *mut ffts_plan_t;

        unsafe {
            plan = match direction {
                FFTSDirection::Forward => ffts_init_2d(n1, n2, FFTS_FORWARD as i32),
                FFTSDirection::Backward => ffts_init_2d(n1, n2, FFTS_BACKWARD as i32),
            };
        }

        if plan.is_null() {
            return Err(FFTSError);
        }

        Ok(FFTSPlan { plan })
    }

    ///Initialize an ND FFT with the given dimension array and direction
    pub fn new_nd(ns: &mut [usize], direction: FFTSDirection) -> FFTSResult<Self> {
        let plan: *mut ffts_plan_t;
        let rank = ns.len().try_into().unwrap();

        unsafe {
            plan = match direction {
                FFTSDirection::Forward => ffts_init_nd(rank, ns.as_mut_ptr(), FFTS_FORWARD as i32),
                FFTSDirection::Backward => {
                    ffts_init_nd(rank, ns.as_mut_ptr(), FFTS_BACKWARD as i32)
                }
            };
        }

        if plan.is_null() {
            return Err(FFTSError);
        }

        Ok(FFTSPlan { plan })
    }

    ///Execute the FFT on the given slice of FFTSComplex and returns the output Vec<FFTSComplex>
    pub fn execute(&mut self, input: &mut [FFTSComplex]) -> Vec<FFTSComplex> {
        let _input = FFTSComplex::to_aligned_c_ptr(input);
        let mut output = vec![FFTSComplex { re: 0.0, im: 0.0 }; input.len()];

        let _output = FFTSComplex::to_aligned_c_ptr(&mut output);

        unsafe {
            ffts_execute(self.plan, _input, _output);
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_1d() {
        let p = FFTSPlan::new_1d(2, FFTSDirection::Forward).unwrap();
        println!("fftsplan 1d struct: {:#?}", p);
    }

    #[test]
    fn test_init_2d() {
        let p = FFTSPlan::new_2d(2, 2, FFTSDirection::Forward).unwrap();
        println!("fftsplan 2d struct: {:#?}", p);
    }

    #[test]
    fn test_init_nd() {
        let p =
            FFTSPlan::new_nd(&mut vec![2usize, 2usize, 2usize], FFTSDirection::Forward).unwrap();
        println!("fftsplan nd struct: {:#?}", p);
    }

    #[test]
    fn test_basic_forward() {
        let random_vals: Vec<f32> = vec![-15.0, 32.0, -1337.0, 62.75];

        let mut p_f = FFTSPlan::new_1d(random_vals.len(), FFTSDirection::Forward).unwrap();

        let mut random_cmplex = FFTSComplex::vec_from_real(&random_vals);

        let forward_vec = p_f.execute(&mut random_cmplex);

        //values taken from a contrived C example

        assert_eq!(forward_vec[0].re, -1257.25);
        assert_eq!(forward_vec[0].im, 0.0);

        assert_eq!(forward_vec[1].re, 1322.0);
        assert_eq!(forward_vec[1].im, 30.75);

        assert_eq!(forward_vec[2].re, -1446.75);
        assert_eq!(forward_vec[2].im, 0.0);

        assert_eq!(forward_vec[3].re, 1322.0);
        assert_eq!(forward_vec[3].im, -30.75);
    }

    #[test]
    fn test_basic_backward() {
        let mut random_cmplex: Vec<FFTSComplex> = vec![
            FFTSComplex { re: -15.0, im: 3.0 },
            FFTSComplex { re: 32.0, im: 2.0 },
            FFTSComplex {
                re: -1337.0,
                im: 1.0,
            },
            FFTSComplex {
                re: 62.75,
                im: -1.0,
            },
        ];

        let mut p_f = FFTSPlan::new_1d(random_cmplex.len(), FFTSDirection::Backward).unwrap();

        let backward_vec = p_f.execute(&mut random_cmplex);

        //values taken from a contrived C example

        assert_eq!(backward_vec[0].re, -1257.25);
        assert_eq!(backward_vec[0].im, 5.0);

        assert_eq!(backward_vec[1].re, 1319.0);
        assert_eq!(backward_vec[1].im, -28.75);

        assert_eq!(backward_vec[2].re, -1446.75);
        assert_eq!(backward_vec[2].im, 3.0);

        assert_eq!(backward_vec[3].re, 1325.0);
        assert_eq!(backward_vec[3].im, 32.75);
    }

    #[test]
    fn test_fft_round_trip_real_2_cmplx() {
        let random_vals: Vec<f32> = vec![-15.0, 32.0, -1337.0, 62.75];

        let mut p_f = FFTSPlan::new_1d(4, FFTSDirection::Forward).unwrap();

        let mut random_cmplex = FFTSComplex::vec_from_real(&random_vals);

        let mut forward_vec = p_f.execute(&mut random_cmplex);

        let mut p_b = FFTSPlan::new_1d(random_vals.len(), FFTSDirection::Backward).unwrap();

        let backward_vec = p_b.execute(&mut forward_vec);

        for i in 0..4 {
            assert_eq!(random_cmplex[i].re, backward_vec[i].re / 4.0);
            assert_eq!(random_cmplex[i].im, backward_vec[i].im / 4.0);
        }
    }

    #[test]
    fn test_fft_round_trip_cmplx_2_cmplx() {
        let mut random_cmplex: Vec<FFTSComplex> = vec![
            FFTSComplex { re: -15.0, im: 3.0 },
            FFTSComplex { re: 32.0, im: 2.0 },
            FFTSComplex {
                re: -1337.0,
                im: 1.0,
            },
            FFTSComplex {
                re: 62.75,
                im: -1.0,
            },
        ];

        let mut p_f = FFTSPlan::new_1d(4, FFTSDirection::Forward).unwrap();

        let mut forward_vec = p_f.execute(&mut random_cmplex);

        let mut p_b = FFTSPlan::new_1d(random_cmplex.len(), FFTSDirection::Backward).unwrap();

        let backward_vec = p_b.execute(&mut forward_vec);

        for i in 0..4 {
            assert_eq!(random_cmplex[i].re, backward_vec[i].re / 4.0);
            assert_eq!(random_cmplex[i].im, backward_vec[i].im / 4.0);
        }
    }
}
