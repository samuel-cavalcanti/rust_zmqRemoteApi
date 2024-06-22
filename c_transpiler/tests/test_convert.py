import unittest
import convert


class ConvertTestCase(unittest.TestCase):
    def test_replace_function_name_to_markdown(self):
        descriptions = [
            "Deprecated. See sim.getJointTargetForce instead.",
            "Deprecated. See sim.setJointTargetForce instead.",
            "Deprecated. Use sim.createPrimitiveShape instead",
            "",
            "Deprecated (yes, what an irony!). Returns 0",
            "test sim.stuff"
        ]

        expecteds = [
            "Deprecated. See [sim_get_joint_target_force](#method.sim_get_joint_target_force) instead.",
            "Deprecated. See [sim_set_joint_target_force](#method.sim_set_joint_target_force) instead.",
            "Deprecated. Use [sim_create_primitive_shape](#method.sim_create_primitive_shape) instead",
            "",
            "Deprecated (yes, what an irony!). Returns 0",
            "test [sim_stuff](#method.sim_stuff)"
        ]

        for description, expecetd in zip(descriptions, expecteds):
            new_description = convert.replace_function_name_to_markdown_link(
                "sim", description
            )
            self.assertEqual(new_description, expecetd)

        pass
