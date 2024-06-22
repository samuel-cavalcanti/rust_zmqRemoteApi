import unittest
from pathlib import Path
from reference_scraping import reference_scraping


TEST_ASSETS_DIR = Path(__file__).parent / "test_assets"


class ReferenceScrapingTestCase(unittest.TestCase):
    def test_scraping_regular_api(self):
        descriptions = [
            reference_scraping.Description(
                note="Closes current scene, and switches to another open scene. If there is no other open scene,\na new scene is then created. Can only be called from an add-on,\nor from the sanbox script, when called from within CoppeliaSim",
                args=[],
                return_values=["result: the current scene index."],
            ),
            reference_scraping.Description(
                note="Loads a previously saved scene",
                args=[
                    'filename: scene buffer or scene filename, including extension. An empty string loads the default scene. By default, the current scene is overwritten (append "@keepCurrent" to the filename to preserve current scene). An empty string loads the default scene'
                ],
                return_values=[],
            ),
        ]

        function_names = ["closeScene", "loadScene"]

        result = reference_scraping.scraping_regular_api(
            function_names, coppeliasim_dir=TEST_ASSETS_DIR
        )

        for name, expected_description in zip(function_names, descriptions):
            description = result[name]

            self.assertEqual(description.note, expected_description.note)
            self.assertEqual(description.args, expected_description.args)
            self.assertEqual(
                description.return_values, expected_description.return_values
            )

    def test_scraping_sim_ik(self):
        descriptions = [
            "Adds a new IK element to an IK group.",
            "Sets flags of an IK group.",
        ]

        function_names = ["addElement", "setGroupFlags"]

        result = reference_scraping.scraping_sim_ik_api(function_names, TEST_ASSETS_DIR)

        for name, description in zip(function_names, descriptions):
            self.assertEqual(result[name], description)

    def test_get_sim_description(self):
        description = reference_scraping.scraping_regular_api_description(
            TEST_ASSETS_DIR
        )

        self.assertEqual(
            description,
            "The list of API functions below allows you to access many CoppeliaSim parameters. There are however too many parameters in CoppeliaSim to have a specific API function for each one of them. Auxiliary parameters can be accessed via a set of given functions that use object parameter IDs. Refer also to  the global parameter IDs.\nAll units going to, or coming from the API are in meters, kilograms, seconds and radians or a combination of those (unless otherwise explicitly indicated).\n",
        )

    def test_get_sim_ik_description(self):
        description = reference_scraping.scraping_sim_ik_api_description(
            TEST_ASSETS_DIR
        )

        self.assertEqual(
            description,
            "API functions for creating kinematics tasks. All units, unless otherwise indicated, are specified in meters and radians.",
        )

        pass
