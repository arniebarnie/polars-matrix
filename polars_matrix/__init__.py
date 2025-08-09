from pathlib import Path

from polars import Expr
from polars.api import register_expr_namespace
from polars._typing import IntoExpr
from polars.plugins import register_plugin_function

LIB = Path(__file__).parent

@register_expr_namespace('matrix')
class MatrixNamespace:
    def __init__(self, expr: IntoExpr):
        self.expr = expr

    def all(self) -> Expr:
        return register_plugin_function(
            args=[self.expr],
            plugin_path=LIB,
            function_name="all",
            is_elementwise=True
        )

    def any(self) -> Expr:
        return register_plugin_function(
            args=[self.expr],
            plugin_path=LIB,
            function_name="any",
            is_elementwise=True
        )
