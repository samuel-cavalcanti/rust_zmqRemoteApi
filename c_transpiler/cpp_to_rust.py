from pathlib import Path

from scanner import Scanner
from stream import StringStream
from cpp_token import Token, TokenType
from parser import parser
from ir_transpiler import ir_to_macro_request_rust


def main():
    assets =Path('assets') 
    sim_ik_h = assets / Path('sim_ik_api_header.h')
    sim_h = assets /Path('sim_api_header.h')

    sim_ik = assets / Path('sim_ik.rs')
    sim = assets / Path('sim_api.rs')

    
    for header, rust_file in zip([sim_ik_h,sim_h],[sim_ik,sim]):

        content = header.read_text()
        stream = StringStream(content)
        scanner = Scanner(stream)

        assigns = parser(scanner, stream)
        
        rust_assigns = [ir_to_macro_request_rust(assign) for assign in assigns]
        rust_string = ",\n".join(rust_assigns)

        content = f'requests!{{\n{rust_string}\n}}'
        
        
        rust_file.write_text(content)



if __name__ == '__main__':
    main()
