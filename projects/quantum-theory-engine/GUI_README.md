# 🖥️ Quantum Theory Engine - Desktop Application

**Classic 2000s-style desktop app with modern quantum computing features**

![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-blue)
![Python](https://img.shields.io/badge/python-3.6+-blue)
![GUI](https://img.shields.io/badge/GUI-tkinter-green)

## ✨ Features

### Classic 2000s Design
- 🎨 **Windows XP-inspired Interface** - Beige/gray color scheme
- 📊 **Professional Menu Bar** - File, Edit, Tools, View, Help
- 🔧 **Toolbar with 3D Buttons** - Quick access to all features
- 📑 **Tabbed Interface** - Multi-document interface (MDI) style
- ⏱️ **Live Status Bar** - System status and clock
- 💬 **Classic Dialogs** - Modal windows and message boxes

### Modern Quantum Features
- ⚛️ **Quantum Simulator** - Interactive state evolution
- 🔬 **Theorem Prover** - Automated symbolic proving
- 📈 **Parameter Fitting** - Maximum likelihood estimation
- 📚 **Template Library** - Pre-configured experiments
- 📊 **System Diagnostics** - Real-time health monitoring
- 📝 **Activity Logging** - Complete audit trail

## 🚀 Quick Install

### Automated Install (Recommended)

```bash
curl -sL https://raw.githubusercontent.com/CFDefi/VanFoCO/main/projects/quantum-theory-engine/install-gui.sh | bash
```

### Manual Install

1. **Download the app:**
   ```bash
   curl -O https://raw.githubusercontent.com/CFDefi/VanFoCO/main/projects/quantum-theory-engine/gui_app.py
   chmod +x gui_app.py
   ```

2. **Run it:**
   ```bash
   python3 gui_app.py
   ```

### From Source

```bash
cd projects/quantum-theory-engine
chmod +x gui_app.py
python3 gui_app.py
```

## 📋 Requirements

- **Python 3.6+** (pre-installed on macOS/Linux)
- **tkinter** (included with Python on most systems)
- **No external dependencies!** - Pure Python standard library

### Verify tkinter installation:
```bash
python3 -c "import tkinter; print('✓ tkinter available')"
```

If missing:
- **macOS:** `brew install python-tk`
- **Ubuntu/Debian:** `sudo apt-get install python3-tk`
- **Fedora:** `sudo dnf install python3-tkinter`

## 🎯 Usage

### Launch Methods

**1. Desktop Application (macOS):**
- Double-click "Quantum Theory Engine" in Applications folder
- Or run: `open ~/Applications/Quantum\ Theory\ Engine.app`

**2. Command Line:**
```bash
qte-gui
```

**3. Direct Python:**
```bash
python3 gui_app.py
```

## 📱 Application Features

### Main Menu

| Menu | Options |
|------|---------|
| **File** | New Simulation, Open, Save, Exit |
| **Edit** | Copy, Preferences |
| **Tools** | Quantum Simulator, Theorem Prover, Parameter Fitting, Diagnostics |
| **View** | Templates, Activity Log, Status Bar |
| **Help** | User Guide, Tutorials, About |

### Toolbar Buttons

| Icon | Function | Description |
|------|----------|-------------|
| 🆕 New | New Simulation | Start a new quantum simulation |
| 📂 Open | Open File | Load saved simulation |
| 💾 Save | Save Results | Export simulation data |
| ▶️ Simulate | Run Simulator | Execute quantum evolution |
| 🔬 Prove | Theorem Prover | Symbolic verification |
| 📊 Fit | Parameter Fit | MLE parameter estimation |
| 📚 Templates | Template Library | Browse experiments |
| 🔍 Diagnostics | System Health | Monitor performance |
| ❓ Help | Help | User documentation |

### Tabs

1. **Welcome** - Getting started guide
2. **Quantum Simulator** - Interactive simulation interface
3. **Theorem Prover** - Symbolic mathematics
4. **Results** - View simulation output

## 🎨 Screenshots

### Main Window
```
┌─────────────────────────────────────────────────────────────────┐
│ File  Edit  Tools  View  Help                                   │
├─────────────────────────────────────────────────────────────────┤
│ 🆕 New │ 📂 Open │ 💾 Save ║ ▶️ Simulate │ 🔬 Prove │ 📊 Fit  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ╔══════════════════════════════════════════════════════════╗  │
│  ║   Quantum Theory Engine - Professional Edition           ║  │
│  ╚══════════════════════════════════════════════════════════╝  │
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐  │
│  │  Welcome │ Quantum Simulator │ Theorem Prover │ Results │  │
│  ├─────────────────────────────────────────────────────────┤  │
│  │                                                           │  │
│  │   [Content Area]                                         │  │
│  │                                                           │  │
│  └─────────────────────────────────────────────────────────┘  │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│ Ready                         │ System: Online │ 02:30:45 PM    │
└─────────────────────────────────────────────────────────────────┘
```

### Quantum Simulator
```
┌─────────────────┬───────────────────────────────────────────────┐
│ Simulation      │ Simulation Output                             │
│ Controls        │ ═══════════════════════════════════════════   │
│─────────────────│                                               │
│ Template:       │ [13:47:23] Simulation completed!              │
│ [Rabi Osc. ▼]   │ Template: Rabi Oscillations                   │
│                 │ ω = 1.5 rad/s                                 │
│ Parameters:     │ Time = 10.0 s                                 │
│ ω: [1.5    ]    │ Final fidelity: 99.94%                        │
│ Time: [10.0 ]   │ Execution time: 247 ms                        │
│ Steps: [100 ]   │                                               │
│                 │                                               │
│ Progress:       │                                               │
│ [████████] 100% │                                               │
│                 │                                               │
│ [▶️ Run]        │                                               │
│ [⏹️ Stop]       │                                               │
└─────────────────┴───────────────────────────────────────────────┘
```

## 🔧 Customization

The GUI is a single Python file - easy to customize!

### Change Color Scheme
Edit the `self.colors` dictionary in `gui_app.py`:
```python
self.colors = {
    'bg': '#ECE9D8',        # Background (beige)
    'active': '#316AC5',    # Active elements (blue)
    'success': '#00A000',   # Success messages (green)
    # ... customize any color
}
```

### Change Window Size
```python
self.root.geometry("1280x800")  # Width x Height
```

### Add Custom Menu Items
```python
file_menu.add_command(label="My Feature", command=my_function)
```

## 🗂️ What Gets Installed?

```
~/.quantum-theory-engine/
├── gui_app.py              # Main application
└── launch-gui              # Launcher script

~/Applications/
└── Quantum Theory Engine.app/    # macOS app bundle
    └── Contents/
        ├── MacOS/
        │   └── Quantum Theory Engine
        └── Info.plist

~/.local/bin/
└── qte-gui                 # Command-line shortcut
```

## 🗑️ Uninstall

```bash
rm -rf ~/.quantum-theory-engine
rm ~/.local/bin/qte-gui
rm -rf ~/Applications/Quantum\ Theory\ Engine.app
```

## 🐛 Troubleshooting

### tkinter not found
**macOS:**
```bash
brew install python-tk@3.9
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get install python3-tk
```

**Linux (Fedora):**
```bash
sudo dnf install python3-tkinter
```

### Application doesn't open on macOS
Try running from terminal to see error:
```bash
python3 gui_app.py
```

### Window too small/large
Edit `gui_app.py` and change:
```python
self.root.geometry("1024x768")  # Adjust size here
```

### Colors look wrong
Make sure you're using Python 3.6+ with modern tkinter support.

## 💡 Pro Tips

1. **Full Screen:** Most systems support F11 for fullscreen mode
2. **Keyboard Shortcuts:** 
   - `Ctrl+N` - New Simulation
   - Menu access via `Alt` key (Windows/Linux)
3. **High DPI:** The app auto-scales on Retina/HiDPI displays
4. **Multiple Windows:** You can run multiple instances
5. **Themes:** While designed for classic look, tkinter themes can be applied

## 🎮 Feature Highlights

### Quantum Simulator
- Select from 5 pre-configured templates
- Adjust parameters in real-time
- Live progress monitoring
- Detailed output logs

### Theorem Prover
- Enter mathematical statements
- Automated proof generation
- Step-by-step proof transcript
- Confidence metrics

### Template Library
- 7 validated quantum experiments
- Detailed descriptions
- Status indicators
- One-click access

### System Diagnostics
- Real-time health monitoring
- 8 system components tracked
- Visual status indicators
- Performance metrics

## 🔗 Integration

### Use with Full Platform

This GUI is a standalone interface. For the complete platform:

1. **Full Rust Engine:** See main [README.md](README.md)
2. **Python Bindings:** Build with `maturin develop`
3. **CLI Tools:** Use `qte` command-line interface
4. **Docker:** Deploy with `docker-compose up`

The GUI can be extended to call actual backends once compiled.

## 🌐 Links

- **Full Project:** [Quantum Theory Engine](https://github.com/CFDefi/VanFoCO/tree/main/projects/quantum-theory-engine)
- **Retro Terminal UI:** See [RETRO_UI_README.md](RETRO_UI_README.md)
- **Report Issues:** [GitHub Issues](https://github.com/CFDefi/VanFoCO/issues)

## 📜 License

MIT License - Free to use, modify, and distribute!

## 🎯 What's Different from Terminal UI?

| Feature | Terminal UI | Desktop GUI |
|---------|-------------|-------------|
| **Interface** | Text-based | Graphical windows |
| **Navigation** | Keyboard menu | Mouse + keyboard |
| **Style** | 90s mainframe | 2000s desktop app |
| **Multitasking** | Sequential | Tabbed interface |
| **Visual Feedback** | ASCII art | Progress bars |
| **Accessibility** | High | High |
| **Installation** | None required | Creates app bundle |

Both interfaces access the same quantum simulation capabilities!

---

**Made with 🖥️ for desktop application enthusiasts**

*"Computing like Windows XP, with quantum power from 2026"*
