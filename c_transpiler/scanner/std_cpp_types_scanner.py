from enum import Enum, auto
from typing import Optional
from cpp_token import Token, TokenType
from stream import Stream


class StateLettersWithDigits(Enum):
    initial_state = auto()
    creating_lexema = auto()


def cpp_types_scan(stream: Stream) -> Optional[Token]:

    lexema = ''
    state = StateLettersWithDigits.initial_state

    while True:
        char = stream.current_char()
        if char is None:
            return None

        match state:

            case StateLettersWithDigits.initial_state:
                if char.isalpha():
                    lexema += char
                    state = StateLettersWithDigits.creating_lexema
                else:
                    return None

            case  StateLettersWithDigits.creating_lexema:

                if char.isalnum() or char == '_' or char == ':':
                    lexema += char
                else:
                    # print(f'current char: {char.encode()}')
                    return create_token(lexema)

        stream.next()


def create_token(content: str) -> Token:

    cpp_tokens_eq = {
        'uint8_t': Token(TokenType.U8, 'uint8_t'),
        'int64_t': Token(TokenType.I64, 'int64_t'),
        'double': Token(TokenType.F64, 'double'),
        'bool': Token(TokenType.BOOL, 'bool'),
        'void': Token(TokenType.VOID, 'void'),
        'std::tuple': Token(TokenType.TUPLE, 'std::tuple'),
        'std::vector': Token(TokenType.VEC, 'std::vector'),
        'std::optional': Token(TokenType.OPTION, 'std::optional'),
        'std::string': Token(TokenType.STRING, 'std::string'),
        'json': Token(TokenType.JSON, 'json')
    }

    token = cpp_tokens_eq.get(content, None)

    if token is None:
        return Token(TokenType.ID, content)
    else:
        return token
