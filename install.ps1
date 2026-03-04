# Copyright 2026 Fitrian Musya
# SPDX-License-Identifier: MIT
#
# Windows Install Script for easydocker
# Usage:
#   .\install.ps1                         # Build and install to default path
#   .\install.ps1 -InstallPath "C:\tools" # Install to custom path
#   .\install.ps1 -BuildOnly              # Build only, don't install

param(
    [string]$InstallPath = "$env:USERPROFILE\.local\bin",
    [switch]$BuildOnly
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Write-Step($msg) {
    Write-Host "`n==> $msg" -ForegroundColor Cyan
}

function Write-Success($msg) {
    Write-Host "[OK] $msg" -ForegroundColor Green
}

function Write-Fail($msg) {
    Write-Host "[FAIL] $msg" -ForegroundColor Red
}

Write-Step "Checking prerequisites"

foreach ($tool in @("cargo", "go")) {
    if (-not (Get-Command $tool -ErrorAction SilentlyContinue)) {
        Write-Fail "$tool is not installed or not in PATH"
        exit 1
    }
    Write-Success "$tool found"
}

Write-Step "Building easydocker (Rust)"

cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Fail "cargo build failed"
    exit 1
}
Write-Success "Built target\release\easydocker.exe"

Write-Step "Building easydocker-runner (Go)"

$runnerDir = Join-Path $PSScriptRoot "runner"
$binDir    = Join-Path $PSScriptRoot "bin"

if (-not (Test-Path $binDir)) {
    New-Item -ItemType Directory -Path $binDir | Out-Null
}

Push-Location $runnerDir
try {
    go build -o (Join-Path $binDir "easydocker-runner.exe") .
    if ($LASTEXITCODE -ne 0) {
        Write-Fail "go build failed"
        exit 1
    }
} finally {
    Pop-Location
}
Write-Success "Built bin\easydocker-runner.exe"

if ($BuildOnly) {
    Write-Host "`nBuild complete." -ForegroundColor Green
    Write-Host "  Rust binary : target\release\easydocker.exe"
    Write-Host "  Go runner   : bin\easydocker-runner.exe"
    exit 0
}

Write-Step "Installing to $InstallPath"

if (-not (Test-Path $InstallPath)) {
    New-Item -ItemType Directory -Path $InstallPath | Out-Null
}

Copy-Item (Join-Path $PSScriptRoot "target\release\easydocker.exe")      "$InstallPath\easydocker.exe"      -Force
Copy-Item (Join-Path $PSScriptRoot "bin\easydocker-runner.exe")           "$InstallPath\easydocker-runner.exe" -Force
Write-Success "Copied binaries to $InstallPath"

$userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($userPath -notlike "*$InstallPath*") {
    [Environment]::SetEnvironmentVariable("PATH", "$userPath;$InstallPath", "User")
    Write-Success "Added $InstallPath to user PATH (restart your terminal to apply)"
} else {
    Write-Success "$InstallPath is already in PATH"
}

Write-Host "`nInstallation complete! Run 'easydocker' to start." -ForegroundColor Green
