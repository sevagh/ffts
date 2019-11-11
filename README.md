#  FFTS -- The Fastest Fourier Transform in the South

Forked from <https://github.com/anthonix/ffts>. Original README below.

Build with cmake:

```bash
$  mkdir build && cd build
$  cmake .. && make && sudo make install
```

### Rust bindings

Rust (nightly only for now) bindings for ffts using bindgen.

Components:

* [ffts-sys](./rust/ffts-sys) - bindgen bindings for lffts
* [ffts-rs](./rust/ffts-rs) - idiomatic Rust API for ffts-sys

Structs:

* FFTSPlan - wraps `ffts_sys::ffts_plan_t`
* FFTSComplex - `re: f32, im: f32`, 32-bit aligned as per ffts requirements
* FFTSResult - `Result<T, FFTSError>`
* FFTSError - whenever `ffts_init_*` returns a nullptr

### Basic example

```rust
use ffts::*;

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
```

Compared to C:

```c
#include <ffts/ffts.h>

const float _Complex nums[4] = {-15.0 + 0.0I, 32.0 + 0.0I, -1337.0 + 0.0I, 62.75 + 0.0I};
float _Complex out1[4] = {0.0 + 0.0I, 0.0 + 0.0I, 0.0 + 0.0I, 0.0 + 0.0I};
float _Complex out2[4] = {0.0 + 0.0I, 0.0 + 0.0I, 0.0 + 0.0I, 0.0 + 0.0I};

ffts_plan_t *p = ffts_init_1d(4, FFTS_FORWARD);
ffts_execute(p, nums, out1);

ffts_plan_t *p2 = ffts_init_1d(4, FFTS_BACKWARD);
ffts_execute(p2, out1, out2);

ffts_free(p);
ffts_free(p2);
return 0;
```

# FFTS -- The Fastest Fourier Transform in the South

[![Build Status](https://travis-ci.org/linkotec/ffts.svg?branch=master)](https://travis-ci.org/linkotec/ffts)

To build for Android, edit and run build_android.sh

To build for iOS, edit and run build_iphone.sh 

To build for Linux or OS X on x86, run 
  ./configure --enable-sse --enable-single --prefix=/usr/local
  make
  make install

Optionally build for Windows and Linux with CMake, run
  mkdir build
  cd build
  cmake ..
  
FFTS dynamically generates code at runtime. This can be disabled with 
--disable-dynamic-code

Note that 32 bit x86 dynamic machine code generation is not supported at the moment.

For JNI targets: --enable-jni will build the jni stuff automatically for
the host target, and --enable-shared must also be added manually for it to
work.

If you like FFTS, please show your support by sending a postcard to:

Anthony Blake<br>
Department of Computer Science<br>
The University of Waikato<br>
Private Bag 3105<br>
Hamilton 3240<br>
NEW ZEALAND
