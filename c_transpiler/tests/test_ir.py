from pathlib import Path
import unittest


from ir_transpiler import FunctionAssign, Arg, TypeNode
from ir_transpiler import ir_to_cpp, ir_to_string, ir_to_macro_request_rust
from cpp_token import TokenType
from scanner import Scanner
from stream import StringStream
from parser import parser


class IrTestCase(unittest.TestCase):

    def test_ir_to_cpp(self):

        vec_u8_ir = TypeNode(TokenType.VEC, [TypeNode(TokenType.U8, [])])
        vec_i64_ir = TypeNode(TokenType.VEC, [TypeNode(TokenType.I64, [])])
        option_bool_ir = TypeNode(
            TokenType.OPTION, [TypeNode(TokenType.BOOL, [])])

        wait_ir = FunctionAssign(
            return_type=TypeNode(TokenType.F64, [],),
            function_name='wait',
            function_args=[
                Arg(arg_type=TypeNode(TokenType.F64, []), arg_name='dt'),

                Arg(arg_type=option_bool_ir, arg_name='simulationTime'),
            ])

        unpack_table_ir = FunctionAssign(
            return_type=TypeNode(TokenType.JSON, [],),
            function_name='unpackTable',
            function_args=[
                Arg(arg_type=vec_u8_ir, arg_name='buffer'),
            ])

        switch_thread_ir = FunctionAssign(
            return_type=TypeNode(TokenType.VOID, [],),
            function_name='switchThread',
            function_args=[])

        get_vision_sensor_depth_buffer_if = FunctionAssign(
            return_type=TypeNode(TokenType.TUPLE, [vec_u8_ir, vec_i64_ir],),
            function_name='getVisionSensorDepthBuffer',
            function_args=[
                Arg(arg_type=TypeNode(TokenType.I64, []), arg_name='sensorHandle'),

                Arg(arg_type=TypeNode(TokenType.OPTION,
                    [vec_i64_ir]), arg_name='pos'),
                Arg(arg_type=TypeNode(TokenType.OPTION,
                    [vec_i64_ir]), arg_name='size'),
            ])
        inputs = [switch_thread_ir,
                  unpack_table_ir,
                  wait_ir,
                  get_vision_sensor_depth_buffer_if]

        expected_strings = [
            "void switchThread();",
            "json unpackTable(std::vector<uint8_t> buffer);",
            "double wait(double dt, std::optional<bool> simulationTime = {});",
            "std::tuple<std::vector<uint8_t>, std::vector<int64_t>> getVisionSensorDepthBuffer(int64_t sensorHandle, std::optional<std::vector<int64_t>> pos = {}, std::optional<std::vector<int64_t>> size = {});",
        ]

        result = [ir_to_cpp(ir) for ir in inputs]
        self.assertEqualStrings(result, expected_strings)

    def test_ir_parser_remote_api_header(self):
        assets = Path('assets')
        header = assets / Path('remote_api_header.h')
        expected_h = assets / Path('expected.h')

        content = header.read_text()
        stream = StringStream(content)
        scanner = Scanner(stream)

        assigns = parser(scanner, stream)
        ir = [ir_to_cpp(assign) for assign in assigns]
        result_content = "\n".join(ir)

        self.assertEqual(result_content, expected_h.read_text())

    def test_ir_to_string(self):
        vec_u8_ir = TypeNode(TokenType.VEC, [TypeNode(TokenType.U8, [])])
        vec_i64_ir = TypeNode(TokenType.VEC, [TypeNode(TokenType.I64, [])])
        option_bool_ir = TypeNode(
            TokenType.OPTION, [TypeNode(TokenType.BOOL, [])])

        wait_ir = FunctionAssign(
            return_type=TypeNode(TokenType.F64, [],),
            function_name='wait',
            function_args=[
                Arg(arg_type=TypeNode(TokenType.F64, []), arg_name='dt'),

                Arg(arg_type=option_bool_ir, arg_name='simulationTime'),
            ])

        unpack_table_ir = FunctionAssign(
            return_type=TypeNode(TokenType.JSON, [],),
            function_name='unpackTable',
            function_args=[
                Arg(arg_type=vec_u8_ir, arg_name='buffer'),
            ])

        switch_thread_ir = FunctionAssign(
            return_type=TypeNode(TokenType.VOID, [],),
            function_name='switchThread',
            function_args=[])

        get_vision_sensor_depth_buffer_if = FunctionAssign(
            return_type=TypeNode(TokenType.TUPLE, [vec_u8_ir, vec_i64_ir],),
            function_name='getVisionSensorDepthBuffer',
            function_args=[
                Arg(arg_type=TypeNode(TokenType.I64, []), arg_name='sensorHandle'),

                Arg(arg_type=TypeNode(TokenType.OPTION,
                    [vec_i64_ir]), arg_name='pos'),
                Arg(arg_type=TypeNode(TokenType.OPTION,
                    [vec_i64_ir]), arg_name='size'),
            ])
        inputs = [switch_thread_ir, unpack_table_ir,
                  wait_ir, get_vision_sensor_depth_buffer_if]

        expected_strings = [
            "VOID switchThread()",
            "JSON unpackTable(VEC[U8] buffer)",
            "F64 wait(F64 dt, OPTION[BOOL] simulationTime)",
            "TUPLE[VEC[U8], VEC[I64]] getVisionSensorDepthBuffer(I64 sensorHandle, OPTION[VEC[I64]] pos, OPTION[VEC[I64]] size)",
        ]

        result = [ir_to_string(ir) for ir in inputs]

        self.assertEqualStrings(result, expected_strings)

    def test_ir_to_macro_request_rust(self):
        vec_u8_ir = TypeNode(TokenType.VEC, [TypeNode(TokenType.U8, [])])
        vec_i64_ir = TypeNode(TokenType.VEC, [TypeNode(TokenType.I64, [])])
        option_bool_ir = TypeNode(
            TokenType.OPTION, [TypeNode(TokenType.BOOL, [])])

        wait_ir = FunctionAssign(
            return_type=TypeNode(TokenType.F64, [],),
            function_name='wait',
            function_args=[
                Arg(arg_type=TypeNode(TokenType.F64, []), arg_name='dt'),

                Arg(arg_type=option_bool_ir, arg_name='simulationTime'),
            ])

        unpack_table_ir = FunctionAssign(
            return_type=TypeNode(TokenType.JSON, [],),
            function_name='unpackTable',
            function_args=[
                Arg(arg_type=vec_u8_ir, arg_name='buffer'),
            ])

        switch_thread_ir = FunctionAssign(
            return_type=TypeNode(TokenType.VOID, [],),
            function_name='switchThread',
            function_args=[])

        get_vision_sensor_depth_buffer_if = FunctionAssign(
            return_type=TypeNode(TokenType.TUPLE, [vec_u8_ir, vec_i64_ir],),
            function_name='getVisionSensorDepthBuffer',
            function_args=[
                Arg(arg_type=TypeNode(TokenType.I64, []), arg_name='sensorHandle'),

                Arg(arg_type=TypeNode(TokenType.OPTION,
                    [vec_i64_ir]), arg_name='pos'),
                Arg(arg_type=TypeNode(TokenType.OPTION,
                    [vec_i64_ir]), arg_name='size'),
            ])
        inputs = [switch_thread_ir, unpack_table_ir,
                  wait_ir, get_vision_sensor_depth_buffer_if]

        expected_strings = [
            '(switch_thread,"switchThread")',
            '(unpack_table,"unpackTable",(buffer:Vec<u8>)->serde_json::Value)',
            '(wait,"wait",(dt:f64),opt(simulation_time:bool)->f64)',
            '(get_vision_sensor_depth_buffer,"getVisionSensorDepthBuffer",(sensor_handle:i64),opt(pos:Vec<i64>,size:Vec<i64>)->(Vec<u8>,Vec<i64>))'
        ]
        result = [ir_to_macro_request_rust(ir) for ir in inputs]
        self.assertEqualStrings(result, expected_strings)

    def assertEqualStrings(self, result: list[str], expected: list[str]):
        self.assertEqual(len(result), len(expected))
        for r, e in zip(result, expected):
            self.assertEqual(r, e)
