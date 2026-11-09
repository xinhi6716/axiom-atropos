# README workflow artwork

The README workflow diagrams retain Axiom's ivory, midnight and antique-gold
palette. The existing portrait and compact logo have not been replaced.
The diagrams explain the documented workflow; they are not a new capability.

## Edit and regenerate

Edit the localized `COPY` entries in `tools/render_readme_workflow.py` and run:

```sh
python tools/render_readme_workflow.py
python tools/render_readme_workflow.py --check
python -m unittest discover -s tests -p test_readme_workflow_assets.py
```

The generator uses the Python standard library. Keep the English, Traditional
Chinese and, where present, Simplified Chinese entries aligned. Do not edit
an SVG directly: CI compares every checked-in asset to its generated source.

Each language has desktop/mobile and light/dark editions. Desktop cards read
left to right, then top to bottom; mobile uses one vertical sequence. Explicit
numbers retain the order without relying on color or a serpentine path.

## Check the result

The offline tests check generated-file parity, all four picture sources in
each README, accessibility metadata, text contrast, step order, body sizes and
absence of remote resources or active SVG content. Instructional text colors
meet 4.5:1 contrast against both the field and panel in each theme.

Render all variants with a local SVG viewer. Inspect Chinese glyphs, line
lengths, clipping and text collisions. Check the mobile image at 288 px wide
(a 320 px viewport with 16 px margins) and the desktop image at 720–900 px.
For browser QA, also check `picture` selection at 600/601 px in light and dark
mode. Image-rendering QA is not a substitute for that browser selection test.

Text uses system fonts with no network loads or bundled fonts. The README's
localized alt text is the image fallback; SVG title and description provide
context when the diagram is viewed directly. Existing notices are retained in
`assets/ASSET_NOTICES.txt`.
