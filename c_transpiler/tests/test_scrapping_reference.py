import unittest
from pathlib import Path
from reference_scraping import reference_scraping


TEST_ASSETS_DIR = Path(__file__).parent/'test_assets'


class ReferenceScrapingTestCase(unittest.TestCase):

    def test_scraping_regular_api(self):

        descriptions = [
            'Closes current scene, and switches to another open scene.\n                                    If there is no other open scene, a new scene is then created. Can only be called\n                                    from an add-on, or from the sanbox script, when called\n                                    from Lua. See also sim.loadScene and sim.saveScene.',
            'Loads a previously saved scene.See also sim.saveScene, sim.loadModel, simCloseScene and sim.setBoolParam with sim.boolparam_scene_and_model_load_messages.\n                                    '
        ]

        function_names = ['closeScene', 'loadScene']

        result = reference_scraping.scraping_regular_api(function_names,
                                                          coppeliasim_dir=TEST_ASSETS_DIR)

        for name, description in zip(function_names, descriptions):
            self.assertEqual(result[name], description)

    def test_scraping_sim_ik(self):

        descriptions = [
            'Adds a new IK element to an IK group.',
            'Sets flags of an IK group.'
        ]

        function_names = ['addElement', 'setGroupFlags']

        result = reference_scraping.scraping_sim_ik_api(function_names,
                                                         TEST_ASSETS_DIR)

        for name, description in zip(function_names, descriptions):
            self.assertEqual(result[name], description)

    def test_get_sim_description(self):
        description = reference_scraping.scraping_regular_api_description(
            TEST_ASSETS_DIR)

        self.assertEqual(description, 'The list of API functions below allows you to access many CoppeliaSim parameters. There are however too many parameters in CoppeliaSim to have a specific API function for each one of them. Auxiliary parameters can be accessed via a set of given functions that use object parameter IDs. Refer also to  the global parameter IDs.\nAll units going to, or coming from the API are in meters, kilograms, seconds and radians or a combination of those (unless otherwise explicitly indicated).\n')

    def test_get_sim_ik_description(self):
        description = reference_scraping.scraping_sim_ik_api_description(
            TEST_ASSETS_DIR)

        self.assertEqual(
            description, 'API functions for creating kinematics tasks. All units, unless otherwise indicated, are specified in meters and radians.')

        pass
