
from typing import Optional, Protocol
from cpp_token import Token
from stream import Stream

from .special_chars import special_chars_scan
from .std_cpp_types_scanner import cpp_types_scan
from .ignore_chars_scan import ignore_chars


class ScannerProtocol(Protocol):

    def current_token(self) -> Optional[Token]:
        ...

    def update(self) -> None:
        ...

class Scanner:
    __token: Optional[Token]
    __stream: Stream
    __scanners = [ignore_chars, special_chars_scan, cpp_types_scan]

    def __init__(self, stream: Stream) -> None:
        self.__stream = stream
        self.update()
        

    def current_token(self) -> Optional[Token]:
        return self.__token

    def update(self) -> None:
        self.__token = self.__run_scan()

    def __run_scan(self):

        for scanner in self.__scanners:
            token = scanner(self.__stream)
            if token:
                return token

        if self.__stream.current_char() is None:
            return None

        raise Exception(
            f'Unable to find token on: stream pos: {self.__stream.pos()}')
