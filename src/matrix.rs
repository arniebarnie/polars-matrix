use polars::prelude::{PolarsResult, polars_bail, DataType};

pub mod field {
    use super::{DataType};

    pub fn dtype(element: DataType, rows: usize, columns: usize) -> DataType {
        DataType::Array(
            Box::new(DataType::Array(
                Box::new(element), 
                columns
            )), 
            rows
        )
    }
    
    pub fn validate(dtype: &DataType) -> bool {
        match dtype {
            DataType::Array(inner, _) => !matches!(**inner, DataType::Boolean),
            _ => false,
        }
    }
}

pub mod dimensions {
    use super::{PolarsResult, polars_bail, DataType};
    pub fn rows(dtype: &DataType) -> PolarsResult<usize> {
        match dtype {
            DataType::Array(_, rows) => Ok(*rows),
            _ => polars_bail!(InvalidOperation:
                "Expected 2D array dtype, got {:?}", dtype
            ),
        }
    }

    pub fn columns(dtype: &DataType) -> PolarsResult<usize> {
        match dtype {
            DataType::Array(inner, _) => match **inner {
                DataType::Array(_, columns) => Ok(columns),
                _ => polars_bail!(InvalidOperation:
                    "Expected 2D array dtype, got {:?}", dtype
                ),
            },
            _ => polars_bail!(InvalidOperation:
                "Expected 2D array dtype, got {:?}", dtype
            ),
        }
    }

    pub fn shape(dtype: &DataType) -> PolarsResult<(usize, usize)> {
        match dtype {
            DataType::Array(inner, rows) => match inner.as_ref() {
                DataType::Array(_, columns) => Ok((*rows, *columns)),
                _ => polars_bail!(InvalidOperation:
                    "Expected 2D array dtype, got {:?}", dtype
                ),
            },
            _ => polars_bail!(InvalidOperation:
                "Expected 2D array dtype, got {:?}", dtype
            ),
        }
    }
}

pub mod boolean {
    use polars::prelude::{ArrayChunked, BooleanChunked};

    use super::*;

    pub fn all(matrix: &ArrayChunked) -> PolarsResult<bool> {
        let reduced: BooleanChunked = matrix.try_apply_amortized_generic(|row| {
            if let Some(row) = row {
                let row = row.as_ref().bool()?;

                Ok(Some(row.all()))
            } else {
                Ok(None)
            }
        })?;

        Ok(reduced.all())
    }

    pub fn any(matrix: &ArrayChunked) -> PolarsResult<bool> {
        let reduced: BooleanChunked = matrix.try_apply_amortized_generic(|row| {
            if let Some(row) = row {
                let row = row.as_ref().bool()?;

                Ok(Some(row.any()))
            } else {
                Ok(None)
            }
        })?;

        Ok(reduced.any())
    }
}