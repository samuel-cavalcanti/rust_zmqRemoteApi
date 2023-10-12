from dataclasses import dataclass
from pathlib import Path
from ir_transpiler import FunctionAssign


from scanner import Scanner
from stream import StringStream

from parser import parser
from ir_transpiler import ir_to_macro_request_rust, ir_to_py
import inflection


def cpp_to_rust(assigns: list[FunctionAssign], rust_file: Path, api_name: str) -> None:

    space = ' '*4

    trait_name = inflection.camelize(api_name)
    rust_assigns = [ir_to_macro_request_rust(
        assign, api_name) for assign in assigns]
    rust_string = ",\n".join(rust_assigns)
    content = 'use crate::RemoteApiClientInterface;\n'
    content += f'pub trait {trait_name} : RemoteApiClientInterface {{\n{space}requests!{{\n"{api_name}",\n{rust_string}\n}}\n}}'

    rust_file.write_text(content)


PYTHON_TEMPLATE = \
    """
from typing import Optional, Protocol

class {class_name}(Protocol):

    {functions}
"""


def cpp_to_py(assigns: list[FunctionAssign], py_file: Path, protocol_name: str) -> None:
    py_assigns = [ir_to_py(assign) for assign in assigns]

    assigns_string = "    \n    ".join(py_assigns)

    content = PYTHON_TEMPLATE.format(
        class_name=inflection.camelize(protocol_name),
        functions=assigns_string)

    py_file.write_text(content)


@dataclass
class API:
    header: Path
    api_name: str
    file_name: str


def main():
    assets = Path('assets')
    sim_ik_h = assets / Path('sim_ik_api_header.h')
    sim_h = assets / Path('sim_api_header.h')

    apis = [
        API(sim_ik_h, 'simIK', 'sim_ik_api'),
        API(sim_h, 'sim', 'sim_api'),
    ]

    for api in apis:
        content = api.header.read_text()
        stream = StringStream(content)
        scanner = Scanner(stream)

        assigns = parser(scanner, stream)

        rust_file = assets / Path(f'{api.file_name}.rs')
        python_file = assets / Path(f'{api.file_name}.py')

        cpp_to_rust(assigns, rust_file, api.api_name)
        cpp_to_py(assigns, python_file, api.api_name)


if __name__ == '__main__':
    main()
