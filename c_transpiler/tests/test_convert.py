import unittest
import convert


class ConvertTestCase(unittest.TestCase):
    def test_change_description_function_to_rust_function(self):
        descriptions = [
            "Deprecated. See sim.getJointTargetForce instead.",
            "Deprecated. See sim.setJointTargetForce instead.",
            "Deprecated. Use sim.createPrimitiveShape instead",
            "",
            "Deprecated (yes, what an irony!). Returns 0",
        ]

        expecteds = [
            "Deprecated. See sim_get_joint_target_force instead.",
            "Deprecated. See sim_set_joint_target_force instead.",
            "Deprecated. Use sim_create_primitive_shape instead",
            "",
            "Deprecated (yes, what an irony!). Returns 0",
        ]

        for description, expecetd in zip(descriptions, expecteds):
            new_description = convert.description_function_to_rust_function(
                "sim", description
            )
            self.assertEqual(new_description, expecetd)

        pass
