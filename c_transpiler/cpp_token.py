from dataclasses import dataclass
from enum import Enum, auto
from typing import Optional


class TokenType(Enum):
    U8 = auto()
    I64 = auto()
    F64 = auto()
    VEC = auto()
    TUPLE = auto()
    VOID = auto()
    STRING = auto()
    OPTION = auto()
    BOOL = auto()
    JSON = auto()
    ID = auto()
    # special chars
    LESS = auto()
    BIGGER = auto()
    COMMA = auto()
    OPEN_PARENTHESES = auto()
    CLOSE_PARENTHESES = auto()
    SEMICOLON = auto()


@dataclass
class Token:
    token_type: TokenType
    content: Optional[str]
