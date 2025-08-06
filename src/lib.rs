use polars::prelude::*;
use pyo3::prelude::*;
use pyo3_polars::derive::polars_expr;

// fn identity_output_type(inputs: &[Field]) -> PolarsResult<Field> {
//     Ok(Field::new(
//         inputs[0].name.clone(),
//         inputs[0].dtype.clone(),
//     ))
// }

/// Identity function that takes a matrix and returns it unchanged
// #[polars_expr(output_type_func = identity_output_type)]
// fn identity(input: &[Series]) -> PolarsResult<Series> {
//     // Return the input matrix unchanged
//     Ok(input[0].clone())
// }
fn identity_output_type(inputs: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        inputs[0].name.clone(),
        inputs[0].dtype.clone(),
    ))
}

/// Identity function that takes a matrix and returns it unchanged
#[polars_expr(output_type_func = identity_output_type)]
fn identity(input: &[Series]) -> PolarsResult<Series> {
    let series = &input[0];
    println!("series: {:?}", series);
    println!("series.dtype(): {:?}", series.dtype());
    
    // Check that the series has a 2D array dtype
    if let DataType::Array(inner_dtype, _) = series.dtype() {
        println!("inner_dtype: {:?}", inner_dtype);
        if let DataType::Array(_, _) = inner_dtype.as_ref() {
            // This is a 2D array (Array of Arrays)
            // Continue with the function logic
            Ok(series.clone())
        } else {
            // This is a 1D array, not 2D
            polars_bail!(InvalidOperation: "Expected 2D array (Array of Arrays), got 1D array")
        }
    } else {
        // Not an array type at all
        polars_bail!(InvalidOperation: "Expected 2D array dtype, got {:?}", series.dtype())
    }
}

#[pymodule]
fn _lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
