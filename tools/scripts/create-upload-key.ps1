# 產生 Google Play 上傳金鑰，並寫本機 android/keystore.properties（不進 git）
# 用法：在「看得到的」PowerShell 視窗執行，密碼由使用者自己輸入，不經過 Claude
#   pwsh tools/scripts/create-upload-key.ps1 [-AndroidTools D:\git\Android]
param([string]$AndroidTools = 'D:\git\Android')
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$keyDir = Join-Path $AndroidTools 'keys'
$keyFile = Join-Path $keyDir 'cacao-upload.jks'
$props = Join-Path $root 'android\keystore.properties'
$keytool = Join-Path $AndroidTools 'jdk17\bin\keytool.exe'

if (Test-Path $keyFile) { Write-Host "金鑰已存在：$keyFile，不覆蓋。" -ForegroundColor Yellow; exit 1 }
New-Item -ItemType Directory -Force $keyDir | Out-Null

Write-Host '=== Cacao 上傳金鑰 ===' -ForegroundColor Cyan
Write-Host '請想一組密碼（至少 6 個字），等一下要輸入 3 次。請存進密碼管理器，弄丟就無法更新 App。'
$p1 = Read-Host '密碼' -AsSecureString
$p2 = Read-Host '再輸入一次' -AsSecureString
$plain = [Runtime.InteropServices.Marshal]::PtrToStringBSTR([Runtime.InteropServices.Marshal]::SecureStringToBSTR($p1))
$check = [Runtime.InteropServices.Marshal]::PtrToStringBSTR([Runtime.InteropServices.Marshal]::SecureStringToBSTR($p2))
if ($plain -ne $check) { Write-Host '兩次不一樣，請重新執行。' -ForegroundColor Red; Read-Host '按 Enter 關閉'; exit 1 }
if ($plain.Length -lt 6) { Write-Host '至少 6 個字，請重新執行。' -ForegroundColor Red; Read-Host '按 Enter 關閉'; exit 1 }

$env:CACAO_KS_PASS = $plain
& $keytool -genkeypair -keystore $keyFile -storetype PKCS12 -alias upload -keyalg RSA -keysize 2048 -validity 10000 `
    -dname 'CN=adaruru, O=Cacao, C=TW' -storepass:env CACAO_KS_PASS -keypass:env CACAO_KS_PASS
$code = $LASTEXITCODE
Remove-Item Env:CACAO_KS_PASS
if ($code -ne 0) { Write-Host '產生失敗。' -ForegroundColor Red; Read-Host '按 Enter 關閉'; exit $code }

@"
storeFile=$($keyFile -replace '\\','/')
storePassword=$plain
keyAlias=upload
keyPassword=$plain
"@ | Set-Content -Encoding UTF8 $props

Write-Host "完成：金鑰 $keyFile；設定 $props（已被 git 忽略）" -ForegroundColor Green
Read-Host '按 Enter 關閉'
