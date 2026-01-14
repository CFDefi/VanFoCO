#!/bin/bash
#
# Quantum Theory Engine - Web UI Installer
# Professional desktop application in your browser
#

set -e

GREEN='\033[1;32m'
CYAN='\033[1;36m'
YELLOW='\033[1;33m'
RESET='\033[0m'

echo -e "${CYAN}"
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║                                                              ║"
echo "║   QUANTUM THEORY ENGINE - WEB APPLICATION INSTALLER         ║"
echo "║   Professional 2000s Desktop UI in Your Browser             ║"
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

# Create installation directory
INSTALL_DIR="$HOME/.quantum-theory-engine"
echo -e "\n${YELLOW}>>> Creating installation directory...${RESET}"
mkdir -p "$INSTALL_DIR"
echo -e "${GREEN}✓ Directory: ${INSTALL_DIR}${RESET}"

# Download or copy the web UI
echo -e "\n${YELLOW}>>> Installing Web UI Application...${RESET}"
if [ -f "web_ui.py" ]; then
    cp web_ui.py "$INSTALL_DIR/web_ui.py"
    echo -e "${GREEN}✓ Copied from local repository${RESET}"
else
    echo -e "${CYAN}Downloading from GitHub...${RESET}"
    curl -sL "https://raw.githubusercontent.com/CFDefi/VanFoCO/main/projects/quantum-theory-engine/web_ui.py" \
         -o "$INSTALL_DIR/web_ui.py"
    echo -e "${GREEN}✓ Downloaded successfully${RESET}"
fi

chmod +x "$INSTALL_DIR/web_ui.py"

# Create launcher script
echo -e "\n${YELLOW}>>> Creating launcher...${RESET}"
cat > "$INSTALL_DIR/launch-web" << 'EOF'
#!/bin/bash
cd "$HOME/.quantum-theory-engine"
python3 web_ui.py
EOF

chmod +x "$INSTALL_DIR/launch-web"

# Create command-line shortcut
USER_BIN="$HOME/.local/bin"
mkdir -p "$USER_BIN"
ln -sf "$INSTALL_DIR/launch-web" "$USER_BIN/qte-web"
echo -e "${GREEN}✓ Command-line launcher created: qte-web${RESET}"

# Check PATH
if [[ ":$PATH:" != *":$USER_BIN:"* ]]; then
    echo -e "${YELLOW}⚠ Add to your PATH by adding this to ~/.zshrc:${RESET}"
    echo -e "${CYAN}  export PATH=\"\$HOME/.local/bin:\$PATH\"${RESET}"
fi

# Create desktop shortcut for macOS
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo -e "\n${YELLOW}>>> Creating macOS Application...${RESET}"
    APP_DIR="$HOME/Applications/QTE Web.app"
    mkdir -p "$APP_DIR/Contents/MacOS"
    
    cat > "$APP_DIR/Contents/MacOS/QTE Web" << 'LAUNCHER'
#!/bin/bash
cd "$HOME/.quantum-theory-engine"
exec python3 web_ui.py
LAUNCHER
    
    chmod +x "$APP_DIR/Contents/MacOS/QTE Web"
    
    cat > "$APP_DIR/Contents/Info.plist" << 'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>QTE Web</string>
    <key>CFBundleName</key>
    <string>Quantum Theory Engine</string>
    <key>CFBundleDisplayName</key>
    <string>Quantum Theory Engine (Web)</string>
    <key>CFBundleIdentifier</key>
    <string>com.github.cfdefi.qte-web</string>
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

echo -e "${CYAN}Launch options:${RESET}\n"

if [[ "$OSTYPE" == "darwin"* ]]; then
    echo -e "  1. ${YELLOW}Double-click 'QTE Web' in Applications folder${RESET}"
    echo -e "  2. ${YELLOW}Run: open ~/Applications/QTE\\ Web.app${RESET}"
fi

echo -e "  3. ${YELLOW}Run: qte-web${RESET} (from terminal)"
echo -e "  4. ${YELLOW}Run: python3 ${INSTALL_DIR}/web_ui.py${RESET}"

echo -e "\n${CYAN}The application will open at: ${YELLOW}http://localhost:8080${RESET}"
echo -e "\n${GREEN}Enjoy the professional 2000s desktop experience! 🖥${RESET}\n"
