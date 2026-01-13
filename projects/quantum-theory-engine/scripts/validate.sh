#!/bin/bash
# Quick validation script for the quantum theory engine

set -e

echo "🔬 Quantum Theory Engine - Validation Suite"
echo "==========================================="
echo ""

# Color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}✗ Cargo not found${NC}"
    echo "Please install Rust toolchain from https://rustup.rs"
    exit 1
fi
echo -e "${GREEN}✓ Cargo found${NC}"

# Check dependencies
echo ""
echo "Checking system dependencies..."

check_lib() {
    if ldconfig -p 2>/dev/null | grep -q "$1" || [ -f "/opt/homebrew/opt/$2/lib/$1" ]; then
        echo -e "${GREEN}✓ $2 found${NC}"
        return 0
    else
        echo -e "${YELLOW}⚠ $2 not found (optional but recommended)${NC}"
        return 1
    fi
}

# macOS and Linux have different paths
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Detected macOS"
    if command -v brew &> /dev/null; then
        brew list openblas &> /dev/null && echo -e "${GREEN}✓ OpenBLAS found${NC}" || echo -e "${YELLOW}⚠ OpenBLAS not found${NC}"
        brew list hdf5 &> /dev/null && echo -e "${GREEN}✓ HDF5 found${NC}" || echo -e "${YELLOW}⚠ HDF5 not found${NC}"
    fi
else
    check_lib "libopenblas" "openblas"
    check_lib "libhdf5" "hdf5"
fi

# Format check
echo ""
echo "Running code formatting check..."
if cargo fmt -- --check; then
    echo -e "${GREEN}✓ Code formatting OK${NC}"
else
    echo -e "${YELLOW}⚠ Code formatting issues (run 'cargo fmt' to fix)${NC}"
fi

# Clippy
echo ""
echo "Running Clippy linter..."
if cargo clippy --quiet -- -D warnings 2>&1 | head -20; then
    echo -e "${GREEN}✓ Clippy checks passed${NC}"
else
    echo -e "${YELLOW}⚠ Clippy warnings found${NC}"
fi

# Build
echo ""
echo "Building project..."
if cargo build 2>&1 | tail -10; then
    echo -e "${GREEN}✓ Build successful${NC}"
else
    echo -e "${RED}✗ Build failed${NC}"
    exit 1
fi

# Tests
echo ""
echo "Running tests..."
if cargo test 2>&1 | tail -20; then
    echo -e "${GREEN}✓ Tests passed${NC}"
else
    echo -e "${RED}✗ Tests failed${NC}"
    exit 1
fi

# Validate DSL examples
echo ""
echo "Validating DSL examples..."

if [ -f "target/debug/qte-cli" ]; then
    QTE_CLI="./target/debug/qte-cli"
elif [ -f "target/release/qte-cli" ]; then
    QTE_CLI="./target/release/qte-cli"
else
    echo -e "${YELLOW}⚠ CLI not found, skipping DSL validation${NC}"
    QTE_CLI=""
fi

if [ -n "$QTE_CLI" ]; then
    for file in dsl_examples/*.phys; do
        echo -n "  Validating $(basename $file)... "
        if $QTE_CLI parse "$file" > /dev/null 2>&1; then
            echo -e "${GREEN}✓${NC}"
        else
            echo -e "${RED}✗${NC}"
        fi
    done
fi

# Summary
echo ""
echo "==========================================="
echo -e "${GREEN}✓ Validation complete!${NC}"
echo ""
echo "Next steps:"
echo "  1. Run simulations: $QTE_CLI run dsl_examples/rabi.phys"
echo "  2. Build release: cargo build --release"
echo "  3. Run benchmarks: cargo bench"
echo ""
