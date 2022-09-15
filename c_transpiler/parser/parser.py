
from typing import Optional
from cpp_token import Token, TokenType, STD_CPP_PRIMITIVE_TYPES
from scanner import ScannerProtocol
from stream import Stream
from ir_transpiler import Arg, TypeNode, FunctionAssign


def digest(scanner: ScannerProtocol, stream: Stream, token_type: TokenType) -> Token:

    token = scanner.current_token()
    scanner.update()

    assert token is not None
    assert token.token_type == token_type, f'token:{token}, expected token_type: {token_type}, stream pos: {stream.pos()}'
    return token


def unwrap_token(scanner: ScannerProtocol, stream: Stream) -> Token:

    token = scanner.current_token()
    assert token is not None, f'unable to unwrap token,  {stream.pos()}, {stream.current_char()}'

    return token


def parser(scanner: ScannerProtocol, stream: Stream) -> list[FunctionAssign]:

    assigns = []

    while scanner.current_token():
        assign = cpp_function_assign(scanner, stream)
        assigns.append(assign)

    return assigns


"""
       cpp_function_assign = expression_type ID args;
       args = (arg')
       arg' = arg| arg, | e
       arg = expression_type id 
"""


def cpp_function_assign(scanner: ScannerProtocol, stream: Stream) -> FunctionAssign:

    return_type = expression_type(scanner, stream)

    function_name = unwrap_id(scanner, stream)
    scanner.update()

    args = parse_args(scanner, stream)

    digest(scanner, stream, TokenType.SEMICOLON)

    return FunctionAssign(return_type, function_name, args)


def unwrap_id(scanner: ScannerProtocol, stream: Stream) -> str:
    token = unwrap_token(scanner, stream)
    assert token.token_type == TokenType.ID, f'token {token}'
    assert token.content is not None
    return token.content


"""
    args = ([expression_type id],*)
"""


def parse_args(scanner: ScannerProtocol, stream: Stream) -> list[Arg]:

    args = []
    digest(scanner, stream, TokenType.OPEN_PARENTHESES)

    arg = parse_arg(scanner, stream)
    if arg:
        args.append(arg)
    else:
        digest(scanner, stream, TokenType.CLOSE_PARENTHESES)
        return []

    token = unwrap_token(scanner, stream)
    while token.token_type == TokenType.COMMA:
        digest(scanner, stream, TokenType.COMMA)
        arg = parse_arg(scanner, stream)
        assert arg is not None
        args.append(arg)

        token = unwrap_token(scanner, stream)

    digest(scanner, stream, TokenType.CLOSE_PARENTHESES)
    # assert token.token_type == TokenType.CLOSE_PARENTHESES, f'token: {token}'

    return args


def parse_arg(scanner: ScannerProtocol, stream: Stream) -> Optional[Arg]:
    token = unwrap_token(scanner, stream)

    if token.token_type == TokenType.CLOSE_PARENTHESES:
        return None

    arg_type = expression_type(scanner, stream)

    arg_name = unwrap_id(scanner, stream)
    digest(scanner, stream, TokenType.ID)

    return Arg(arg_type, arg_name)


def expression_type(scanner: ScannerProtocol, stream: Stream) -> TypeNode:
    """
    expression_type = (PrimitiveType | VecType | TupleType | OptionalType | ) )
    complex_type_expression = (VecType | TupleType | OptionalType )
    VecType   =  std::vector<PrimitiveType>
    TupleType = std::tuple<[expression_type],*>
    OptionalType = std::optional<expression_type>
    PrimitiveType = U8, I64, F64, VOID, JSON,STRING,
    """
    token = unwrap_token(scanner, stream)

    is_primitive = token.token_type in STD_CPP_PRIMITIVE_TYPES

    if is_primitive:
        digest(scanner, stream, token.token_type)
        return TypeNode(token.token_type, [])

    if token.token_type == TokenType.VEC:
        return vec_type(scanner, stream)

    if token.token_type == TokenType.TUPLE:
        return tuple_type(scanner, stream)

    if token.token_type == TokenType.OPTION:
        return option_type(scanner, stream)

    raise Exception(
        f'error on complex type, token: {token}, Steam pos: {stream.pos()} ')


"""
 VecType   =   std::vector<PrimitiveType>
"""


def vec_type(scanner: ScannerProtocol, stream: Stream) -> TypeNode:
    digest(scanner, stream, TokenType.VEC)
    digest(scanner, stream, TokenType.LESS)  # <
    primitive_type = unwrap_token(scanner, stream)

    digest(scanner, stream, primitive_type.token_type)
    digest(scanner, stream, TokenType.BIGGER)  # >

    sub_type = TypeNode(primitive_type.token_type, [])
    return TypeNode(TokenType.VEC, [sub_type])


"""
 TupleType = std::tuple<[expression_type'],+>
"""


def tuple_type(scanner: ScannerProtocol, stream: Stream):
    digest(scanner, stream, TokenType.TUPLE)
    digest(scanner, stream, TokenType.LESS)  # <

    type_node = expression_type(scanner, stream)

    sub_types: list[TypeNode] = [type_node]

    token = unwrap_token(scanner, stream)

    while token.token_type == TokenType.COMMA:
        digest(scanner, stream, TokenType.COMMA)
        node = expression_type(scanner, stream)
        sub_types.append(node)

        token = unwrap_token(scanner, stream)

    digest(scanner, stream, TokenType.BIGGER)

    return TypeNode(TokenType.TUPLE, sub_types)


"""
  OptionalType = std::optional<expression_type>
"""


def option_type(scanner: ScannerProtocol, stream: Stream) -> TypeNode:
    digest(scanner, stream, TokenType.OPTION)
    digest(scanner, stream, TokenType.LESS)  # <

    node = expression_type(scanner, stream)

    digest(scanner, stream, TokenType.BIGGER)  # <

    return TypeNode(TokenType.OPTION, [node])
