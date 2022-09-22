from cpp_token import  TokenType, STD_CPP_PRIMITIVE_TYPES
from .ir import  FunctionAssign, Arg, TypeNode

PY_FUNCTION_TEMPLATE = \
"""def {fun_name}({args})->{return_type}:
        ...
"""

def ir_to_py(assign: FunctionAssign)->str:

    return_type = type_node_to_py(assign.return_type)
    args = [arg_to_py(arg) for arg in assign.function_args]

    args_string = ", ".join(args) 

    args_string = f'self,{args_string}' if args_string  else  'self'   
   

    return PY_FUNCTION_TEMPLATE.format(fun_name=assign.function_name,args=args_string,return_type=return_type)
    
def arg_to_py(arg:Arg)->str:
    type_string = type_node_to_py(arg.arg_type)
    
    string = f'{arg.arg_name}:{type_string}'
    if arg.arg_type.cpp_type == TokenType.OPTION:
         return  f'{string} = None'
    return string

def type_node_to_py(node: TypeNode) -> str:
    py_tokens_eq = {
        TokenType.U8: 'int',
        TokenType.I64: 'int',
        TokenType.F64: 'float',
        TokenType.BOOL: 'bool',
        TokenType.VOID:  'None',
        TokenType.TUPLE: 'tuple',
        TokenType.VEC:  'list',
        TokenType.OPTION: 'Optional',
        TokenType.STRING:'str',
        TokenType.JSON: 'dict'
    }

    py_type = py_tokens_eq.get(node.cpp_type)
    assert py_type is not None

    if node.cpp_type in STD_CPP_PRIMITIVE_TYPES:
        return py_type

    else:
        sub_types = [type_node_to_py(child) for child in node.sub_type]
        sub_types_string = ", ".join(sub_types)
    

        return f'{py_type}[{sub_types_string}]'
