import json
from pathlib import Path
import inflection

RUST_CONST_TEMPLATE = "pub const {name}: i64 = {value};"


def main():
    assets = Path('assets')
    json_file = assets / 'consts.json'
    sim_const_file = assets / 'sim_const.rs'

    json_content = json.loads(json_file.read_bytes())

    rust_content = ''
    for name, value in json_content["sim"].items():
        rust_name = inflection.underscore(name).upper()
        rust_const = RUST_CONST_TEMPLATE.format(name=rust_name, value=value)
        rust_content += f'{rust_const}\n\n'

    # removing last end line
    rust_content = rust_content[:-1]

    sim_const_file.write_text(rust_content)


if __name__ == "__main__":
    main()
    pass
