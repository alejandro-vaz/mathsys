#
#   HEAD
#

# HEAD -> UNITTEST
import unittest

# HEAD -> JSON
import json

# HEAD -> ENTRY POINT
import mathsys


#
#   TESTING
#

# TESTING -> CLASS
class Test(unittest.TestCase):
    def testView(self):
        with open("python/testing/test.json", "r") as file: content = json.load(file)
        for index, (stdin, expected) in enumerate(content):
            with self.subTest(i = index, inp = stdin):
                self.assertEqual(mathsys.view(stdin), expected)