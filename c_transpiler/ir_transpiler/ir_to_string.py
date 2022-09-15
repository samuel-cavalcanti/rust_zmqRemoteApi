from cpp_token import  TokenType, STD_CPP_PRIMITIVE_TYPES
from .ir import  FunctionAssign, Arg, TypeNode


def type_node_string(node: TypeNode) -> str:

    if node.cpp_type in STD_CPP_PRIMITIVE_TYPES:
        return node.cpp_type.name

    else:
        sub_types = [type_node_string(child) for child in node.sub_type]
        sub_types_string = ", ".join(sub_types)
        return f"{node.cpp_type.name}[{sub_types_string}]"


def arg_string(arg: Arg) -> str:
    type_string = type_node_string(arg.arg_type)
    return f'{type_string} {arg.arg_name}'


def ir_to_string(assign: FunctionAssign) -> str:
        return_type = type_node_string(assign.return_type)
        args = [arg_string(arg) for arg in assign.function_args]
        args_string = ", ".join(args) if len(args) != 0 else ""

        return f'{return_type} {assign.function_name}({args_string})'