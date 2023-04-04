from enum import Enum, auto
from typing import Optional
from cpp_token import Token
from stream import Stream


class State(Enum):
    Comment = auto()
    Ignore_chars = auto()
    pass


def ignore_chars(stream: Stream) -> Optional[Token]:

    state = State.Ignore_chars

    while True:
        char = stream.current_char()
        if char is None:
            return None

        match state:
            case State.Ignore_chars:
                if ignore(char):
                    pass
                else:
                    if char == '/':
                        state = State.Comment
                    else:
                        return None

            case State.Comment:
                if char == '\n':
                    state = State.Ignore_chars
                else:
                    pass

        stream.next()


def ignore(char: str) -> bool:
    ignore_chars = {
        '=',
        '{',
        '}',
    }
    is_ignore = char in ignore_chars

    return char.isspace() or is_ignore
