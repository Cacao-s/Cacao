param(
    [switch]$VerboseMode
)

$ErrorActionPreference = 'Stop'

function Ensure-TaskExists {
    if (-not (Get-Command task -ErrorAction SilentlyContinue)) {
        Write-Error "找不到 'task' 指令。請先安裝 go-task (https://taskfile.dev) 並確保其位於 PATH。"
        exit 1
    }
}

Ensure-TaskExists

Write-Host "[check-readiness] Running task health..."
if ($VerboseMode) {
    task --verbose health
} else {
    task health
}

if ($LASTEXITCODE -ne 0) {
    Write-Error "Readiness checks failed (exit code $LASTEXITCODE)."
    exit $LASTEXITCODE
}

Write-Host "[check-readiness] All readiness checks passed."