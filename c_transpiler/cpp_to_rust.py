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



if __name__ == '__main__':
    main()
