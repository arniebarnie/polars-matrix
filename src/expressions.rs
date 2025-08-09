use polars::prelude::*;
use pyo3_polars::{derive::polars_expr};

use crate::matrix;

#[polars_expr(output_type = Boolean)]
pub fn all(input: &[Series]) -> PolarsResult<Series> {
    let series = &input[0];

    let array: &ArrayChunked = series.array()?;

    let out: BooleanChunked = array.try_apply_amortized_generic(|row| {
        if let Some(row) = row {
            let row = row.as_ref().array()?;

            let out = matrix::boolean::all(row)?;
            Ok(Some(out))
        } else {
            Ok(None)
        }
    })?;

    Ok(out.into_series())
}

#[polars_expr(output_type = Boolean)]
pub fn any(input: &[Series]) -> PolarsResult<Series> {
    let series = &input[0];

    let array: &ArrayChunked = series.array()?;

    let out: BooleanChunked = array.try_apply_amortized_generic(|row| {
        if let Some(row) = row {
            let row = row.as_ref().array()?;

            let out = matrix::boolean::any(row)?;
            Ok(Some(out))
        } else {
            Ok(None)
        }
    })?;

    Ok(out.into_series())
}