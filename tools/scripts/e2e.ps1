# 跑全部 Maestro 端對端測試（模擬器要先開好、App 已安裝）
# 用法：pwsh tools/scripts/e2e.ps1 [-AndroidTools D:\git\Android] [-Flow m3_goals]
param(
    [string]$AndroidTools = 'D:\git\Android',
    [string]$Flow = ''
)

$ErrorActionPreference = 'Stop'
$root = Resolve-Path (Join-Path $PSScriptRoot '..\..')
$env:JAVA_HOME = Join-Path $AndroidTools 'jdk17'
$env:Path = (Join-Path $AndroidTools 'sdk\platform-tools') + ';' + $env:Path
$maestro = Join-Path $AndroidTools 'maestro-cli\maestro\bin\maestro.bat'

$flows = Get-ChildItem (Join-Path $root 'tools\e2e') -Filter '*.yaml' | Sort-Object Name
if ($Flow) { $flows = $flows | Where-Object { $_.BaseName -eq $Flow } }

# 一次跑一條，避免同時建置造成記憶體不足
& $maestro test @($flows.FullName) --no-ansi
exit $LASTEXITCODE
