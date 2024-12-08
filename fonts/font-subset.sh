# Download fonts from:
# https://github.com/notofonts/noto-cjk/raw/refs/heads/main/Sans/SubsetOTF/SC/NotoSansSC-Regular.otf
# https://github.com/atelier-anchor/smiley-sans/releases/download/v2.0.1/smiley-sans-v2.0.1.zip (containing SmileySans-Oblique.ttf)
# https://github.com/lxgw/LxgwWenkaiGB-Lite/raw/refs/heads/main/fonts/TTF/LXGWWenKaiMonoGBLite-Medium.ttf
# All these fonts are released under SIL Open Font License, Version 1.1.
# Install fonttools (https://fonttools.readthedocs.io) and run the following script to get font subsets.

pyftsubset NotoSansSC-Regular.otf --text="0123456789 挂号喂药打针开刀捂嘴捂肩捂腹全防反弹攻击防御 再来一局"
pyftsubset SmileySans-Oblique.ttf --text="0123456789 挂号喂药打针开刀捂嘴捂肩捂腹全防反弹攻击防御"
pyftsubset LXGWWenKaiMonoGBLite-Medium.ttf --text="0123456789 挂号全防反弹 胜负"
