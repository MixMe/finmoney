@echo off
REM Script to validate the project is ready for release on Windows

setlocal enabledelayedexpansion

set PASSED=0
set FAILED=0
set WARNINGS=0

echo Validating finmoney project for release...

REM Check Cargo.toml
echo.
echo === Checking Cargo.toml ===

findstr /C:"yourusername" Cargo.toml >nul
if !errorlevel! equ 0 (
    echo [FAIL] Repository URL still contains placeholder 'yourusername'
    set /a FAILED+=1
) else (
    echo [PASS] Repository URL looks correct
    set /a PASSED+=1
)

findstr /C:"your.email@example.com" Cargo.toml >nul
if !errorlevel! equ 0 (
    echo [FAIL] Author email still contains placeholder
    set /a FAILED+=1
) else (
    echo [PASS] Author email looks correct
    set /a PASSED+=1
)

findstr /C:"Your Name" Cargo.toml >nul
if !errorlevel! equ 0 (
    echo [FAIL] Author name still contains placeholder
    set /a FAILED+=1
) else (
    echo [PASS] Author name looks correct
    set /a PASSED+=1
)

REM Check required files
echo.
echo === Checking Required Files ===

for %%f in (README.md LICENSE-MIT LICENSE-APACHE CHANGELOG.md CONTRIBUTING.md) do (
    if exist "%%f" (
        echo [PASS] %%f exists
        set /a PASSED+=1
    ) else (
        echo [FAIL] %%f is missing
        set /a FAILED+=1
    )
)

REM Check code quality
echo.
echo === Checking Code Quality ===

cargo fmt --check >nul 2>&1
if !errorlevel! equ 0 (
    echo [PASS] Code is properly formatted
    set /a PASSED+=1
) else (
    echo [FAIL] Code needs formatting (run: cargo fmt)
    set /a FAILED+=1
)

cargo clippy --all-targets --all-features -- -D warnings >nul 2>&1
if !errorlevel! equ 0 (
    echo [PASS] No clippy warnings
    set /a PASSED+=1
) else (
    echo [FAIL] Clippy warnings found
    set /a FAILED+=1
)

cargo test --all-features >nul 2>&1
if !errorlevel! equ 0 (
    echo [PASS] All tests pass
    set /a PASSED+=1
) else (
    echo [FAIL] Some tests are failing
    set /a FAILED+=1
)

cargo doc --no-deps --all-features >nul 2>&1
if !errorlevel! equ 0 (
    echo [PASS] Documentation builds successfully
    set /a PASSED+=1
) else (
    echo [FAIL] Documentation build failed
    set /a FAILED+=1
)

REM Check examples
echo.
echo === Checking Examples ===

cargo run --example basic_usage >nul 2>&1
if !errorlevel! equ 0 (
    echo [PASS] basic_usage example works
    set /a PASSED+=1
) else (
    echo [FAIL] basic_usage example failed
    set /a FAILED+=1
)

cargo run --example trading_ticks >nul 2>&1
if !errorlevel! equ 0 (
    echo [PASS] trading_ticks example works
    set /a PASSED+=1
) else (
    echo [FAIL] trading_ticks example failed
    set /a FAILED+=1
)

REM Check publish readiness
echo.
echo === Checking Publish Readiness ===

cargo publish --dry-run >nul 2>&1
if !errorlevel! equ 0 (
    echo [PASS] Ready for publishing to crates.io
    set /a PASSED+=1
) else (
    echo [FAIL] Not ready for publishing
    set /a FAILED+=1
)

REM Check git status
echo.
echo === Checking Git Status ===

git status --porcelain > temp.txt
set /p STATUS=<temp.txt
del temp.txt

if not "!STATUS!"=="" (
    echo [WARN] Working directory has uncommitted changes
    set /a WARNINGS+=1
) else (
    echo [PASS] Working directory is clean
    set /a PASSED+=1
)

REM Summary
echo.
echo === Summary ===
echo Passed: !PASSED!
echo Failed: !FAILED!
echo Warnings: !WARNINGS!

if !FAILED! equ 0 (
    echo.
    echo Project is ready for release!
    exit /b 0
) else (
    echo.
    echo Please fix the issues above before releasing.
    exit /b 1
)