from dataclasses import dataclass
from re import I
import unittest

from scanner.scanner import Scanner, ScannerProtocol
from parser import parser, Arg, TypeNode, FunctionAssign
from stream import Stream, StringStream
from cpp_token import TokenType, Token


@dataclass
class StreamMock:
    tokens: list[Token]


class ParserAndScannerTestCase(unittest.TestCase):

    def test_switch_thread(self):
        """Parser And Scanner: void switchThread();"""
        stream = StringStream('void switchThread();')
        scanner = Scanner(stream)
        functions = parser(scanner, stream)

        expected = FunctionAssign(
            return_type=TypeNode(TokenType.VOID, [],),
            function_name='switchThread',
            function_args=[])

        self.assertEqual(functions[0], expected)

    def test_unpack_table(self):
        """ Parse and Scanner: json unpackTable(std::vector<uint8_t> buffer);"""
        stream = StringStream('json unpackTable(std::vector<uint8_t> buffer);')
        scanner = Scanner(stream)
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
        """Parser and Stream: double wait(double dt, std::optional<bool> simulationTime = {});"""
        stream = StringStream(
            'double wait(double dt, std::optional<bool> simulationTime = {});')
        scanner = Scanner(stream)
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

    def test_multiple_lines(self):
        """
            Parse and Scanner:

            double wait(double dt, std::optional<bool> simulationTime = {});
            json unpackTable(std::vector<uint8_t> buffer);
            void switchThread();
        """

        stream = StringStream(
            """
            double wait(double dt, std::optional<bool> simulationTime = {});
            json unpackTable(std::vector<uint8_t> buffer);
            void switchThread();
            """
        )

        

        scanner = Scanner(stream)

        expected_functions = [
            FunctionAssign(
                return_type=TypeNode(TokenType.F64, [],),
                function_name='wait',
                function_args=[
                    Arg(arg_type=TypeNode(TokenType.F64, []), arg_name='dt'),

                    Arg(arg_type=TypeNode(TokenType.OPTION, [
                        TypeNode(TokenType.BOOL, [])]), arg_name='simulationTime'),
                ]),
            FunctionAssign(
                return_type=TypeNode(TokenType.JSON, [],),
                function_name='unpackTable',
                function_args=[
                    Arg(arg_type=TypeNode(TokenType.VEC, [
                        TypeNode(TokenType.U8, [])]), arg_name='buffer'),
                ]),
            FunctionAssign(
                return_type=TypeNode(TokenType.VOID, [],),
                function_name='switchThread',
                function_args=[])
        ]

        functions = parser(scanner, stream)
        self.assertEqual(len(functions), len(expected_functions))

        for expected_function,function in zip(expected_functions,functions):
             self.assertEqual(expected_function,function)
