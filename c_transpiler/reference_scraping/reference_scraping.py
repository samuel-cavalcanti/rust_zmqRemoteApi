from pathlib import Path
import inflection
from bs4 import BeautifulSoup


def scraping_regular_api(function_names: list[str], coppeliasim_dir: Path) -> dict[str, str]:
    """Expecting function_names like: [ 'loadScene','setJointMaxForce'], and the simulator dir"""
    regular_api_dir = get_regular_api_dir(coppeliasim_dir)

    function_descriptions = {}

    for function_name in function_names:
        description = scraping_regular_api_function(
            function_name,
            regular_api_dir)
        function_descriptions[function_name] = description

    return function_descriptions


def scraping_regular_api_description(coppeliasim_dir: Path) -> str:
    en = get_regular_api_dir(coppeliasim_dir).parent
    index = en / 'apiFunctions.htm'
    soup = BeautifulSoup(index.read_text(), 'html.parser')
    description = _get_text_for_warning_box(soup)
    return description


def scraping_sim_ik_api_description(coppeliasim_dir: Path) -> str:
    sim_ik_file = get_sim_ik_file(coppeliasim_dir)
    soup = BeautifulSoup(sim_ik_file.read_text(), 'html.parser')
    description = _get_text_for_warning_box(soup)
    return description


def _get_text_for_warning_box(soup: BeautifulSoup) -> str:

    p = soup.find("p", {"class": "warningBox"})
    if p is None:
        raise Exception('Unable to find Regular API description')

    return p.text


def scraping_sim_ik_api(function_names: list[str], coppeliasim_dir: Path) -> dict[str, str]:

    function_descriptions = {}

    sim_ik_file = get_sim_ik_file(coppeliasim_dir)

    for name in function_names:
        description = scraping_sim_ik_api_function(name, sim_ik_file)
        function_descriptions[name] = description

    return function_descriptions


def get_regular_api_dir(coppeliasim_dir: Path) -> Path:
    return coppeliasim_dir/'helpFiles'/'en'/'regularApi'


def get_sim_ik_file(coppeliasim_dir: Path) -> Path:
    return coppeliasim_dir/'helpFiles'/'en'/'simIK.htm'


def scraping_regular_api_function(function_name: str, regular_api_dir: Path) -> str:

    name = inflection.camelize(function_name)
    reference_function_name = f'sim{name}'
    reference_file = regular_api_dir / f'{reference_function_name}.htm'
    if not reference_file.exists():
        raise Exception(
            f'Unable to find {function_name} and your file: {reference_file}')

    html = reference_file.read_text()
    description = _scraping_sim_api_html(html)

    return description


def scraping_sim_ik_api_function(function_name: str, sim_ik_file: Path) -> str:
    name = f'simIK.{function_name}'  # id="simIK.addIkElement"
    soup = BeautifulSoup(sim_ik_file.read_text(), 'html.parser')
    a_tag = soup.find("a", {"id": name})

    if a_tag is None:
        raise Exception(
            f'Unable to find {function_name} in SimIK')

    # getting the next table of p
    '''
        <p> <a id="SimIK.addElement"> </p>
        <table>
                <tr>
                    <td class="apiTableLeftDescr">Description</td>
                    <td class="apiTableRightDescr">Adds a new IK element to an IK group.</td>
                </tr>
                ...
        </table>
    '''
    p_tag = a_tag.parent

    if p_tag is None:
        raise Exception(
            f'Unable to find p tag of{function_name} in SimIK')
    table_tag = p_tag.next_sibling

    if table_tag is None:
        raise Exception(
            f'Unable to find table tag of{function_name} in SimIK')

    td_tag = table_tag.find_next("td", {"class": "apiTableRightDescr"})

    if td_tag is None:
        raise Exception(
            f'Unable to find description tag of{function_name} in SimIK')

    description = td_tag.text

    return description


def _scraping_sim_api_html(html: str) -> str:
    """returning the function description"""
    soup = BeautifulSoup(html, 'html.parser')

    description_td = soup.find("td", {"class": "apiTableRightDescr"})

    if description_td is None:
        raise Exception('Unable to find description by class')

    return description_td.text
