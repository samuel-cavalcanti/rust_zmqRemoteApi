from dataclasses import dataclass
from typing import Optional
import unittest

from scanner.scanner import Scanner, ScannerProtocol
from parser import parser, Arg, TypeNode, FunctionAssign
from stream import Stream, StringStream
from cpp_token import TokenType, Token


class mockScanner:
    __tokens: list[Token]
    __current_token: int

    def __init__(self, tokens: list[Token]) -> None:
        self.__current_token = 0
        self.__tokens = tokens

    def current_token(self) -> Optional[Token]:
        if self.__current_token >= len(self.__tokens):
            return None

        return self.__tokens[self.__current_token]

    def update(self) -> None:

        self.__current_token += 1
        return


class mockStream:

    __number_lines: int
    __current_line: int

    def __init__(self, number_lines: int) -> None:
        self.__number_lines = number_lines
        self.__current_line = 0

    def current_char(self) -> Optional[str]:
        if self.__current_line != self.__number_lines:
            self.__current_line += 1
            return 'Mock'
        else:
            return None

        ...

    def back(self, pos: int) -> None:
        pass

    def next(self) -> None:
        pass

    def pos(self) -> int:
        ...


class ParserTestCase(unittest.TestCase):

    def test_switch_thread(self):
        """Parse: void switchThread();"""

        stream = mockStream(number_lines=1)

        tokens = [
            Token(TokenType.VOID, 'void'),
            Token(TokenType.ID, 'switchThread'),
            Token(TokenType.OPEN_PARENTHESES, '('),
            Token(TokenType.CLOSE_PARENTHESES, ')'),
            Token(TokenType.SEMICOLON, ';'),
        ]
        scanner: ScannerProtocol = mockScanner(tokens)
        functions = parser(scanner, stream)

        expected = FunctionAssign(
            return_type=TypeNode(TokenType.VOID, [],),
            function_name='switchThread',
            function_args=[])

        self.assertEqual(functions[0], expected)

    def test_unpack_table(self):
        """Parse: json unpackTable(std::vector<uint8_t> buffer);"""

        tokens = [
            Token(TokenType.JSON, 'json'),
            Token(TokenType.ID, 'unpackTable'),
            Token(TokenType.OPEN_PARENTHESES, '('),
            Token(TokenType.VEC, 'std::vector'),
            Token(TokenType.LESS, '<'),
            Token(TokenType.U8, 'uint8_t'),
            Token(TokenType.BIGGER, '>'),
            Token(TokenType.ID, 'buffer'),
            Token(TokenType.CLOSE_PARENTHESES, ')'),
            Token(TokenType.SEMICOLON, ';'),
        ]
        scanner: ScannerProtocol = mockScanner(tokens)
        stream = mockStream(number_lines=1)
        functions = parser(scanner, stream)

        expected = FunctionAssign(
            return_type=TypeNode(TokenType.JSON, [],),
            function_name='unpackTable',
            function_args=[
                Arg(arg_type=TypeNode(TokenType.VEC, [
                    TypeNode(TokenType.U8, [])]), arg_name='buffer'),
            ])
        self.assertEqual(functions[0], expected)

    def test_wait(self):
        """Parse: double wait(double dt, std::optional<bool> simulationTime = {});"""

        tokens = [
            Token(TokenType.F64, 'double'),
            Token(TokenType.ID, 'wait'),
            Token(TokenType.OPEN_PARENTHESES, '('),

            Token(TokenType.F64, 'double'),
            Token(TokenType.ID, 'dt'),
            Token(TokenType.COMMA, ','),

            Token(TokenType.OPTION, 'std::optional'),
            Token(TokenType.LESS, '<'),
            Token(TokenType.BOOL, 'bool'),
            Token(TokenType.BIGGER, '>'),

            Token(TokenType.ID, 'simulationTime'),
            Token(TokenType.CLOSE_PARENTHESES, ')'),
            Token(TokenType.SEMICOLON, ';'),
        ]
        scanner = mockScanner(tokens)
        stream = mockStream(number_lines=1)
        functions = parser(scanner, stream)
        expected = FunctionAssign(
            return_type=TypeNode(TokenType.F64, [],),
            function_name='wait',
            function_args=[
                Arg(arg_type=TypeNode(TokenType.F64, []), arg_name='dt'),

                Arg(arg_type=TypeNode(TokenType.OPTION, [
                    TypeNode(TokenType.BOOL, [])]), arg_name='simulationTime'),
            ])

        self.assertEqual(functions[0], expected)
