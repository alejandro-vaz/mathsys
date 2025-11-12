#^
#^  HEAD
#^

#> HEAD -> MODULES
import unittest
import json

#> HEAD -> TESTER
import mathsys


#^
#^  TESTING
#^

#> TESTING -> CLASS
class Test(unittest.TestCase):
    def testLatex(self) -> None:
        with open("python/testing/test.json", "r") as file: content = json.load(file)
        for index, (stdin, expected) in enumerate(content):
            with self.subTest(i = index, inp = stdin): self.assertEqual(mathsys.view(stdin), expected)