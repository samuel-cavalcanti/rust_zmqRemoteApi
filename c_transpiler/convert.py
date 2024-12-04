from dataclasses import dataclass
from pathlib import Path
from ir_transpiler import FunctionAssign
from reference_scraping import reference_scraping


from scanner import Scanner
from stream import StringStream

from parser import parser
from ir_transpiler import ir_to_macro_request_rust, ir_to_py
import inflection
import sys
import logging


@dataclass
class API:
    header: Path
    api_name: str
    file_name: str
    description: str


def cpp_to_rust(assigns: list[FunctionAssign], rust_file: Path, api: API) -> None:
    space = " " * 4

    api_name = api.api_name

    trait_name = inflection.camelize(api_name)
    rust_assigns = [
        ir_to_macro_request_rust.ir_to_macro_request_rust(assign, api_name)
        for assign in assigns
    ]
    rust_string = ",\n".join(rust_assigns)
    content = "use crate::RemoteApiClientInterface;\n"
    content += f"""#[doc=r#"{api.description}"#]\n"""
    content += f'pub trait {trait_name} : RemoteApiClientInterface {{\n{space}requests!{{\n"{api_name}",\n{rust_string}\n}}\n}}'

    rust_file.write_text(content)


PYTHON_TEMPLATE = """
from typing import Optional, Protocol

class {class_name}(Protocol):

    {functions}
"""


def cpp_to_py(assigns: list[FunctionAssign], py_file: Path, protocol_name: str) -> None:
    py_assigns = [ir_to_py(assign) for assign in assigns]

    assigns_string = "    \n    ".join(py_assigns)

    content = PYTHON_TEMPLATE.format(
        class_name=inflection.camelize(protocol_name), functions=assigns_string
    )

    py_file.write_text(content)


def main():
    assets = Path("assets")
    sim_ik_h = assets / Path("sim_ik_api_header.h")
    sim_h = assets / Path("sim_api_header.h")

    if len(sys.argv) < 2:
        print("you shoud pass the coppeliasim dir as argument")
        exit(1)
    coppeliasim_dir = Path(sys.argv[1])
    regular_api_dir = reference_scraping.get_regular_api_dir(coppeliasim_dir)
    sim_ik_reference_file = reference_scraping.get_sim_ik_file(coppeliasim_dir)

    sim_ik_description = reference_scraping.scraping_sim_ik_api_description(
        coppeliasim_dir
    )
    sim_description = reference_scraping.scraping_regular_api_description(
        coppeliasim_dir
    )
    apis = [
        API(sim_ik_h, "simIK", "sim_ik_api", sim_ik_description),
        API(sim_h, "sim", "sim_api", sim_description),
    ]

    def sim_function_description(
        assign: FunctionAssign,
    ) -> reference_scraping.Description:
        try:
            descrip = reference_scraping.scraping_regular_api_function(
                assign.function_name, regular_api_dir
            )
            logging.info(f"reference found {assign.function_name}")
        except Exception:
            descrip = reference_scraping.Description(note="", args=[], return_values=[])
            logging.warning(f"Unable to find reference for sim.{assign.function_name}")
        return descrip

    def sim_ik_function_description(
        assign: FunctionAssign,
    ) -> reference_scraping.Description:
        try:
            descrip = reference_scraping.scraping_sim_ik_api_function(
                assign.function_name, sim_ik_reference_file
            )
        except Exception:
            descrip = ""
            # print(f"Unable to find reference for SimIK {assign.function_name}")
        return reference_scraping.Description(note=descrip, args=[], return_values=[])

    descriptions = {
        apis[0].api_name: sim_ik_function_description,
        apis[1].api_name: sim_function_description,
    }

    for api in apis:
        content = api.header.read_text()
        stream = StringStream(content)
        scanner = Scanner(stream)

        assigns = parser(scanner, stream)

        for assign in assigns:
            description = descriptions[api.api_name](assign)
            assign.description = description_function_to_rust_function(
                api.api_name, description
            )

        rust_file = assets / Path(f"{api.file_name}.rs")
        # python_file = assets / Path(f"{api.file_name}.py")

        cpp_to_rust(assigns, rust_file, api)
        # cpp_to_py(assigns, python_file, api.api_name)


def description_function_to_rust_function(
    api_name: str, description: reference_scraping.Description
) -> str:
    note = replace_function_name_to_markdown_link(api_name, description.note)

    markdown = note

    # def to_mark_list(values):
    #     return "\n".join([f"- {v}" for v in values])
    # args = to_mark_list(description.args)
    # returns = to_mark_list(description.return_values)
    # if len(args) > 0:
    #     markdown += f"**### Arguments\n{args}\n"
    #
    # if len(returns) > 0:
    #     markdown += f"### Return values\n{returns}"

    return markdown


def replace_function_name_to_markdown_link(api_name: str, note: str) -> str:
    descrip_size = len(note)
    if descrip_size == 0:
        return note
    pos = note.find("sim.")
    if pos < 0:
        return note

    end = pos + len("sim.")
    while end < descrip_size and note[end].isalnum():
        end += 1

    function_name = note[pos:end]

    rust_name = ir_to_macro_request_rust.rust_function_name_style(
        api_name, function_name[len("sim.") :]
    )

    docs_function_name = f"[{rust_name}](#method.{rust_name})"

    return note.replace(function_name, docs_function_name)


if __name__ == "__main__":
    main()
