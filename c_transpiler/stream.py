from typing import Optional, Protocol


class Stream(Protocol):
    def current_char(self) -> Optional[str]:
        ...

    def back(self, pos: int) -> None:
        ...

    def next(self) -> None:
        ...

    def pos(self) -> int:
        ...


class StringStream:
    current_pos: int
    last_pos: int
    string: str

    def __init__(self, string: str) -> None:
        self.current_pos = 0
        self.string = string
        self.last_pos = len(string)

    def current_char(self) -> Optional[str]:
        if self.current_pos < self.last_pos:
            char = self.string[self.current_pos]
            return char
        return None

    def back(self, pos: int) -> None:
        if pos >= self.current_pos:
            raise Exception(f'Pos {pos}, current: {self.current_pos}')

        self.current_pos = pos

    def pos(self) -> int:
        return self.current_pos

    def next(self) -> None:
        self.current_pos += 1
