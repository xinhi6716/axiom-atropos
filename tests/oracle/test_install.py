"""Exercise ownership and update behavior in disposable host homes."""
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
from install import HOSTS, install_skills


class InstallationTests(unittest.TestCase):
    def setUp(self):
        env_patch = patch.dict('os.environ', {}, clear=True)
        env_patch.start()
        self.addCleanup(env_patch.stop)
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.source = self.root / 'source'
        for directory in ('skills/example', 'pstack-resources'):
            (self.source / directory).mkdir(parents=True)
        (self.source / 'skills/example/SKILL.md').write_text('version one')
        (self.source / 'pstack-resources/HOSTS.md').write_text('resource')
        (self.source / 'plugin.json').write_text(json.dumps({'version': '1.0.0'}))

    def test_all_hosts_install_update_preserve_settings(self):
        for host, directory in HOSTS.items():
            with self.subTest(host=host):
                home = self.root / host
                base = home / directory
                base.mkdir(parents=True)
                settings = base / 'settings.json'
                settings.write_text('personal settings')
                install_skills(self.source, host, home)
                install_skills(self.source, host, home)
                self.assertEqual(settings.read_text(), 'personal settings')
                self.assertEqual((base / 'skills/example/SKILL.md').read_text(), 'version one')

    def test_existing_skill_aborts_before_any_write(self):
        home = self.root / 'home'
        target = home / '.dsh/skills/example'
        target.mkdir(parents=True)
        (target / 'SKILL.md').write_text('user skill')
        with self.assertRaises(RuntimeError):
            install_skills(self.source, 'deepseek', home)
        self.assertEqual((target / 'SKILL.md').read_text(), 'user skill')
        self.assertFalse((home / '.dsh/pstack-resources').exists())

    def test_modified_owned_skill_preserved(self):
        home = self.root / 'home'
        install_skills(self.source, 'deepseek', home)
        target = home / '.dsh/skills/example/SKILL.md'
        target.write_text('local edits')
        with self.assertRaises(RuntimeError):
            install_skills(self.source, 'deepseek', home)
        self.assertEqual(target.read_text(), 'local edits')

    def test_update_replaces_unchanged_owned_skill(self):
        home = self.root / 'home'
        install_skills(self.source, 'deepseek', home)
        (self.source / 'skills/example/SKILL.md').write_text('version two')
        install_skills(self.source, 'deepseek', home)
        self.assertEqual((home / '.dsh/skills/example/SKILL.md').read_text(), 'version two')

    def test_failed_update_rolls_back(self):
        home = self.root / 'home'
        install_skills(self.source, 'deepseek', home)
        (self.source / 'skills/example/SKILL.md').write_text('version two')
        with patch('install.os.replace', side_effect=OSError('simulated failure')):
            with self.assertRaises(OSError):
                install_skills(self.source, 'deepseek', home)
        self.assertEqual((home / '.dsh/skills/example/SKILL.md').read_text(), 'version one')

    def test_custom_dsh_home_preserves_settings(self):
        home = self.root / 'home'
        profile = self.root / 'custom-profile'
        with patch.dict('os.environ', {'DSH_HOME': str(profile)}):
            install_skills(self.source, 'deepseek', home)
        self.assertTrue((profile / 'skills/example/SKILL.md').exists())
        self.assertFalse((home / '.dsh').exists())


if __name__ == '__main__':
    unittest.main()
