"""shared/tokens/themes.json → Android Kotlin（Phase 2 再加 Swift 輸出）。

用法：python tools/codegen/tokens.py          產生檔案
      python tools/codegen/tokens.py --check  只檢查產生的檔案是否最新（CI 用）
"""

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "shared" / "tokens" / "themes.json"
KOTLIN_OUT = ROOT / "android" / "app" / "src" / "main" / "java" / "com" / "ruru" / "cacao" / "ui" / "theme" / "generated" / "ThemeTokens.kt"

COLOR_KEYS = [
    "background", "surface", "onSurface", "muted", "outline",
    "primary", "onPrimary", "secondary", "onSecondary",
    "locked", "onLocked", "danger",
]
FONTS = {"fraunces", "rubik", "nunito", "sora"}


def camel(snake: str) -> str:
    head, *rest = snake.split("_")
    return head + "".join(part.capitalize() for part in rest)


def color(hex_value: str) -> str:
    value = hex_value.lstrip("#").upper()
    if len(value) != 6:
        raise ValueError(f"顏色必須是 #RRGGBB：{hex_value}")
    return f"Color(0xFF{value})"


def validate(themes: list) -> None:
    ids = [t["id"] for t in themes]
    if len(ids) != len(set(ids)):
        raise ValueError("主題 id 重複")
    for theme in themes:
        missing = [k for k in COLOR_KEYS if k not in theme["color"]]
        if missing:
            raise ValueError(f"{theme['id']} 缺少顏色：{missing}")
        if theme["font"]["number"] not in FONTS:
            raise ValueError(f"{theme['id']} 的字型不在打包清單：{theme['font']['number']}")


def kotlin(themes: list) -> str:
    lines = [
        "// 自動產生，請勿手改。來源：shared/tokens/themes.json；產生：python tools/codegen/tokens.py",
        "package com.ruru.cacao.ui.theme.generated",
        "",
        "import androidx.compose.ui.graphics.Color",
        "import androidx.compose.ui.unit.dp",
        "import com.ruru.cacao.R",
        "import com.ruru.cacao.ui.theme.ThemeColors",
        "import com.ruru.cacao.ui.theme.ThemeShape",
        "import com.ruru.cacao.ui.theme.ThemeTokens",
        "",
        "object GeneratedThemes {",
    ]
    for theme in themes:
        c = theme["color"]
        s = theme["shape"]
        lines += [
            f"    val {camel(theme['id'])} = ThemeTokens(",
            f"        id = \"{theme['id']}\",",
            f"        nameZh = \"{theme['name']['zh-Hant']}\",",
            f"        nameEn = \"{theme['name']['en']}\",",
            f"        dark = {str(theme['dark']).lower()},",
            "        colors = ThemeColors(",
            *[f"            {key} = {color(c[key])}," for key in COLOR_KEYS],
            "        ),",
            f"        numberFont = R.font.{theme['font']['number']},",
            f"        titleSerif = {str(theme['font']['titleSerif']).lower()},",
            f"        shape = ThemeShape(radius = {s['radius']}.dp, border = {float(s['border'])}.dp, hardShadow = {s['hardShadow']}.dp),",
            "    )",
            "",
        ]
    lines.append(f"    val all = listOf({', '.join(camel(t['id']) for t in themes)})")
    lines.append("}")
    return "\n".join(lines) + "\n"


def main() -> int:
    sys.stdout.reconfigure(encoding="utf-8")
    themes = json.loads(SOURCE.read_text(encoding="utf-8"))["themes"]
    validate(themes)
    output = kotlin(themes)
    if "--check" in sys.argv:
        current = KOTLIN_OUT.read_text(encoding="utf-8") if KOTLIN_OUT.exists() else ""
        if current != output:
            print(f"{KOTLIN_OUT.relative_to(ROOT)} 不是最新，請執行 python tools/codegen/tokens.py")
            return 1
        print("tokens：產生的檔案是最新的")
        return 0
    KOTLIN_OUT.parent.mkdir(parents=True, exist_ok=True)
    KOTLIN_OUT.write_text(output, encoding="utf-8", newline="\n")
    print(f"已產生 {KOTLIN_OUT.relative_to(ROOT)}（{len(themes)} 個主題）")
    return 0


if __name__ == "__main__":
    sys.exit(main())
