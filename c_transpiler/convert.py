from pathlib import Path
from ir_transpiler import FunctionAssign


from scanner import Scanner
from stream import StringStream

from parser import parser
from ir_transpiler import ir_to_macro_request_rust, ir_to_py
import inflection


def cpp_to_rust(assigns: list[FunctionAssign], rust_file: Path) -> None:

    rust_assigns = [ir_to_macro_request_rust(assign) for assign in assigns]
    rust_string = ",\n".join(rust_assigns)
    file_name = rust_file.name.split(".")[0]
    struct_name = inflection.camelize(file_name)
    content = f' impl {struct_name} {{\n requests!{{\n"{file_name}",\n{rust_string}\n}}\n}}'

    rust_file.write_text(content)


PYTHON_TEMPLATE = \
    """
from typing import Optional, Protocol

class {class_name}(Protocol):

    {functions}
"""


def cpp_to_py(assigns: list[FunctionAssign], py_file: Path) -> None:
    py_assigns = [ir_to_py(assign) for assign in assigns]

    assigns_string = "    \n    ".join(py_assigns)

    file_name = py_file.name.split(".")[0]

    content = PYTHON_TEMPLATE.format(
        class_name=inflection.camelize(file_name),
        functions=assigns_string)

    py_file.write_text(content)


def main():
    assets = Path('assets')
    sim_ik_h = assets / Path('sim_ik_api_header.h')
    sim_h = assets / Path('sim_api_header.h')

    for header, file_name in zip([sim_ik_h, sim_h], ["sim_ik", "sim"]):
        content = header.read_text()
        stream = StringStream(content)
        scanner = Scanner(stream)

        assigns = parser(scanner, stream)

        rust_file = assets / Path(f'{file_name}.rs')
        python_file = assets / Path(f'{file_name}.py')

        cpp_to_rust(assigns, rust_file)
        cpp_to_py(assigns, python_file)


if __name__ == '__main__':
    main()
