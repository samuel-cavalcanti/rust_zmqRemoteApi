from pathlib import Path

from scanner import Scanner
from stream import StringStream
from cpp_token import Token, TokenType
from parser import parser
from ir_transpiler import ir_to_macro_request_rust


def main():
    assets =Path('assets') 
    header = assets / Path('remote_api_header.h')
    output = assets / Path('output.rs')

    content = header.read_text()
    stream = StringStream(content)
    scanner = Scanner(stream)

    assigns = parser(scanner, stream)
    
    rust_assigns = [ir_to_macro_request_rust(assign) for assign in assigns]
    rust_string = ",\n".join(rust_assigns)

    content = f'requests!{{\n{rust_string}\n}}'
    
    
    output.write_text(content)

    

   

    # irs = [ir_to_string(assign) for assign in assigns]

    # cpp = [ir_to_cpp(assign) for assign in assigns]

    # output_content = "\n".join(cpp)

    # assert output_content == expected_h.read_text()

    # print(output_content,end='')


if __name__ == '__main__':
    main()
