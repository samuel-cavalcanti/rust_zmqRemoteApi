from cpp_token import  TokenType, STD_CPP_PRIMITIVE_TYPES
from .ir import  FunctionAssign, Arg, TypeNode

def ir_to_cpp(assign: FunctionAssign)->str:
    return_type = type_node_to_cpp(assign.return_type)
    args = [arg_to_cpp(arg) for arg in assign.function_args]
    args_string = ", ".join(args) if len(args) != 0 else ""
    return f'{return_type} {assign.function_name}({args_string});'

def arg_to_cpp(arg:Arg)->str:
    type_string = type_node_to_cpp(arg.arg_type)
   
    if arg.arg_type.cpp_type == TokenType.OPTION:
         return f'{type_string} {arg.arg_name} = ' +'{}'
    return f'{type_string} {arg.arg_name}'

def type_node_to_cpp(node: TypeNode) -> str:
    cpp_tokens_eq = {
        TokenType.U8: 'uint8_t',
        TokenType.I64: 'int64_t',
        TokenType.F64: 'double',
        TokenType.BOOL: 'bool',
        TokenType.VOID:  'void',
        TokenType.TUPLE: 'std::tuple',
        TokenType.VEC:  'std::vector',
        TokenType.OPTION: 'std::optional',
        TokenType.STRING:'std::string',
        TokenType.JSON: 'json'
    }

    cpp_type = cpp_tokens_eq.get(node.cpp_type)
    assert cpp_type is not None

    if node.cpp_type in STD_CPP_PRIMITIVE_TYPES:
        return cpp_type

    else:
        sub_types = [type_node_to_cpp(child) for child in node.sub_type]
        sub_types_string = ", ".join(sub_types)
    

        return f'{cpp_type}<{sub_types_string}>'
