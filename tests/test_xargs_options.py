"""Dependency-free regression tests for the shared modern xargs backend.
Run: python3 -m unittest discover -s tests -p test_xargs_options.py
Builds only this module using the installed stable toolchain, not Linux applets.
"""
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest


class XargsOptions(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tmp = tempfile.TemporaryDirectory(prefix='rustybox-xargs-')
        cls.addClassCleanup(cls.tmp.cleanup)
        root = Path(cls.tmp.name)
        module = Path(__file__).resolve().parents[1] / 'modern/xargs.rs'
        source = root / 'main.rs'
        source.write_text(
            '#[path = ' + json.dumps(str(module)) + '] mod xargs;\n'
            'fn main() { let args: Vec<String> = std::env::args().collect();\n'
            'let refs: Vec<&str> = args.iter().map(String::as_str).collect();\n'
            'std::process::exit(xargs::run(&refs)); }\n'
        )
        cls.binary = root / 'xargs-test'
        subprocess.run(['rustc', '--edition=2021', str(source), '-o', str(cls.binary)],
                       check=True, capture_output=True, timeout=30,
                       env={**os.environ, "RUSTUP_TOOLCHAIN": "stable"})

    def invoke(self, options, data='one two three'):
        return subprocess.run([str(self.binary), *options], input=data, text=True,
                              capture_output=True, timeout=3)

    def test_invalid_values_never_invoke_command(self):
        for options in [['-n', '0'], ['-n0'], ['-n', 'bad'], ['-nbad'],
                        ['--max-args', '-1'], ['-n', '999999999999999999999999']]:
            with self.subTest(options=options):
                result = self.invoke([*options, '/bin/echo', 'EXECUTED'])
                self.assertNotEqual(result.returncode, 0)
                self.assertEqual(result.stdout, '')
                self.assertIn('max-args', result.stderr)

    def test_missing_value(self):
        for option in ['-n', '--max-args']:
            with self.subTest(option=option):
                result = self.invoke([option])
                self.assertNotEqual(result.returncode, 0)
                self.assertEqual(result.stdout, '')
                self.assertIn('max-args', result.stderr)

    def test_invalid_option_does_not_wait_for_stdin(self):
        child = subprocess.Popen([str(self.binary), '-n0'], stdin=subprocess.PIPE,
                                 stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        try:
            self.assertNotEqual(child.wait(timeout=3), 0)
        finally:
            if child.poll() is None:
                child.kill()
            child.communicate()

    def test_positive_batching(self):
        for options in [['-n', '2'], ['-n2'], ['--max-args', '2']]:
            with self.subTest(options=options):
                result = self.invoke([*options, '/bin/echo'])
                self.assertEqual(result.returncode, 0)
                self.assertEqual(result.stdout, 'one two\nthree\n')

    def test_null_input_and_option_terminator(self):
        result = self.invoke(['-0', '-n1', '--', '/bin/echo'], 'one two\x00three\x00')
        self.assertEqual(result.returncode, 0)
        self.assertEqual(result.stdout, 'one two\nthree\n')
