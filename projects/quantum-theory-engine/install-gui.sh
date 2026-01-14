#!/bin/bash
#
# Quantum Theory Engine - GUI Installer
# Creates a standalone macOS application
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
echo "║   QUANTUM THEORY ENGINE - GUI APP INSTALLER                 ║"
echo "║   Classic 2000s Desktop Application                          ║"
echo "║                                                              ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo -e "${RESET}"

# Check for Python 3
echo -e "${YELLOW}>>> Checking Python installation...${RESET}"
if ! command -v python3 &> /dev/null; then
    echo -e "${RED}ERROR: Python 3 is required but not installed.${RESET}"
    exit 1
fi

PYTHON_VERSION=$(python3 --version 2>&1 | awk '{print $2}')
echo -e "${GREEN}✓ Found Python ${PYTHON_VERSION}${RESET}"

# Check for tkinter
echo -e "${YELLOW}>>> Checking tkinter (GUI library)...${RESET}"
if python3 -c "import tkinter" 2>/dev/null; then
    echo -e "${GREEN}✓ tkinter is available${RESET}"
else
    echo -e "${RED}ERROR: tkinter is not available.${RESET}"
    echo -e "${YELLOW}On macOS, tkinter should come with Python.${RESET}"
    echo -e "${YELLOW}If missing, try: brew install python-tk@3.9${RESET}"
    exit 1
fi

# Create installation directory
INSTALL_DIR="$HOME/.quantum-theory-engine"
echo -e "\n${YELLOW}>>> Creating installation directory...${RESET}"
mkdir -p "$INSTALL_DIR"
echo -e "${GREEN}✓ Directory: ${INSTALL_DIR}${RESET}"

# Copy the GUI app
echo -e "\n${YELLOW}>>> Installing GUI Application...${RESET}"
if [ -f "gui_app.py" ]; then
    cp gui_app.py "$INSTALL_DIR/gui_app.py"
    echo -e "${GREEN}✓ Copied from local repository${RESET}"
else
    echo -e "${CYAN}Downloading from GitHub...${RESET}"
    curl -sL "https://raw.githubusercontent.com/CFDefi/VanFoCO/main/projects/quantum-theory-engine/gui_app.py" \
         -o "$INSTALL_DIR/gui_app.py"
    echo -e "${GREEN}✓ Downloaded successfully${RESET}"
fi

chmod +x "$INSTALL_DIR/gui_app.py"

# Create launcher script  
echo -e "\n${YELLOW}>>> Creating launcher...${RESET}"
cat > "$INSTALL_DIR/launch-gui" << 'EOF'
#!/bin/bash
cd "$HOME/.quantum-theory-engine"
python3 gui_app.py
EOF

chmod +x "$INSTALL_DIR/launch-gui"

# Create macOS application bundle
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo -e "\n${YELLOW}>>> Creating macOS Application Bundle...${RESET}"
    
    APP_NAME="Quantum Theory Engine"
    APP_DIR="$HOME/Applications/${APP_NAME}.app"
    
    # Create app structure
    mkdir -p "$APP_DIR/Contents/MacOS"
    mkdir -p "$APP_DIR/Contents/Resources"
    
    # Copy executable
    cat > "$APP_DIR/Contents/MacOS/${APP_NAME}" << 'LAUNCHER'
#!/bin/bash
cd "$HOME/.quantum-theory-engine"
exec python3 gui_app.py
LAUNCHER
    
    chmod +x "$APP_DIR/Contents/MacOS/${APP_NAME}"
    
    # Create Info.plist
    cat > "$APP_DIR/Contents/Info.plist" << 'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>Quantum Theory Engine</string>
    <key>CFBundleName</key>
    <string>Quantum Theory Engine</string>
    <key>CFBundleDisplayName</key>
    <string>Quantum Theory Engine</string>
    <key>CFBundleIdentifier</key>
    <string>com.github.cfdefi.quantum-theory-engine</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.13</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
PLIST
    
    echo -e "${GREEN}✓ macOS app bundle created${RESET}"
fi

# Create command-line shortcut
USER_BIN="$HOME/.local/bin"
mkdir -p "$USER_BIN"
ln -sf "$INSTALL_DIR/launch-gui" "$USER_BIN/qte-gui"
echo -e "${GREEN}✓ Command-line launcher: qte-gui${RESET}"

# Check PATH
if [[ ":$PATH:" != *":$USER_BIN:"* ]]; then
    echo -e "${YELLOW}⚠ Add to your PATH by adding this to ~/.zshrc:${RESET}"
    echo -e "${CYAN}  export PATH=\"\$HOME/.local/bin:\$PATH\"${RESET}"
fi

# Installation complete
echo -e "\n${GREEN}"
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║                                                              ║"
echo "║              INSTALLATION COMPLETE!                          ║"
echo "║                                                              ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo -e "${RESET}"

echo -e "${CYAN}Launch options:${RESET}\n"

if [[ "$OSTYPE" == "darwin"* ]]; then
    echo -e "  1. ${YELLOW}Double-click 'Quantum Theory Engine' in Applications folder${RESET}"
    echo -e "  2. ${YELLOW}Run: open ~/Applications/Quantum\\ Theory\\ Engine.app${RESET}"
fi

echo -e "  3. ${YELLOW}Run: qte-gui${RESET} (from terminal)"
echo -e "  4. ${YELLOW}Run: python3 ${INSTALL_DIR}/gui_app.py${RESET}"

echo -e "\n${GREEN}Enjoy the classic 2000s GUI experience! 🖥️${RESET}\n"
