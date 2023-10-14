from typing import Optional
import unittest
from ir_transpiler import Arg, TypeNode, FunctionAssign
from scanner.scanner import ScannerProtocol
from parser import parser
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

    def test_callback_(self):
        """parse: int64_t testCB(int64_t a, std::string cb, int64_t b);"""

        stream = mockStream(number_lines=1)
        tokens = [
            Token(TokenType.I64, 'int64_t'),
            Token(TokenType.ID, 'testCB'),

            Token(TokenType.OPEN_PARENTHESES, '('),

            Token(TokenType.I64, 'int64_t'),
            Token(TokenType.ID, 'a'),
            Token(TokenType.COMMA, ','),

            Token(TokenType.STRING, 'std::string'),
            Token(TokenType.ID, 'cb'),
            Token(TokenType.COMMA, ','),

            Token(TokenType.I64, 'int64_t'),
            Token(TokenType.ID, 'b'),

            Token(TokenType.CLOSE_PARENTHESES, ')'),

            Token(TokenType.SEMICOLON, ';'),
        ]
        scanner: ScannerProtocol = mockScanner(tokens)
        functions = parser(scanner, stream)

        expected = FunctionAssign(
            return_type=TypeNode(TokenType.I64, [],),
            function_name='testCB',
            function_args=[
                Arg(arg_type=TypeNode(TokenType.I64, []), arg_name='a'),
                Arg(arg_type=TypeNode(TokenType.STRING, []), arg_name='cb'),
                Arg(arg_type=TypeNode(TokenType.I64, []), arg_name='b')
            ]
        )

        self.assertEqual(functions[0], expected)

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

    def test_get_vision_sensor_depth_buffer(self):
        """ Parser: std::tuple<std::vector<uint8_t>, std::vector<int64_t>> getVisionSensorDepthBuffer(int64_t sensorHandle, std::optional<std::vector<int64_t>> pos = {},std::optional<std::vector<int64_t>> size = {});
        """
        vec_u8 = [Token(TokenType.VEC, 'std::vector'),
                  Token(TokenType.LESS, '<'),
                  Token(TokenType.U8, 'uint8_t'),
                  Token(TokenType.BIGGER, '>'),
                  ]

        vec_i64 = [Token(TokenType.VEC, 'std::vector'),
                   Token(TokenType.LESS, '<'),
                   Token(TokenType.I64, 'int64_t'),
                   Token(TokenType.BIGGER, '>'),
                   ]

        sensor_handle = [
            Token(TokenType.I64, 'int64_t'),
            Token(TokenType.ID, 'sensorHandle'),
        ]

        option_vec_i64 = [
            Token(TokenType.OPTION, 'std::optional'),
            Token(TokenType.LESS, '<'),
            *vec_i64,
            Token(TokenType.BIGGER, '>'),

        ]

        tokens = [
            Token(TokenType.TUPLE, 'std::tuple'),
            Token(TokenType.LESS, '<'),
            *vec_u8,
            Token(TokenType.COMMA, ','),
            *vec_i64,
            Token(TokenType.BIGGER, '>'),
            Token(TokenType.ID, 'getVisionSensorDepthBuffer'),

            Token(TokenType.OPEN_PARENTHESES, '('),
            *sensor_handle,
            Token(TokenType.COMMA, ','),
            *option_vec_i64,
            Token(TokenType.ID, 'pos'),
            Token(TokenType.COMMA, ','),
            *option_vec_i64,
            Token(TokenType.ID, 'size'),
            Token(TokenType.CLOSE_PARENTHESES, ')'),
            Token(TokenType.SEMICOLON, ';'),
        ]
        scanner = mockScanner(tokens)
        stream = mockStream(number_lines=1)
        functions = parser(scanner, stream)

        vec_u8_ir = TypeNode(TokenType.VEC, [TypeNode(TokenType.U8, [])])
        vec_i64_ir = TypeNode(TokenType.VEC, [TypeNode(TokenType.I64, [])])

        expected = FunctionAssign(
            return_type=TypeNode(TokenType.TUPLE, [vec_u8_ir, vec_i64_ir],),
            function_name='getVisionSensorDepthBuffer',
            function_args=[
                Arg(arg_type=TypeNode(TokenType.I64, []), arg_name='sensorHandle'),

                Arg(arg_type=TypeNode(TokenType.OPTION,
                    [vec_i64_ir]), arg_name='pos'),
                Arg(arg_type=TypeNode(TokenType.OPTION,
                    [vec_i64_ir]), arg_name='size'),
            ])

        self.assertEqual(functions[0], expected)
