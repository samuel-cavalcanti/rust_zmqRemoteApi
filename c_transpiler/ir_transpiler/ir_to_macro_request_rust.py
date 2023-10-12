from cpp_token import TokenType, STD_CPP_PRIMITIVE_TYPES
from .ir import FunctionAssign, Arg, TypeNode
import inflection


def ir_to_macro_request_rust(assign: FunctionAssign, api_name: str) -> str:
    return_type = type_node_to_rust(assign.return_type)
    rust_func_name = f'{inflection.underscore(api_name)}_{inflection.underscore(assign.function_name)}'

    required_args = []
    option_args = []

    for arg in assign.function_args:
        string_arg = arg_to_rust(arg)
        if arg.arg_type.cpp_type == TokenType.OPTION:
            option_args.append(string_arg)
        else:
            required_args.append(string_arg)

    required_args_string = f',({",".join(required_args)})'if len(
        required_args) != 0 else ""

    opt_string = f',opt({",".join(option_args)})' if len(
        option_args) != 0 else ""

    return_type_string = f'->{return_type}' if return_type else ""
    return f'({rust_func_name},"{assign.function_name}"{required_args_string}{opt_string}{return_type_string})'


def arg_to_rust(arg: Arg) -> str:

    if arg.arg_type.cpp_type == TokenType.OPTION:
        sub_type = arg.arg_type.sub_type[0]
        type_string = type_node_to_rust(sub_type)
    else:
        type_string = type_node_to_rust(arg.arg_type)

    snake_case = inflection.underscore(arg.arg_name)

    return f'{snake_case}:{type_string}'


def type_node_to_rust(node: TypeNode) -> str:
    cpp_tokens_eq = {
        TokenType.U8: "u8",
        TokenType.I64: "i64",
        TokenType.F64: "f64",
        TokenType.BOOL: "bool",
        TokenType.VOID:  "()",
        TokenType.TUPLE: "",
        TokenType.VEC: "Vec",
        TokenType.OPTION: "Option",
        TokenType.STRING: "String",
        TokenType.JSON: "serde_json::Value"
    }

    cpp_type = cpp_tokens_eq.get(node.cpp_type)
    assert cpp_type is not None

    if node.cpp_type in STD_CPP_PRIMITIVE_TYPES:
        return cpp_type

    else:
        sub_types = [type_node_to_rust(child) for child in node.sub_type]
        sub_types_string = ",".join(sub_types)

        if node.cpp_type == TokenType.TUPLE:
            return f'({sub_types_string})'

        return f'{cpp_type}<{sub_types_string}>'
