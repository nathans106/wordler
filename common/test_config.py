from . import config
import unittest

class TestConfig(unittest.TestCase):
    def test_local(self):
        self.assertEqual(config.local()['project']['name'], 'wordler')

    def test_secret(self):
        self.assertEqual(config.secret('test'), 'safe')
