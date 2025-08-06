from pathlib import Path

from polars import Expr
from polars._typing import IntoExpr
from polars.plugins import register_plugin_function

LIB = Path(__file__).parent

def identity(expr: IntoExpr) -> Expr:
    return register_plugin_function(
        args=[expr],
        plugin_path=LIB,
        function_name="identity",
        is_elementwise=True
    )