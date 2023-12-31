#![allow(improper_ctypes_definitions)]
#![cfg_attr(
    target_os = "cuda",
    no_std,
    feature(register_attr),
    register_attr(nvvm_internal)
)]

use cuda_std::*;
pub mod kernels;
