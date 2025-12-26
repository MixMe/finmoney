#!/bin/bash
# Script to validate the project is ready for release

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
PASSED=0
FAILED=0
WARNINGS=0

# Function to print colored output
print_header() {
    echo -e "\n${BLUE}=== $1 ===${NC}"
}

print_pass() {
    echo -e "${GREEN}✓${NC} $1"
    ((PASSED++))
}

print_fail() {
    echo -e "${RED}✗${NC} $1"
    ((FAILED++))
}

print_warn() {
    echo -e "${YELLOW}⚠${NC} $1"
    ((WARNINGS++))
}

# Check functions
check_cargo_toml() {
    print_header "Checking Cargo.toml"
    
    if grep -q "yourusername" Cargo.toml; then
        print_fail "Repository URL still contains placeholder 'yourusername'"
    else
        print_pass "Repository URL looks correct"
    fi
    
    if grep -q "your.email@example.com" Cargo.toml; then
        print_fail "Author email still contains placeholder"
    else
        print_pass "Author email looks correct"
    fi
    
    if grep -q "Your Name" Cargo.toml; then
        print_fail "Author name still contains placeholder"
    else
        print_pass "Author name looks correct"
    fi
    
    # Check required fields
    local required_fields=("name" "version" "description" "license" "repository" "keywords" "categories")
    for field in "${required_fields[@]}"; do
        if grep -q "^$field = " Cargo.toml; then
            print_pass "$field is present"
        else
            print_fail "$field is missing"
        fi
    done
}

check_files() {
    print_header "Checking Required Files"
    
    local required_files=("README.md" "LICENSE-MIT" "LICENSE-APACHE" "CHANGELOG.md" "CONTRIBUTING.md")
    for file in "${required_files[@]}"; do
        if [ -f "$file" ]; then
            print_pass "$file exists"
        else
            print_fail "$file is missing"
        fi
    done
}

check_code_quality() {
    print_header "Checking Code Quality"
    
    # Format check
    if cargo fmt --check > /dev/null 2>&1; then
        print_pass "Code is properly formatted"
    else
        print_fail "Code needs formatting (run: cargo fmt)"
    fi
    
    # Clippy check
    if cargo clippy --all-targets --all-features -- -D warnings > /dev/null 2>&1; then
        print_pass "No clippy warnings"
    else
        print_fail "Clippy warnings found (run: cargo clippy --all-targets --all-features)"
    fi
    
    # Tests
    if cargo test --all-features > /dev/null 2>&1; then
        print_pass "All tests pass"
    else
        print_fail "Some tests are failing"
    fi
    
    # Documentation
    if cargo doc --no-deps --all-features > /dev/null 2>&1; then
        print_pass "Documentation builds successfully"
    else
        print_fail "Documentation build failed"
    fi
}

check_examples() {
    print_header "Checking Examples"
    
    if cargo run --example basic_usage > /dev/null 2>&1; then
        print_pass "basic_usage example works"
    else
        print_fail "basic_usage example failed"
    fi
    
    if cargo run --example trading_ticks > /dev/null 2>&1; then
        print_pass "trading_ticks example works"
    else
        print_fail "trading_ticks example failed"
    fi
}

check_security() {
    print_header "Checking Security"
    
    if command -v cargo-audit &> /dev/null; then
        if cargo audit > /dev/null 2>&1; then
            print_pass "No security vulnerabilities found"
        else
            print_fail "Security vulnerabilities detected (run: cargo audit)"
        fi
    else
        print_warn "cargo-audit not installed (install with: cargo install cargo-audit)"
    fi
}

check_publish_ready() {
    print_header "Checking Publish Readiness"
    
    if cargo publish --dry-run > /dev/null 2>&1; then
        print_pass "Ready for publishing to crates.io"
    else
        print_fail "Not ready for publishing (run: cargo publish --dry-run for details)"
    fi
}

check_git_status() {
    print_header "Checking Git Status"
    
    if [ -n "$(git status --porcelain)" ]; then
        print_warn "Working directory has uncommitted changes"
    else
        print_pass "Working directory is clean"
    fi
    
    local current_branch=$(git branch --show-current)
    if [ "$current_branch" = "main" ]; then
        print_pass "On main branch"
    else
        print_warn "Not on main branch (current: $current_branch)"
    fi
}

# Run all checks
echo "Validating finmoney project for release..."

check_cargo_toml
check_files
check_code_quality
check_examples
check_security
check_publish_ready
check_git_status

# Summary
print_header "Summary"
echo -e "Passed: ${GREEN}$PASSED${NC}"
echo -e "Failed: ${RED}$FAILED${NC}"
echo -e "Warnings: ${YELLOW}$WARNINGS${NC}"

if [ $FAILED -eq 0 ]; then
    echo -e "\n${GREEN}🎉 Project is ready for release!${NC}"
    exit 0
else
    echo -e "\n${RED}❌ Please fix the issues above before releasing.${NC}"
    exit 1
fi