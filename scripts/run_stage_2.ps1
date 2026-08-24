# Copyright (c) 2026 Matjaz Domen Pecan
# Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
# Licensed under the MIT License
# SPDX-License-Identifier: MIT

$ErrorActionPreference = "Stop"
Push-Location (Split-Path $PSScriptRoot -Parent)

function Invoke-Stern4RustGate {
    param(
        [string]$Label,
        [string[]]$Packages,
        [string[]]$SkippedRules = @()
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
    foreach ($rule in $SkippedRules) {
        $args += @("--skip", $rule)
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

# validation is gated too, at twenty rules instead of twenty-one.
#
# paired-test-file is the one that cannot hold there, and not because the tests
# are exempt: validation has no src/ at all, so no test file in it can be named
# after a source file. Every other rule applies -- the header, AAA structure,
# import naming, ordering, test naming -- and they hold.
#
# It is skipped by name on the command line rather than turned off in
# stern4rust.toml, so a hand-run of `cargo stern4rust` against core still
# applies all twenty-one, and the report says which rule was not applied.
Invoke-Stern4RustGate "House rules validation" @("validation") -SkippedRules @("paired-test-file")

# ---------------------------------------------------------------------------
# CRAP gate
# ---------------------------------------------------------------------------

# core only, and this one genuinely cannot cover validation. CRAP scores source
# functions against their coverage, and validation has no source -- only tests.
# Running it there also fails outright: it drives coverage with `-p validation`,
# which does not build core's binary, so the 53 tests that spawn it cannot find
# it.
Invoke-Crap4RustGate "CRAP dry4rust" @("cargo-dry4rust")

# ---------------------------------------------------------------------------
# Mirrored test gate
#
# Every source file has a test file beside it. No exemptions.
# ---------------------------------------------------------------------------

Invoke-Twin4RustGate "Mirrored tests dry4rust" @("cargo-dry4rust")
Invoke-Twin4RustGate "Mirrored tests validation" @("validation")

# ---------------------------------------------------------------------------
# File-risk gate
#
# 10 is a real bound with headroom, not a ratchet: the worst file here is
# src/rust/parser.rs at 9.55, and nothing else is close.
# ---------------------------------------------------------------------------

Invoke-Iceberg4RustGate "File risk dry4rust" @("cargo-dry4rust") -Threshold "10"
Invoke-Iceberg4RustGate "File risk validation" @("validation") -Threshold "10"

# ---------------------------------------------------------------------------

Write-Host "`ndry4rust Stage 2 passed!" -ForegroundColor Green
Pop-Location
exit 0
