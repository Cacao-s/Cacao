"""產生慶祝動畫（Lottie JSON）與音效（WAV），放進 shared/，兩平台共用（D-22）。

用法：python tools/codegen/celebrate_assets.py
動畫：中間一枚金幣彈出放大，周圍 12 顆彩色紙花往外散開後淡出，約 1.5 秒。
音效：三個上行音（C6、E6、G6）的短鈴聲，約 0.6 秒。
"""

import json
import math
import struct
import sys
import wave
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
ANIM = ROOT / "shared" / "animations" / "celebrate.json"
SOUND = ROOT / "shared" / "sounds" / "celebrate.wav"

SIZE = 400
FPS = 60
FRAMES = 90
COLORS = ["#E9B44C", "#2F7D6D", "#FF9EC0", "#6A51F0", "#3DDC97", "#FFD23F"]
EASE_OUT = {"o": {"x": [0.2], "y": [0]}, "i": {"x": [0.3], "y": [1]}}


def rgba(hex_value: str) -> list:
    h = hex_value.lstrip("#")
    return [int(h[i:i + 2], 16) / 255 for i in (0, 2, 4)] + [1]


def static(value):
    return {"a": 0, "k": value}


def keyframes(*frames):
    """frames: (time, value)；最後一格只放 t 與 s"""
    out = []
    for i, (t, v) in enumerate(frames):
        frame = {"t": t, "s": v if isinstance(v, list) else [v]}
        if i < len(frames) - 1:
            frame.update(EASE_OUT)
        out.append(frame)
    return {"a": 1, "k": out}


def circle_layer(index, name, color, size, position, scale, opacity):
    return {
        "ddd": 0, "ind": index, "ty": 4, "nm": name, "sr": 1,
        "ks": {"o": opacity, "r": static(0), "p": position, "a": static([0, 0, 0]), "s": scale},
        "ao": 0, "ip": 0, "op": FRAMES, "st": 0, "bm": 0,
        "shapes": [{
            "ty": "gr", "nm": "shape",
            "it": [
                {"ty": "el", "p": static([0, 0]), "s": static([size, size]), "nm": "ellipse"},
                {"ty": "fl", "c": static(rgba(color)), "o": static(100), "r": 1, "nm": "fill"},
                {"ty": "tr", "p": static([0, 0]), "a": static([0, 0]), "s": static([100, 100]), "r": static(0), "o": static(100), "sk": static(0), "sa": static(0)},
            ],
        }],
    }


def animation() -> dict:
    center = [SIZE / 2, SIZE / 2, 0]
    layers = [circle_layer(
        1, "coin", "#E9B44C", 140, static(center),
        keyframes((0, [0, 0, 100]), (18, [118, 118, 100]), (30, [100, 100, 100])),
        static(100),
    )]
    for i in range(12):
        angle = 2 * math.pi * i / 12
        end = [SIZE / 2 + math.cos(angle) * 170, SIZE / 2 + math.sin(angle) * 170, 0]
        layers.append(circle_layer(
            i + 2, f"confetti{i}", COLORS[i % len(COLORS)], 22 if i % 2 else 16,
            keyframes((6, center), (60, end)),
            static([100, 100, 100]),
            keyframes((6, 100), (50, 100), (FRAMES - 1, 0)),
        ))
    return {"v": "5.7.4", "fr": FPS, "ip": 0, "op": FRAMES, "w": SIZE, "h": SIZE, "nm": "celebrate", "ddd": 0, "assets": [], "layers": layers}


def chime(rate: int = 44100) -> bytes:
    notes = [(1046.5, 0.0), (1318.5, 0.12), (1568.0, 0.24)]
    length = int(rate * 0.65)
    samples = []
    for n in range(length):
        t = n / rate
        value = 0.0
        for freq, start in notes:
            if t >= start:
                local = t - start
                value += math.sin(2 * math.pi * freq * local) * math.exp(-local * 7)
        samples.append(int(max(-1.0, min(1.0, value / 2.2)) * 32767))
    return struct.pack(f"<{len(samples)}h", *samples)


def main() -> int:
    sys.stdout.reconfigure(encoding="utf-8")
    ANIM.parent.mkdir(parents=True, exist_ok=True)
    ANIM.write_text(json.dumps(animation(), ensure_ascii=False, separators=(",", ":")), encoding="utf-8", newline="\n")
    SOUND.parent.mkdir(parents=True, exist_ok=True)
    with wave.open(str(SOUND), "wb") as w:
        w.setnchannels(1)
        w.setsampwidth(2)
        w.setframerate(44100)
        w.writeframes(chime())
    print(f"已產生 {ANIM.relative_to(ROOT)}、{SOUND.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
