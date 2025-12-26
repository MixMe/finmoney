@echo off
REM Script to prepare a new release on Windows

setlocal enabledelayedexpansion

if "%1"=="" (
    echo [ERROR] Please provide a version number (e.g., 0.1.0)
    exit /b 1
)

set VERSION=%1

echo [INFO] Preparing release v%VERSION%

REM Check if working directory is clean
git status --porcelain > temp.txt
set /p STATUS=<temp.txt
del temp.txt

if not "%STATUS%"=="" (
    echo [ERROR] Working directory is not clean. Please commit or stash changes.
    exit /b 1
)

REM Update version in Cargo.toml
echo [INFO] Updating version in Cargo.toml
powershell -Command "(Get-Content Cargo.toml) -replace '^version = \".*\"', 'version = \"%VERSION%\"' | Set-Content Cargo.toml"

REM Run tests
echo [INFO] Running tests
cargo test --all-features
if errorlevel 1 (
    echo [ERROR] Tests failed
    exit /b 1
)

REM Run clippy
echo [INFO] Running clippy
cargo clippy --all-targets --all-features -- -D warnings
if errorlevel 1 (
    echo [ERROR] Clippy failed
    exit /b 1
)

REM Format code
echo [INFO] Formatting code
cargo fmt

REM Build documentation
echo [INFO] Building documentation
cargo doc --no-deps --all-features
if errorlevel 1 (
    echo [ERROR] Documentation build failed
    exit /b 1
)

REM Run examples
echo [INFO] Testing examples
cargo run --example basic_usage > nul
if errorlevel 1 (
    echo [ERROR] Basic usage example failed
    exit /b 1
)

cargo run --example trading_ticks > nul
if errorlevel 1 (
    echo [ERROR] Trading ticks example failed
    exit /b 1
)

REM Dry run publish
echo [INFO] Running publish dry run
cargo publish --dry-run
if errorlevel 1 (
    echo [ERROR] Publish dry run failed
    exit /b 1
)

echo [INFO] Release preparation complete!
echo [INFO] Next steps:
echo 1. Review the changes
echo 2. Update CHANGELOG.md
echo 3. Commit changes: git add . ^&^& git commit -m "Prepare release v%VERSION%"
echo 4. Create tag: git tag -a v%VERSION% -m "Release version %VERSION%"
echo 5. Push: git push origin main ^&^& git push origin v%VERSION%
echo 6. Publish: cargo publish