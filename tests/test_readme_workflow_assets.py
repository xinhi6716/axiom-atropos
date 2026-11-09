"""Offline contracts for localized, responsive README workflow illustrations."""
import importlib.util
from pathlib import Path
import unittest
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location('workflow', ROOT / 'tools/render_readme_workflow.py')
WORKFLOW = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(WORKFLOW)
NS = '{http://www.w3.org/2000/svg}'


def contrast(a, b):
    def luminance(value):
        channels = [int(value[i:i+2], 16)/255 for i in (1, 3, 5)]
        linear = [v/12.92 if v <= .04045 else ((v+.055)/1.055)**2.4 for v in channels]
        return sum(v*w for v, w in zip(linear, (.2126, .7152, .0722)))
    light, dark = sorted((luminance(a), luminance(b)), reverse=True)
    return (light+.05)/(dark+.05)


class WorkflowAssetsTest(unittest.TestCase):
    def test_generated_assets_are_current(self):
        assets = list(WORKFLOW.assets())
        self.assertEqual(len(assets), 4*len(WORKFLOW.COPY))
        for path, content in assets:
            with self.subTest(path=path.name):
                self.assertEqual(path.read_text(encoding='utf-8'), content)

    def test_assets_are_accessible_and_self_contained(self):
        forbidden = {'script', 'image', 'foreignObject', 'animate', 'set', 'style'}
        for path, content in WORKFLOW.assets():
            with self.subTest(path=path.name):
                root = ET.fromstring(content)
                self.assertEqual(root.tag, NS+'svg')
                self.assertEqual(root.attrib['role'], 'img')
                self.assertEqual(root.attrib['aria-labelledby'], 'title desc')
                self.assertTrue(root.find(NS+'title').text)
                self.assertTrue(root.find(NS+'desc').text)
                for e in root.iter():
                    self.assertNotIn(e.tag.removeprefix(NS), forbidden)
                    self.assertFalse(any(k.startswith('on') or k.endswith('href') for k in e.attrib))
                self.assertNotIn('url(', content)

    def test_all_text_colors_have_aa_contrast(self):
        for theme, p in WORKFLOW.PALETTES.items():
            for color in ('ink', 'muted', 'gold'):
                for background in ('bg', 'panel'):
                    with self.subTest(theme=theme, color=color, background=background):
                        self.assertGreaterEqual(contrast(p[color], p[background]), 4.5)

    def test_localized_picture_sources_resolve(self):
        from html.parser import HTMLParser
        class Pictures(HTMLParser):
            def __init__(self):
                super().__init__()
                self.sources = []
            def handle_starttag(self, tag, attrs):
                attrs = dict(attrs)
                source = attrs.get('srcset') or attrs.get('src', '')
                if source.startswith('assets/'+WORKFLOW.PRODUCT+'-process'):
                    self.sources.append((tag, attrs, source))
        for language in WORKFLOW.COPY:
            name = 'README.md' if language == 'en' else f'README.{language}.md'
            parser = Pictures()
            parser.feed((ROOT / name).read_text(encoding='utf-8'))
            sources = parser.sources
            self.assertEqual(len(sources), 4)
            expected = [WORKFLOW.filename(language, 'dark', True),
                        WORKFLOW.filename(language, 'light', True),
                        WORKFLOW.filename(language, 'dark', False),
                        WORKFLOW.filename(language, 'light', False)]
            self.assertEqual([Path(s[2]).name for s in sources], expected)
            self.assertEqual(sources[0][1]['media'], '(prefers-color-scheme: dark) and (max-width: 600px)')
            self.assertEqual(sources[1][1]['media'], '(max-width: 600px)')
            self.assertEqual(sources[2][1]['media'], '(prefers-color-scheme: dark)')
            self.assertTrue(sources[3][1]['alt'])
            for _, _, source in sources:
                self.assertTrue((ROOT / source).is_file())

    def test_every_variant_keeps_each_step_in_reading_order(self):
        for language, copy in WORKFLOW.COPY.items():
            for theme in WORKFLOW.PALETTES:
                for mobile in (False, True):
                    with self.subTest(language=language, theme=theme, mobile=mobile):
                        root = ET.fromstring(WORKFLOW.render(language, theme, mobile))
                        steps = [g for g in root.findall(NS+'g') if 'data-step' in g.attrib]
                        self.assertEqual([g.attrib['data-step'] for g in steps],
                                         [str(i+1) for i in range(len(copy['steps']))])
                        positions = []
                        for i, step in enumerate(steps):
                            words = [t.text for t in step.findall(NS+'text')]
                            self.assertEqual(words, [f'{i+1:02}', *copy['steps'][i]])
                            rect = step.find(NS+'rect')
                            positions.append((float(rect.attrib['y']), float(rect.attrib['x'])))
                        self.assertEqual(positions, sorted(positions))
                        if mobile:
                            self.assertEqual(len({p[1] for p in positions}), 1)

    def test_layout_variants_and_minimum_body_size(self):
        for path, content in WORKFLOW.assets():
            root = ET.fromstring(content)
            w, h = float(root.attrib['width']), float(root.attrib['height'])
            self.assertEqual(root.attrib['viewBox'], f'0 0 {int(w)} {int(h)}')
            mobile = '-mobile' in path.name
            self.assertEqual(w, 360 if mobile else 1200)
            self.assertGreater(h, 0)
            for g in root.findall(NS+'g'):
                if 'data-step' in g.attrib:
                    body = g.findall(NS+'text')[-2:]
                    for t in body:
                        self.assertGreaterEqual(float(t.attrib['font-size']), 20 if mobile else 24)


if __name__ == '__main__':
    unittest.main()
