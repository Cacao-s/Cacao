# 產生 Google Play 商店截圖到 docs/store/screenshots/<Locale>/
# 用法：pwsh tools/scripts/store-screenshots.ps1 [-Locale zh-TW|en-US] [-AndroidTools D:\git\Android]
param(
    [string]$Locale = 'zh-TW',
    [string]$AndroidTools = 'D:\git\Android'
)
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$env:JAVA_HOME = Join-Path $AndroidTools 'jdk17'
$env:Path = (Join-Path $AndroidTools 'sdk\platform-tools') + ';' + $env:Path
$maestro = Join-Path $AndroidTools 'maestro-cli\maestro\bin\maestro.bat'
$out = Join-Path $root "docs\store\screenshots\$Locale"
New-Item -ItemType Directory -Force $out | Out-Null

# 流程裡從 App 設定切語言（清資料會重設 per-app locale，所以不能事先用 adb 設）
$lang = if ($Locale -like 'zh*') { 'zh-Hant' } else { 'en' }

$tmp = Join-Path $AndroidTools 'store-shots'
Remove-Item $tmp -Recurse -Force -ErrorAction SilentlyContinue
& $maestro test (Join-Path $root 'tools\e2e\store\screenshots.yaml') --no-ansi --test-output-dir $tmp -e LANG=$lang
$code = $LASTEXITCODE
# Maestro 2.x 把 takeScreenshot 存在輸出目錄的 screenshots\takeScreenshot\ 底下
Get-ChildItem $tmp -Recurse -Filter '0*.png' | Copy-Item -Destination $out -Force
exit $code
