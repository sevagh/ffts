#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use libc::{c_float, free, posix_memalign};
    use std::{f64, mem::size_of, ptr, slice};

    fn impulse_error(n: usize, sign: i32, data: &[f32]) -> f64 {
        let mut delta_sum: f64 = 0.0;
        let mut sum: f64 = 0.0;

        let mut re: f64;
        let mut im: f64;
        let mut inner: f64;

        for i in 0..n {
            inner = 2.0 * f64::consts::PI * (i as f64) / (n as f64);
            re = inner.cos();
            im = if sign < 0 { -inner.sin() } else { inner.sin() };

            sum += re * re + im * im;

            re = re - (data[2 * i] as f64);
            im = im - (data[2 * i + 1] as f64);

            delta_sum += re * re + im * im;
        }

        delta_sum.sqrt() / sum.sqrt()
    }

    fn test_transform(n: usize, sign: i32) {
        unsafe {
            let mut _input = ptr::null_mut();
            let mut _output = ptr::null_mut();

            match posix_memalign(&mut _input, 32, 2 * n * size_of::<f32>()) {
                0 => (),
                _ => panic!("posix_memalign failed for input array"),
            }

            match posix_memalign(&mut _output, 32, 2 * n * size_of::<f32>()) {
                0 => (),
                _ => panic!("posix_memalign failed for output array"),
            }

            let input: &mut [c_float] = slice::from_raw_parts_mut(_input as *mut c_float, 2 * n);
            let output: &mut [c_float] = slice::from_raw_parts_mut(_output as *mut c_float, 2 * n);

            for i in 0..n {
                input[2 * i + 0] = 0.0;
                input[2 * i + 1] = 0.0;
            }

            input[2] = 1.0;

            let p = ffts_init_1d(n, sign);

            if p.is_null() {
                panic!("ffts plan unsupported")
            }

            ffts_execute(p, _input, _output);

            println!(
                " {:3}  | {:9} | {:.6E}",
                sign,
                n,
                impulse_error(n, sign, output)
            );

            ffts_free(p);

            free(_input);
            free(_output);
        }
    }

    #[test]
    fn test_ffts_errors() {
        let mut n: usize = 1;
        let mut power2: usize = 2;

        while n <= 18 {
            test_transform(power2, -1);
            n += 1;
            power2 <<= 1;
        }

        n = 1;
        power2 = 2;
        while n <= 18 {
            test_transform(power2, 1);
            n += 1;
            power2 <<= 1;
        }
    }
}
