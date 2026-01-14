#!/bin/bash
#
# Quantum Theory Engine - Retro UI Installer
# Downloads and sets up the 90s supercomputer interface
#

set -e

GREEN='\033[1;32m'
CYAN='\033[1;36m'
YELLOW='\033[1;33m'
RED='\033[1;31m'
RESET='\033[0m'

echo -e "${CYAN}"
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║                                                              ║"
echo "║   QUANTUM THEORY ENGINE - RETRO UI INSTALLER                ║"
echo "║                                                              ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo -e "${RESET}"

# Check for Python 3
echo -e "${YELLOW}>>> Checking Python 3 installation...${RESET}"
if ! command -v python3 &> /dev/null; then
    echo -e "${RED}ERROR: Python 3 is required but not installed.${RESET}"
    echo "Please install Python 3.6 or higher and try again."
    exit 1
fi

PYTHON_VERSION=$(python3 --version 2>&1 | awk '{print $2}')
echo -e "${GREEN}✓ Found Python ${PYTHON_VERSION}${RESET}"

# Create installation directory
INSTALL_DIR="$HOME/.quantum-theory-engine"
echo -e "\n${YELLOW}>>> Creating installation directory...${RESET}"
mkdir -p "$INSTALL_DIR"
echo -e "${GREEN}✓ Directory: ${INSTALL_DIR}${RESET}"

# Download or copy the retro UI
echo -e "\n${YELLOW}>>> Installing Retro UI...${RESET}"

# If running from repo, copy the file
if [ -f "retro_ui.py" ]; then
    cp retro_ui.py "$INSTALL_DIR/retro_ui.py"
    echo -e "${GREEN}✓ Copied from local repository${RESET}"
else
    # Download from GitHub
    echo -e "${CYAN}Downloading from GitHub...${RESET}"
    curl -sL "https://raw.githubusercontent.com/CFDefi/VanFoCO/main/projects/quantum-theory-engine/retro_ui.py" \
         -o "$INSTALL_DIR/retro_ui.py"
    echo -e "${GREEN}✓ Downloaded successfully${RESET}"
fi

# Make it executable
chmod +x "$INSTALL_DIR/retro_ui.py"

# Create launcher script
echo -e "\n${YELLOW}>>> Creating launcher script...${RESET}"
cat > "$INSTALL_DIR/qte-retro" << 'EOF'
#!/bin/bash
# Quantum Theory Engine - Retro UI Launcher
cd "$HOME/.quantum-theory-engine"
python3 retro_ui.py "$@"
EOF

chmod +x "$INSTALL_DIR/qte-retro"
echo -e "${GREEN}✓ Launcher created${RESET}"

# Create symlink in /usr/local/bin (if we have permission)
echo -e "\n${YELLOW}>>> Setting up command-line access...${RESET}"
if [ -w "/usr/local/bin" ]; then
    ln -sf "$INSTALL_DIR/qte-retro" "/usr/local/bin/qte-retro"
    echo -e "${GREEN}✓ Symlink created in /usr/local/bin${RESET}"
    COMMAND="qte-retro"
else
    # Try to add to user's bin
    USER_BIN="$HOME/.local/bin"
    mkdir -p "$USER_BIN"
    ln -sf "$INSTALL_DIR/qte-retro" "$USER_BIN/qte-retro"
    echo -e "${GREEN}✓ Symlink created in ${USER_BIN}${RESET}"
    
    # Check if it's in PATH
    if [[ ":$PATH:" != *":$USER_BIN:"* ]]; then
        echo -e "${YELLOW}⚠ Add to your PATH by running:${RESET}"
        echo -e "${CYAN}  export PATH=\"\$HOME/.local/bin:\$PATH\"${RESET}"
        echo -e "${CYAN}  # Add the above line to your ~/.zshrc or ~/.bashrc${RESET}"
        COMMAND="$USER_BIN/qte-retro"
    else
        COMMAND="qte-retro"
    fi
fi

# Create desktop shortcut for macOS
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo -e "\n${YELLOW}>>> Creating macOS application...${RESET}"
    APP_DIR="$HOME/Applications/QTE Retro.app"
    mkdir -p "$APP_DIR/Contents/MacOS"
    
    cp "$INSTALL_DIR/qte-retro" "$APP_DIR/Contents/MacOS/QTE Retro"
    
    cat > "$APP_DIR/Contents/Info.plist" << 'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>QTE Retro</string>
    <key>CFBundleName</key>
    <string>QTE Retro</string>
    <key>CFBundleDisplayName</key>
    <string>Quantum Theory Engine</string>
    <key>CFBundleIdentifier</key>
    <string>com.github.cfdefi.qte-retro</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
</dict>
</plist>
PLIST
    
    echo -e "${GREEN}✓ macOS app created in ~/Applications${RESET}"
fi

# Installation complete
echo -e "\n${GREEN}"
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║                                                              ║"
echo "║              INSTALLATION COMPLETE!                          ║"
echo "║                                                              ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo -e "${RESET}"

echo -e "${CYAN}To launch the Retro UI, run:${RESET}"
echo -e "  ${YELLOW}${COMMAND}${RESET}"
echo -e "\n${CYAN}Or directly:${RESET}"
echo -e "  ${YELLOW}python3 ${INSTALL_DIR}/retro_ui.py${RESET}"

if [[ "$OSTYPE" == "darwin"* ]]; then
    echo -e "\n${CYAN}Or open from Applications folder:${RESET}"
    echo -e "  ${YELLOW}QTE Retro${RESET}"
fi

echo -e "\n${GREEN}Enjoy the 90s supercomputer experience! 🚀${RESET}\n"
