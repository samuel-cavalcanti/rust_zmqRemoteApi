import unittest

from scanner.scanner import Scanner

from stream import Stream, StringStream
from cpp_token import TokenType, Token


class ScannerTestCase(unittest.TestCase):

    def test_comments(self):
        """Scanner: / DEPRECATED START\n"""
        comment = '// DEPRECATED START\n'
        stream = StringStream(comment)
        scanner = Scanner(stream)

        scanner.update()
        token = scanner.current_token()

        self.assertEqual(token, None)
        self.assertEqual(stream.pos(), stream.last_pos)

    def test_switch_thread(self):
        """Scanner: void switchThread();"""
        stream = StringStream('void switchThread();')
        scanner = Scanner(stream)
        expected_tokens = [
            Token(TokenType.VOID, 'void'),
            Token(TokenType.ID, 'switchThread'),
            Token(TokenType.OPEN_PARENTHESES, '('),
            Token(TokenType.CLOSE_PARENTHESES, ')'),
            Token(TokenType.SEMICOLON, ';'),
        ]
        self.assert_scanner(scanner, stream, expected_tokens)

    def test_unload_module(self):
        """Scanner: int64_t unloadModule(int64_t pluginHandle);"""
        stream = StringStream('int64_t unloadModule(int64_t pluginHandle);')
        scanner = Scanner(stream)
        expected_tokens = [
            Token(TokenType.I64, 'int64_t'),
            Token(TokenType.ID, 'unloadModule'),
            Token(TokenType.OPEN_PARENTHESES, '('),
            Token(TokenType.I64, 'int64_t'),
            Token(TokenType.ID, 'pluginHandle'),
            Token(TokenType.CLOSE_PARENTHESES, ')'),
            Token(TokenType.SEMICOLON, ';'),
        ]
        self.assert_scanner(scanner, stream, expected_tokens)

    def test_unpack_table(self):
        """Scanner: json unpackTable(std::vector<uint8_t> buffer);"""
        stream = StringStream('json unpackTable(std::vector<uint8_t> buffer);')
        scanner = Scanner(stream)
        expected_tokens = [
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
        self.assert_scanner(scanner, stream, expected_tokens)

    def test_simple_type(self):
        """Scanner: int64_t addDrawingObject double void std::tuple std::vector json std::optional std::string"""
        stream = StringStream(
            ' int64_t addDrawingObject double void std::tuple std::vector json std::optional std::string\n')
        scanner = Scanner(stream)

        expected_tokens = [
            Token(TokenType.I64, 'int64_t'),
            Token(TokenType.ID, 'addDrawingObject'),
            Token(TokenType.F64, 'double'),
            Token(TokenType.VOID, 'void'),
            Token(TokenType.TUPLE, 'std::tuple'),
            Token(TokenType.VEC, 'std::vector'),
            Token(TokenType.JSON, 'json'),
            Token(TokenType.OPTION, 'std::optional'),
            Token(TokenType.STRING, 'std::string'),
        ]

        self.assert_scanner(scanner, stream, expected_tokens)

    def test_comma(self):
        """Scanner: int64_t,addDrawingObject,double,void,std::tuple,std::vector,json;"""
        stream = StringStream(
            'int64_t,addDrawingObject,double,void,std::tuple,std::vector,json;\n')

        scanner = Scanner(stream)
        expected_tokens = [
            Token(TokenType.I64, 'int64_t'),
            Token(TokenType.COMMA, ','),
            Token(TokenType.ID, 'addDrawingObject'),
            Token(TokenType.COMMA, ','),
            Token(TokenType.F64, 'double'),
            Token(TokenType.COMMA, ','),
            Token(TokenType.VOID, 'void'),
            Token(TokenType.COMMA, ','),
            Token(TokenType.TUPLE, 'std::tuple'),
            Token(TokenType.COMMA, ','),
            Token(TokenType.VEC, 'std::vector'),
            Token(TokenType.COMMA, ','),
            Token(TokenType.JSON, 'json'),
            Token(TokenType.SEMICOLON, ';'),
        ]

        self.assert_scanner(scanner, stream, expected_tokens)

    def test_complex_type(self):
        """Scanner: std::tuple<int64_t, double, std::vector<double>, int64_t, std::vector<double>>\n"""
        stream = StringStream(
            'std::tuple<int64_t, double, std::vector<double>, int64_t, std::vector<double>>\n')
        scanner = Scanner(stream)

        expected_tokens = [
            Token(TokenType.TUPLE, 'std::tuple'),
            Token(TokenType.LESS, '<'),
            Token(TokenType.I64, 'int64_t'),
            Token(TokenType.COMMA, ','),
            Token(TokenType.F64, 'double'),
            Token(TokenType.COMMA, ','),
            Token(TokenType.VEC, 'std::vector'),
            Token(TokenType.LESS, '<'),
            Token(TokenType.F64, 'double'),
            Token(TokenType.BIGGER, '>'),
            Token(TokenType.COMMA, ','),
            Token(TokenType.I64, 'int64_t'),
            Token(TokenType.COMMA, ','),
            Token(TokenType.VEC, 'std::vector'),
            Token(TokenType.LESS, '<'),
            Token(TokenType.F64, 'double'),
            Token(TokenType.BIGGER, '>'),
            Token(TokenType.BIGGER, '>'),
        ]
        self.assert_scanner(scanner, stream, expected_tokens)

    def test_adjust_view(self):
        """Scanner: int64_t adjustView(int64_t viewHandleOrIndex, int64_t associatedViewableObjectHandle, int64_t options, std::optional<std::string> viewLabel = {});"""
        stream = StringStream(
            'int64_t adjustView(int64_t viewHandleOrIndex, int64_t associatedViewableObjectHandle, int64_t options, std::optional<std::string> viewLabel = {});')
        scanner = Scanner(stream)
        expected_tokens = [
            Token(TokenType.I64, 'int64_t'),
            Token(TokenType.ID, 'adjustView'),
            Token(TokenType.OPEN_PARENTHESES, '('),

            Token(TokenType.I64, 'int64_t'),
            Token(TokenType.ID, 'viewHandleOrIndex'),
            Token(TokenType.COMMA, ','),

            Token(TokenType.I64, 'int64_t'),
            Token(TokenType.ID, 'associatedViewableObjectHandle'),
            Token(TokenType.COMMA, ','),

            Token(TokenType.I64, 'int64_t'),
            Token(TokenType.ID, 'options'),
            Token(TokenType.COMMA, ','),

            Token(TokenType.OPTION, 'std::optional'),
            Token(TokenType.LESS, '<'),
            Token(TokenType.STRING, 'std::string'),
            Token(TokenType.BIGGER, '>'),
            Token(TokenType.ID, 'viewLabel'),


            Token(TokenType.CLOSE_PARENTHESES, ')'),
            Token(TokenType.SEMICOLON, ';'),
        ]

        self.assert_scanner(scanner, stream, expected_tokens)

    def test_wait(self):
        """ Scanner: double wait(double dt, std::optional<bool> simulationTime = {});"""
        stream = StringStream(
            'double wait(double dt, std::optional<bool> simulationTime = {});')
        scanner = Scanner(stream)
        expected_tokens = [
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

        self.assert_scanner(scanner, stream, expected_tokens)

    def test_get_vision_sensor_depth_buffer(self):
        """ Scanner: std::tuple<std::vector<uint8_t>, std::vector<int64_t>> getVisionSensorDepthBuffer(int64_t sensorHandle, std::optional<std::vector<int64_t>> pos = {},std::optional<std::vector<int64_t>> size = {});
        """

        stream = StringStream(
            """ 
                std::tuple<std::vector<uint8_t>, std::vector<int64_t>> getVisionSensorDepthBuffer(int64_t sensorHandle, 
                                                                                                  std::optional<std::vector<int64_t>> pos = {},
                                                                                                  std::optional<std::vector<int64_t>> size = {});
            """)
        scanner = Scanner(stream)

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

        expected_tokens = [
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
        self.assert_scanner(scanner, stream, expected_tokens)

    def assert_scanner(self, scanner: Scanner, stream: Stream, expected_tokens: list[Token]):
        for expected_token in expected_tokens:
            token = scanner.current_token()
            scanner.update()
            self.assertEqual(token, expected_token)

        scanner.update()
        assert scanner.current_token() is None
