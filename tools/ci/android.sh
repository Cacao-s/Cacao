#!/usr/bin/env bash
# CI 與本機共用：檢查產生的程式碼是否最新、跑 Android 單元測試、編譯 debug APK。
# 用法：bash tools/ci/android.sh（在 repo 根目錄執行）
set -euo pipefail

cd "$(dirname "$0")/../.."

echo "== codegen 檢查 =="
python3 tools/codegen/tokens.py --check 2>/dev/null || python tools/codegen/tokens.py --check
python3 tools/codegen/i18n.py --check 2>/dev/null || python tools/codegen/i18n.py --check

echo "== Android 單元測試與編譯 =="
cd android
./gradlew --no-daemon --console=plain testDebugUnitTest assembleDebug
