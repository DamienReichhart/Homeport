# Fonts

The two typefaces of `docs/BRAND.md`, as TrueType: cosmic-text, which draws the iced app's text, does not read the `woff2` files the React app ships. Both are under the SIL Open Font License 1.1; the licence texts are beside the files.

| File                                             | From                                                                   |
| ------------------------------------------------ | ---------------------------------------------------------------------- |
| `HostGrotesk-{Regular,Medium,SemiBold,Bold}.ttf` | Static instances (400, 500, 600, 700) of `HostGrotesk[wght].ttf` 1.003 |
| `FragmentMono-{Regular,Italic}.ttf`              | Fragment Mono 1.011, unchanged                                         |

The sources are the files of [google/fonts](https://github.com/google/fonts) (`ofl/hostgrotesk`, `ofl/fragmentmono`), the same versions as `@fontsource-variable/host-grotesk` and `@fontsource/fragment-mono`, with every glyph instead of the web's Latin subsets.

Host Grotesk is bundled as four static weights and not as the variable file because cosmic-text 0.15 only takes a family's face as the default when its registered weight is the one asked for, and a variable file registers a single weight (see `app/src/ui/theme/font.rs`). To make them again:

```python
from fontTools.ttLib import TTFont
from fontTools.varLib import instancer

for weight, style in [(400, "Regular"), (500, "Medium"), (600, "SemiBold"), (700, "Bold")]:
    font = instancer.instantiateVariableFont(TTFont("HostGrotesk[wght].ttf"), {"wght": weight})
    # Then name it: family "Host Grotesk" (name 16), subfamily `style` (name 17),
    # and OS/2 usWeightClass = weight, so the four share one family.
    font.save(f"HostGrotesk-{style}.ttf")
```
