"""shared/i18n/strings.json → Android strings.xml（Phase 2 再加 iOS String Catalog）。

預設語言（values/）是英文；繁中放 values-b+zh+Hant/，台灣、香港的繁中系統都會用到。
用法：python tools/codegen/i18n.py          產生檔案
      python tools/codegen/i18n.py --check  只檢查產生的檔案是否最新（CI 用）
"""

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "shared" / "i18n" / "strings.json"
RES = ROOT / "android" / "app" / "src" / "main" / "res"
OUTPUTS = {
    "en": RES / "values" / "strings.xml",
    "zh-Hant": RES / "values-b+zh+Hant" / "strings.xml",
}
KEY = re.compile(r"^[a-z][a-z0-9_]*$")
PLACEHOLDER = re.compile(r"%\d+\$[sd]")


def escape(text: str) -> str:
    text = text.replace("\\", "\\\\").replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")
    text = text.replace("'", "\\'").replace('"', '\\"').replace("\n", "\\n")
    if text.startswith("@") or text.startswith("?"):
        text = "\\" + text
    return text


def validate(strings: dict) -> None:
    for key, values in strings.items():
        if not KEY.match(key):
            raise ValueError(f"key 只能用小寫英數與底線：{key}")
        missing = [lang for lang in OUTPUTS if not values.get(lang)]
        if missing:
            raise ValueError(f"{key} 缺少翻譯：{missing}")
        placeholders = {lang: sorted(PLACEHOLDER.findall(values[lang])) for lang in OUTPUTS}
        if len({tuple(p) for p in placeholders.values()}) > 1:
            raise ValueError(f"{key} 各語言的 %1$s 這類參數不一致：{placeholders}")


def xml(strings: dict, lang: str) -> str:
    lines = [
        '<?xml version="1.0" encoding="utf-8"?>',
        "<!-- 自動產生，請勿手改。來源：shared/i18n/strings.json；產生：python tools/codegen/i18n.py -->",
        "<resources>",
    ]
    for key in sorted(strings):
        lines.append(f'    <string name="{key}">{escape(strings[key][lang])}</string>')
    lines.append("</resources>")
    return "\n".join(lines) + "\n"


def main() -> int:
    sys.stdout.reconfigure(encoding="utf-8")
    strings = json.loads(SOURCE.read_text(encoding="utf-8"))["strings"]
    validate(strings)
    outputs = {path: xml(strings, lang) for lang, path in OUTPUTS.items()}
    if "--check" in sys.argv:
        stale = [p for p, text in outputs.items() if not p.exists() or p.read_text(encoding="utf-8") != text]
        if stale:
            for p in stale:
                print(f"{p.relative_to(ROOT)} 不是最新，請執行 python tools/codegen/i18n.py")
            return 1
        print(f"i18n：產生的檔案是最新的（{len(strings)} 個字串）")
        return 0
    for path, text in outputs.items():
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8", newline="\n")
        print(f"已產生 {path.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
