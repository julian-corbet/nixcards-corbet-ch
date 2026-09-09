# SPDX-License-Identifier: FSL-1.1-ALv2
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest


class StagedCardsSource(unittest.TestCase):
    def setUp(self):
        self.fixture = tempfile.TemporaryDirectory()
        self.addCleanup(self.fixture.cleanup)
        self.root = Path(self.fixture.name)
        (self.root / ".ci").mkdir()
        shutil.copyfile(Path(__file__).parents[1] / ".ci/prepare-cards.sh",
                        self.root / ".ci/prepare-cards.sh")
        shutil.copyfile(Path(__file__).parents[1] / ".ci/archives.toml",
                        self.root / ".ci/archives.toml")
        (self.root / "bin").mkdir()
        git = self.root / "bin/git"
        git.write_text("#!/bin/sh\ntouch git-was-called\nexit 91\n")
        git.chmod(0o755)
        self.tool = self.root / "bin/ccid"
        self.tool.write_text("""#!/usr/bin/env python3
import json, os, pathlib, sys
pathlib.Path('tool-arguments.json').write_text(json.dumps(sys.argv[1:]))
if os.environ.get('FAIL_VERIFY'):
    raise SystemExit(23)
if not os.environ.get('OMIT_CATALOG'):
    pathlib.Path('cards/catalog.json').write_text('{}')
""")
        self.tool.chmod(0o755)

    def run_loader(self, **overrides):
        environment = os.environ.copy()
        environment.update(PATH=str(self.root / "bin") + os.pathsep + environment["PATH"],
                           CI_TOOL_BINARY=str(self.tool), CARDS_SOURCE_ARCHIVE="fixture.tar",
                           CARDS_SOURCE_SHA256="a" * 64)
        environment.update(overrides)
        return subprocess.run(["bash", ".ci/prepare-cards.sh"], cwd=self.root,
                              env=environment, capture_output=True, text=True, timeout=5)

    def test_staged_source_uses_exact_pin_without_git(self):
        result = self.run_loader()
        self.assertEqual(result.returncode, 0, result.stderr)
        arguments = json.loads((self.root / "tool-arguments.json").read_text())
        self.assertEqual(arguments, ["verify-source", "--archive", "fixture.tar", "--sha256", "a" * 64,
                                    "--commit", "a3c4d7837fe5abf852e3d861efa0fdf3802ce751",
                                    "--destination", "cards"])
        self.assertFalse((self.root / "git-was-called").exists())

    def test_partial_archive_input_fails_without_network_fallback(self):
        result = self.run_loader(CARDS_SOURCE_SHA256="")
        self.assertNotEqual(result.returncode, 0)
        self.assertFalse((self.root / "git-was-called").exists())
        self.assertFalse((self.root / "tool-arguments.json").exists())

    def test_verification_failure_does_not_fetch_a_replacement(self):
        result = self.run_loader(FAIL_VERIFY="1")
        self.assertEqual(result.returncode, 23)
        self.assertFalse((self.root / "git-was-called").exists())

    def test_verified_source_must_contain_catalog(self):
        self.assertNotEqual(self.run_loader(OMIT_CATALOG="1").returncode, 0)

    def test_existing_cards_are_preserved(self):
        (self.root / "cards").mkdir()
        existing = self.root / "cards/user.md"
        existing.write_text("preserve this work")
        self.assertNotEqual(self.run_loader().returncode, 0)
        self.assertEqual(existing.read_text(), "preserve this work")
        self.assertFalse((self.root / "tool-arguments.json").exists())
        self.assertFalse((self.root / "git-was-called").exists())


if __name__ == "__main__":
    unittest.main()
