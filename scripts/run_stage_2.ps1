# Copyright (c) 2026 Matjaz Domen Pecan
# Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
# Licensed under the MIT License
# SPDX-License-Identifier: MIT

$ErrorActionPreference = "Stop"
Push-Location (Split-Path $PSScriptRoot -Parent)

function Invoke-Stern4RustGate {
    param(
        [string]$Label,
        [string[]]$Packages
    )

    Write-Host "$Label..." -ForegroundColor Cyan

    if (-not (Get-Command cargo-stern4rust -ErrorAction SilentlyContinue)) {
        Write-Host "cargo-stern4rust is not installed." -ForegroundColor Red
        Write-Host "Install it with: cargo install cargo-stern4rust" -ForegroundColor Red
        Pop-Location
        exit 1
    }

    $manifestPath = (Resolve-Path (Join-Path $PSScriptRoot "..\Cargo.toml")).Path
    $args = @("stern4rust", "--manifest-path", $manifestPath)
    foreach ($package in $Packages) {
        $args += @("--package", $package)
    }

    $previousErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    $output = & cargo @args 2>&1
    $ErrorActionPreference = $previousErrorActionPreference
    $exitCode = $LASTEXITCODE
    $output | ForEach-Object { Write-Host $_ }

    # 2 is a rule broken; 1 is the tool failing to run at all. Kept apart so a
    # bad manifest cannot read as a clean codebase.
    if ($exitCode -eq 2) {
        Write-Host "`nFailed: $Label (a house coding rule was broken)" -ForegroundColor Red
        Pop-Location
        exit 1
    }
    if ($exitCode -ne 0) {
        Write-Host "`nFailed: $Label (could not run, exit code $exitCode)" -ForegroundColor Red
        Pop-Location
        exit 1
    }
}

function Invoke-Crap4RustGate {
    param(
        [string]$Label,
        [string[]]$Packages,
        [string]$Features = "",
        [switch]$NoDefaultFeatures,
        [switch]$IncludeTestTargets,
        [double]$Threshold = 15,
        [switch]$UseProjectThreshold,
        [string[]]$ExcludePaths = @()
    )

    Write-Host "$Label..." -ForegroundColor Cyan

    $manifestPath = (Resolve-Path (Join-Path $PSScriptRoot "..\Cargo.toml")).Path
    $args = @("--manifest-path", $manifestPath)
    foreach ($package in $Packages) {
        $args += @("--package", $package)
    }
    if ($Features -ne "") {
        $args += @("--features", $Features)
    }
    if ($NoDefaultFeatures) {
        $args += "--no-default-features"
    }
    if ($IncludeTestTargets) {
        $args += "--include-test-targets"
    }
    foreach ($excludePath in $ExcludePaths) {
        $args += @("--exclude-path", $excludePath)
    }
    $args += @("--warn-only", "--threshold", $Threshold.ToString())

    $previousErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    $output = & cargo crap4rust @args 2>&1
    $ErrorActionPreference = $previousErrorActionPreference
    $exitCode = $LASTEXITCODE
    $output | ForEach-Object { Write-Host $_ }

    if ($exitCode -ne 0) {
        Write-Host "`nFailed: $Label (exit code $exitCode)" -ForegroundColor Red
        Pop-Location
        exit 1
    }

    $summaryLine = $output | Select-String -Pattern "summary:\s+total_functions=.*crappy_functions=(\d+)"
    if (-not $summaryLine) {
        Write-Host "`nFailed: $Label (could not parse crap4rust summary)" -ForegroundColor Red
        Pop-Location
        exit 1
    }

    $crappyCount = [int]$summaryLine.Matches[0].Groups[1].Value

    if ($UseProjectThreshold) {
        $verdictLine = $output | Select-String -Pattern "verdict=(clean|warn|crappy)"
        if (-not $verdictLine) {
            Write-Host "`nFailed: $Label (could not parse crap4rust verdict)" -ForegroundColor Red
            Pop-Location
            exit 1
        }
        $verdict = $verdictLine.Matches[0].Groups[1].Value
        if ($verdict -eq "crappy") {
            Write-Host "`nFailed: $Label (project verdict is crappy)" -ForegroundColor Red
            Pop-Location
            exit 1
        }
    } else {
        if ($crappyCount -gt 0) {
            Write-Host "`nFailed: $Label ($crappyCount crappy functions detected)" -ForegroundColor Red
            Pop-Location
            exit 1
        }
    }
}

function Invoke-Twin4RustGate {
    param(
        [string]$Label,
        [string[]]$Packages
    )

    Write-Host "$Label..." -ForegroundColor Cyan

    if (-not (Get-Command cargo-twin4rust -ErrorAction SilentlyContinue)) {
        Write-Host "`ncargo-twin4rust is not installed." -ForegroundColor Red
        Write-Host "Install it with: cargo install cargo-twin4rust" -ForegroundColor Red
        Pop-Location
        exit 1
    }

    $manifestPath = (Resolve-Path (Join-Path $PSScriptRoot "..\Cargo.toml")).Path

    $args = @("twin4rust", "--manifest-path", $manifestPath)
    foreach ($package in $Packages) {
        $args += @("--package", $package)
    }

    $previousErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    $output = & cargo @args 2>&1
    $ErrorActionPreference = $previousErrorActionPreference
    $exitCode = $LASTEXITCODE
    $output | ForEach-Object { Write-Host $_ }

    if ($exitCode -ne 0) {
        Write-Host "`nFailed: $Label (source files without a mirrored test)" -ForegroundColor Red
        Pop-Location
        exit 1
    }
}

