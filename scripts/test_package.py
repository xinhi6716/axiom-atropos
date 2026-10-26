"""Verify the publication boundary against a real disposable Git repository."""
from pathlib import Path
import subprocess
import tempfile
import unittest
import zipfile

from package import build_archive


class PackageTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.git('init', '-q')
        self.git('config', 'user.name', 'Test')
        self.git('config', 'user.email', 'test@example.invalid')
        (self.root / 'public.txt').write_text('published')
        (self.root / '.gitignore').write_text('private/\n')
        self.git('add', '.')
        self.git('commit', '-qm', 'fixture')
        self.output = self.root / 'release.zip'

    def git(self, *args):
        return subprocess.run(['git', *args], cwd=self.root, check=True, capture_output=True)

    def test_untracked_and_ignored_files_never_ship(self):
        (self.root / 'notes.txt').write_text('unpublished')
        (self.root / 'private').mkdir()
        (self.root / 'private/secret.txt').write_text('private fixture')
        build_archive(self.root, self.output)
        with zipfile.ZipFile(self.output) as archive:
            self.assertEqual(set(archive.namelist()), {'.gitignore', 'public.txt'})
            self.assertEqual(archive.read('public.txt'), b'published')

    def test_dirty_tracked_files_block_release(self):
        (self.root / 'public.txt').write_text('not committed')
        with self.assertRaises(RuntimeError):
            build_archive(self.root, self.output)
        self.assertFalse(self.output.exists())

    def test_same_commit_has_identical_archive(self):
        build_archive(self.root, self.output)
        first = self.output.read_bytes()
        build_archive(self.root, self.output)
        self.assertEqual(first, self.output.read_bytes())


if __name__ == '__main__':
    unittest.main()
