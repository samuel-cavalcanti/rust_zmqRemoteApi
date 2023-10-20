from __future__ import annotations
from dataclasses import dataclass

from dataclasses import dataclass
from cpp_token import TokenType


@dataclass
class TypeNode:
    cpp_type: TokenType
    sub_type: list[TypeNode]


@dataclass
class Arg:
    arg_type: TypeNode
    arg_name: str


@dataclass
class FunctionAssign:
    return_type: TypeNode
    function_name: str
    function_args: list[Arg]
    description: str = ''
