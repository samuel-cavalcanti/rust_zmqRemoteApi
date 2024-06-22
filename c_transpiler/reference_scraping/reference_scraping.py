from dataclasses import dataclass
from pathlib import Path
import inflection
from bs4 import BeautifulSoup, ResultSet, Tag


@dataclass
class Description:
    note: str
    args: list[str]
    return_values: list[str]


def scraping_regular_api(
    function_names: list[str], coppeliasim_dir: Path
) -> dict[str, Description]:
    """Expecting function_names like: [ 'loadScene','setJointMaxForce'], and the simulator dir"""
    regular_api_dir = get_regular_api_dir(coppeliasim_dir)

    function_descriptions = {}

    for function_name in function_names:
        description = scraping_regular_api_function(function_name, regular_api_dir)
        function_descriptions[function_name] = description

    return function_descriptions


def scraping_regular_api_description(coppeliasim_dir: Path) -> str:
    en = get_regular_api_dir(coppeliasim_dir).parent
    index = en / "apiFunctions.htm"
    soup = BeautifulSoup(index.read_text(), "html.parser")
    description = _get_text_for_warning_box(soup)
    return description


def scraping_sim_ik_api_description(coppeliasim_dir: Path) -> str:
    sim_ik_file = get_sim_ik_file(coppeliasim_dir)
    soup = BeautifulSoup(sim_ik_file.read_text(), "html.parser")
    description = _get_text_for_warning_box(soup)
    return description


def _get_text_for_warning_box(soup: BeautifulSoup) -> str:
    p = soup.find("p", {"class": "warningBox"})
    if p is None:
        raise Exception("Unable to find Regular API description")

    return p.text


def scraping_sim_ik_api(
    function_names: list[str], coppeliasim_dir: Path
) -> dict[str, str]:
    function_descriptions = {}

    sim_ik_file = get_sim_ik_file(coppeliasim_dir)

    for name in function_names:
        description = scraping_sim_ik_api_function(name, sim_ik_file)
        function_descriptions[name] = description

    return function_descriptions


ENGLISH_HELP_FILES_DIR = Path("manual") / "en"


def get_regular_api_dir(coppeliasim_dir: Path) -> Path:
    return coppeliasim_dir / ENGLISH_HELP_FILES_DIR / "regularApi"


def get_sim_ik_file(coppeliasim_dir: Path) -> Path:
    return coppeliasim_dir / ENGLISH_HELP_FILES_DIR / "simIK.htm"


def scraping_regular_api_function(
    function_name: str, regular_api_dir: Path
) -> Description:
    name = inflection.camelize(function_name)
    reference_function_name = f"sim{name}"
    reference_file = regular_api_dir / f"{reference_function_name}.htm"
    if not reference_file.exists():
        raise Exception(
            f"Unable to find {function_name} and your file: {reference_file}"
        )

    html = reference_file.read_text()
    description = _scraping_sim_api_html(html)

    return description


def scraping_sim_ik_api_function(function_name: str, sim_ik_file: Path) -> str:
    name = f"simIK.{function_name}"  # id="simIK.addIkElement"
    soup = BeautifulSoup(sim_ik_file.read_text(), "html.parser")
    a_tag = soup.find("a", {"id": name})

    if a_tag is None:
        raise Exception(f"Unable to find {function_name} in SimIK")

    # getting the next table of p
    """
        <p> <a id="SimIK.addElement"> </p>
        <table>
                <tr>
                    <td class="apiTableLeftDescr">Description</td>
                    <td class="apiTableRightDescr">Adds a new IK element to an IK group.</td>
                </tr>
                ...
        </table>
    """
    p_tag = a_tag.parent

    if p_tag is None:
        raise Exception(f"Unable to find p tag of{function_name} in SimIK")
    table_tag = p_tag.next_sibling

    if table_tag is None:
        raise Exception(f"Unable to find table tag of{function_name} in SimIK")

    td_tag = table_tag.find_next("td", {"class": "apiTableRightDescr"})

    if td_tag is None:
        raise Exception(f"Unable to find description tag of{function_name} in SimIK")

    description = td_tag.text

    return description


def _scraping_sim_api_html(html: str) -> Description:
    """returning the function description"""
    soup = BeautifulSoup(html, "html.parser")

    description_td = soup.find("td", {"class": "apiTableRightDescr"})

    if description_td is None:
        return _scraping_new_sim_api_html(html)

    return Description(note=description_td.text, return_values=[], args=[])


def _scraping_new_sim_api_html(html: str) -> Description:
    """returning the function description"""
    soup = BeautifulSoup(html, "html.parser")

    divs: ResultSet[Tag] = soup.find_all("div", {"class": "regApiSection"})
    assert len(divs) == 5, f"should have 5 divs with regApiSection size:{len(divs)}"

    description_div = divs[0]
    description_p = description_div.find("p")
    assert description_p is not None
    note = description_p.text
    args_div = divs[2]
    args = [li.text for li in args_div.find_all("li")]
    return_values_div = divs[3]
    return_values = [li.text for li in return_values_div.find_all("li")]

    return Description(note=note, args=args, return_values=return_values)
