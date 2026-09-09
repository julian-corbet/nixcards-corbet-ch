# SPDX-License-Identifier: FSL-1.1-ALv2
import os
from pathlib import Path
import subprocess
import tempfile
import unittest


class CatalogSourceGuard(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tools = tempfile.TemporaryDirectory()
        cls.binary = Path(cls.tools.name) / "catalog-build"
        source = Path(__file__).parents[1] / "crates/nixcards-core/build.rs"
        subprocess.run(["rustc", str(source), "-o", str(cls.binary)], check=True)

    @classmethod
    def tearDownClass(cls):
        cls.tools.cleanup()

    def setUp(self):
        self.fixture = tempfile.TemporaryDirectory()
        self.root = Path(self.fixture.name)
        (self.root / "crates/core").mkdir(parents=True)
        (self.root / "cards/set").mkdir(parents=True)
        (self.root / "out").mkdir()
        (self.root / "cards/set/card.md").write_text("# Synthetic card fixture\n")

    def tearDown(self):
        self.fixture.cleanup()

    def build(self):
        environment = os.environ.copy()
        environment.update(CARGO_MANIFEST_DIR=str(self.root / "crates/core"), OUT_DIR=str(self.root / "out"))
        return subprocess.run([str(self.binary)], env=environment, capture_output=True, text=True, timeout=5)

    def test_regular_catalogue_is_bundled(self):
        result = self.build()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("cards/set/card.md", (self.root / "out/bundled_cards.rs").read_text())

    def test_directory_link_cannot_escape_catalogue(self):
        (self.root / "outside").mkdir()
        (self.root / "outside/card.md").write_text("outside catalogue")
        (self.root / "cards/escape").symlink_to(self.root / "outside", target_is_directory=True)
        result = self.build()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("must not be symbolic links", result.stderr)
        self.assertFalse((self.root / "out/bundled_cards.rs").exists())

    def test_markdown_link_cannot_embed_outside_file(self):
        (self.root / "outside.md").write_text("outside catalogue")
        (self.root / "cards/set/escape.md").symlink_to(self.root / "outside.md")
        result = self.build()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("must not be symbolic links", result.stderr)
        self.assertFalse((self.root / "out/bundled_cards.rs").exists())


if __name__ == "__main__":
    unittest.main()
