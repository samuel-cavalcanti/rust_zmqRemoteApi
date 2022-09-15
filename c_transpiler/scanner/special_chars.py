from typing import Optional
from cpp_token import Token, TokenType
from stream import Stream


def special_chars_scan(stream: Stream) -> Optional[Token]:

    char = stream.current_char()

    # print(f'special_chars_scan: {char}')
    if char is None:
        return None

    token = get_token(char)
    if token:
        stream.next()
        return token


def get_token(char) -> Optional[Token]:

    tokens = {
        '<': Token(TokenType.LESS, char),
        '>': Token(TokenType.BIGGER, char),
        ';': Token(TokenType.SEMICOLON, char),
        ';': Token(TokenType.SEMICOLON, char),
        '(': Token(TokenType.OPEN_PARENTHESES, char),
        ')': Token(TokenType.CLOSE_PARENTHESES, char),
        ',': Token(TokenType.COMMA, char)
    }

    return tokens.get(char, None)