# ---------------------------------------------------------------------------
# CRAP gate
# ---------------------------------------------------------------------------

function Invoke-Iceberg4RustGate {
    param(
        [string]$Label,
        [string[]]$Packages,
        [string]$Threshold
    )

    Write-Host "$Label..." -ForegroundColor Cyan

    if (-not (Get-Command cargo-iceberg4rust -ErrorAction SilentlyContinue)) {
        Write-Host "`ncargo-iceberg4rust is not installed." -ForegroundColor Red
        Write-Host "Install it with: cargo install cargo-iceberg4rust" -ForegroundColor Red
        Pop-Location
        exit 1
    }

    $manifestPath = (Resolve-Path (Join-Path $PSScriptRoot "..\Cargo.toml")).Path

    # The ceiling is passed as a string rather than a [double] so it reaches the
    # CLI unchanged. Interpolating a [double] formats it with the current culture,
    # which emits a comma decimal separator on some locales and fails to parse.
    $args = @("iceberg4rust", "--manifest-path", $manifestPath, "--threshold", $Threshold)
    foreach ($package in $Packages) {
        $args += @("--package", $package)
    }

    $previousErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    $output = & cargo @args 2>&1
    $ErrorActionPreference = $previousErrorActionPreference
    $exitCode = $LASTEXITCODE
    $output | ForEach-Object { Write-Host $_ }

    # 2 is the tool's own "offenders found"; anything else non-zero means it
    # could not run at all.
    if ($exitCode -eq 2) {
        Write-Host "`nFailed: $Label (file at or above the ceiling of $Threshold)" -ForegroundColor Red
        Pop-Location
        exit 1
    }
    if ($exitCode -ne 0) {
        Write-Host "`nFailed: $Label (exit code $exitCode)" -ForegroundColor Red
        Pop-Location
        exit 1
    }
}
# ---------------------------------------------------------------------------
# House rules
#
# First, because its corrections are renames, file moves and directory splits:
# a layout it is about to reject is one the other three would have measured for
# nothing. Its findings are also the cheapest to act on.
#
# Twenty-one rules apply, none skipped, nothing baselined, zero offences. That
# is the state this gate exists to hold.
# ---------------------------------------------------------------------------

Invoke-Stern4RustGate "House rules dry4rust" @("cargo-dry4rust")

# ---------------------------------------------------------------------------
# CRAP gate
#
# 15, the same number every repository in the family uses. What differs here is
# -UseProjectThreshold, and the reason is arithmetic rather than taste.
#
# CRAP is complexity^2 * (1 - coverage)^3 + complexity, so it is never smaller
# than the complexity itself. A function above 15 complexity therefore cannot
# reach 15 at any coverage, including 100%. Three inherited functions are in
# that position or next to it:
#
#   grouper::find_near_duplicates   complexity 27, 82% -> 31.3   unreachable
#   extractor::extract_recursive    complexity 20, 92% -> 20.2   unreachable
#   main                            complexity 14, 79% -> 15.7   needs 83%
#
# The first two are upstream's matching and extraction algorithms. main is
# glue, but coverage is collected from the test harness and not from spawned
# children, so its dispatch arms cannot be reached by driving the binary --
# seven subprocess tests moved it by nothing.
#
# Raising the number to 32 would have hidden all three and let a newly written
# function at complexity 30 through in silence. Keeping 15 with the project
# threshold keeps every function measured at the family's line and still named
# in the report; what it spends is a budget, currently 2.4% against 5%. The
# gate fails when that budget is exceeded, so new debt has nowhere to hide.
#
# Getting to 15 with zero tolerance means decomposing those functions. That is
# a change to productive code, and it is proposed rather than assumed.
# ---------------------------------------------------------------------------

Invoke-Crap4RustGate "CRAP dry4rust" @("cargo-dry4rust") -Threshold 15 -UseProjectThreshold

# ---------------------------------------------------------------------------
# Mirrored test gate
#
# Every source file has a test file beside it. No exemptions.
# ---------------------------------------------------------------------------

Invoke-Twin4RustGate "Mirrored tests dry4rust" @("cargo-dry4rust")

# ---------------------------------------------------------------------------
# File-risk gate
#
# 10 is a real bound with headroom, not a ratchet: the worst file here is
# src/rust/parser.rs at 9.55, and nothing else is close.
# ---------------------------------------------------------------------------

Invoke-Iceberg4RustGate "File risk dry4rust" @("cargo-dry4rust") -Threshold "10"

# ---------------------------------------------------------------------------

Write-Host "`ndry4rust Stage 2 passed!" -ForegroundColor Green
Pop-Location
exit 0
