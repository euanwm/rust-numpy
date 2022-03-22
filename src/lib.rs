//! This crate provides Rust interfaces for [NumPy C APIs][c-api],
//! especially for the [ndarray][ndarray] class.
//!
//! It uses [`pyo3`] for Rust bindings to CPython, and uses
//! [`ndarray`] as the Rust matrix library.
//!
//! To resolve its dependency on NumPy, it calls `import numpy.core` internally.
//! This means that this crate should work if you can use NumPy in your Python environment,
//! e.g. after installing it by `pip install numpy`. It does not matter whether you use
//! the system environment or a dedicated virtual environment.
//!
//! Loading NumPy is done automatically and on demand. So if it is not installed, the functions
//! provided by this crate will panic instead of returning a result.
//!
//! # Example
//!
//! ```
//! use numpy::pyo3::Python;
//! use numpy::ndarray::array;
//! use numpy::{ToPyArray, PyArray};
//!
//! Python::with_gil(|py| {
//!     let py_array = array![[1i64, 2], [3, 4]].to_pyarray(py);
//!
//!     assert_eq!(
//!         py_array.readonly().as_array(),
//!         array![[1i64, 2], [3, 4]]
//!     );
//! })
//! ```
//!
//! [c-api]: https://numpy.org/doc/stable/reference/c-api
//! [ndarray]: https://numpy.org/doc/stable/reference/arrays.ndarray.html

// We often want to make the GIL lifetime explicit.
#![allow(clippy::needless_lifetimes)]

pub mod array;
pub mod convert;
mod dtype;
mod error;
pub mod npyffi;
pub mod npyiter;
mod readonly;
mod slice_container;
mod sum_products;

pub use ndarray;
pub use pyo3;

pub use crate::array::{
    get_array_module, PyArray, PyArray0, PyArray1, PyArray2, PyArray3, PyArray4, PyArray5,
    PyArray6, PyArrayDyn,
};
pub use crate::convert::{IntoPyArray, NpyIndex, ToNpyDims, ToPyArray};
pub use crate::dtype::{dtype, Complex32, Complex64, Element, PyArrayDescr};
pub use crate::error::{DimensionalityError, FromVecError, NotContiguousError, TypeError};
pub use crate::npyffi::{PY_ARRAY_API, PY_UFUNC_API};
#[allow(deprecated)]
pub use crate::npyiter::{
    IterMode, NpyIterFlag, NpyMultiIter, NpyMultiIterBuilder, NpySingleIter, NpySingleIterBuilder,
};
pub use crate::readonly::{
    PyReadonlyArray, PyReadonlyArray1, PyReadonlyArray2, PyReadonlyArray3, PyReadonlyArray4,
    PyReadonlyArray5, PyReadonlyArray6, PyReadonlyArrayDyn,
};
pub use crate::sum_products::{dot, einsum_impl, inner};
pub use ndarray::{array, Ix1, Ix2, Ix3, Ix4, Ix5, Ix6, IxDyn};

#[cfg(doctest)]
mod doctest {
    macro_rules! doc_comment {
        ($doc_string:expr, $mod_name:ident) => {
            #[doc = $doc_string]
            mod $mod_name {}
        };
    }
    doc_comment!(include_str!("../README.md"), readme);
}

#[cold]
#[inline(always)]
fn cold() {}

/// Create a [`PyArray`] with one, two or three dimensions.
///
/// This macro is backed by [`ndarray::array`].
///
/// # Example
///
/// ```
/// use numpy::pyo3::Python;
/// use numpy::ndarray::array;
/// use numpy::pyarray;
///
/// Python::with_gil(|py| {
///     let array = pyarray![py, [1, 2], [3, 4]];
///
///     assert_eq!(
///         array.readonly().as_array(),
///         array![[1, 2], [3, 4]]
///     );
/// });
#[macro_export]
macro_rules! pyarray {
    ($py: ident, $([$([$($x:expr),* $(,)*]),+ $(,)*]),+ $(,)*) => {{
        $crate::IntoPyArray::into_pyarray($crate::array![$([$([$($x,)*],)*],)*], $py)
    }};
    ($py: ident, $([$($x:expr),* $(,)*]),+ $(,)*) => {{
        $crate::IntoPyArray::into_pyarray($crate::array![$([$($x,)*],)*], $py)
    }};
    ($py: ident, $($x:expr),* $(,)*) => {{
        $crate::IntoPyArray::into_pyarray($crate::array![$($x,)*], $py)
    }};
}
