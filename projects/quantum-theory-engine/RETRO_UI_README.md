# 🎮 Quantum Theory Engine - Retro Supercomputer UI

**Experience quantum computing with authentic 90s supercomputer aesthetics**

![Version](https://img.shields.io/badge/version-1.0-brightgreen)
![Python](https://img.shields.io/badge/python-3.6+-blue)
![License](https://img.shields.io/badge/license-MIT-blue)

## ✨ Features

- 🖥️ **Authentic 90s Terminal Aesthetics** - Green phosphor CRT vibes
- ⚡ **Typewriter Effects** - Classic slow text rendering
- 📊 **Retro Progress Bars** - Block characters and ASCII art
- 🎨 **Color Coding** - Cyan, yellow, green terminal colors
- 🔒 **Security Logs** - Supercomputer-style access logs
- 🧪 **Quantum Simulations** - Interactive experiment launcher
- 🔬 **Theorem Proving** - Symbolic mathematics interface
- 📈 **Parameter Fitting** - Maximum likelihood estimation

## 🚀 Quick Install

### One-Line Install (Recommended)

```bash
curl -sL https://raw.githubusercontent.com/CFDefi/VanFoCO/main/projects/quantum-theory-engine/install.sh | bash
```

Then run:
```bash
qte-retro
```

### Manual Install

1. **Download the UI:**
   ```bash
   curl -O https://raw.githubusercontent.com/CFDefi/VanFoCO/main/projects/quantum-theory-engine/retro_ui.py
   chmod +x retro_ui.py
   ```

2. **Run it:**
   ```bash
   ./retro_ui.py
   ```

### From Source (If you cloned the repo)

```bash
cd projects/quantum-theory-engine
chmod +x retro_ui.py
./retro_ui.py
```

## 📋 Requirements

- **Python 3.6+** (pre-installed on macOS/Linux)
- **Terminal with ANSI color support** (default on macOS Terminal, iTerm2, Linux terminals)
- **No additional dependencies** - uses only Python standard library!

## 🎯 Usage

Launch the retro UI:
```bash
./retro_ui.py
```

Or if you used the installer:
```bash
qte-retro
```

### Main Menu Options

| Command | Feature | Description |
|---------|---------|-------------|
| `1` | Quantum Simulation | Execute quantum state evolution protocols |
| `2` | Theorem Prover | Symbolic mathematics verification engine |
| `3` | Parameter Fitting | Maximum likelihood estimation suite |
| `4` | Template Library | Browse pre-configured experiments |
| `5` | Diagnostics | System health and performance monitoring |
| `6` | Data Analysis | Sweep and optimization routines |
| `7` | Documentation | Access technical manuals |
| `8` | Security Log | View system activity logs |
| `0` | Shutdown | Exit the interface |

## 🎨 Screenshots (Text Mode)

### Boot Sequence
```
═══════════════════════════════════════════════════════════════════════
           QUANTUM SUPERCOMPUTER BOOT SEQUENCE
═══════════════════════════════════════════════════════════════════════

>> BIOS v3.14159
>> Memory check: 2048 TB OK
>> Quantum processor initialization
>> Loading symbolic kernel...
>> Mounting quantum file systems
>> Starting neural network substrate
>> Engaging flux capacitor
>> Initializing user interface

█████████████████████████████████████████████████████████████████████████
                   BOOT COMPLETE - SYSTEM READY
█████████████████████████████████████████████████████████████████████████
```

### Main Interface
```
╔══════════════════════════════════════════════════════════════════════════╗
║                                                                          ║
║  ██████╗ ████████╗███████╗    ███████╗██╗   ██╗███████╗████████╗███████╗║
║ ██╔═══██╗╚══██╔══╝██╔════╝    ██╔════╝╚██╗ ██╔╝██╔════╝╚══██╔══╝██╔════╝║
║ ██║   ██║   ██║   █████╗      ███████╗ ╚████╔╝ ███████╗   ██║   █████╗  ║
║ ██║▄▄ ██║   ██║   ██╔══╝      ╚════██║  ╚██╔╝  ╚════██║   ██║   ██╔══╝  ║
║ ╚██████╔╝   ██║   ███████╗    ███████║   ██║   ███████║   ██║   ███████╗║
║  ╚══▀▀═╝    ╚═╝   ╚══════╝    ╚══════╝   ╚═╝   ╚══════╝   ╚═╝   ╚══════╝║
║                                                                          ║
║             Q U A N T U M   T H E O R Y   E N G I N E   v1.0            ║
╚══════════════════════════════════════════════════════════════════════════╝

╔═════════════════ SYSTEM STATUS ════════════════╗
║ QUANTUM CORE         █████████████ [    ONLINE] ║
║ SYMBOLIC ENGINE      █████████████ [     READY] ║
║ NEURAL SUBSTRATE     █████████████ [    ACTIVE] ║
║ CRYO COOLING         ██████████    [   NOMINAL] ║
╚════════════════════════════════════════════════╝
```

## 🔧 Customization

The UI is a single Python file - easy to customize!

Edit `retro_ui.py` to:
- Change color schemes (modify `Colors` class)
- Adjust animation speeds (change `time.sleep()` values)
- Add new menu items (extend `show_main_menu()`)
- Create custom screens (add new functions)

## 🐛 Troubleshooting

### Colors don't appear correctly
- Make sure you're using a terminal with ANSI color support
- Try iTerm2 on macOS or modern Linux terminals

### Permission denied error
```bash
chmod +x retro_ui.py
```

### Python not found
```bash
# Check Python 3 installation
which python3

# Or use full path
/usr/bin/python3 retro_ui.py
```

## 📦 What Gets Installed?

When you run `install.sh`:

- **`~/.quantum-theory-engine/`** - Installation directory
  - `retro_ui.py` - Main UI script
  - `qte-retro` - Launcher script

- **`/usr/local/bin/qte-retro`** or **`~/.local/bin/qte-retro`** - Symlink for easy access

- **`~/Applications/QTE Retro.app`** (macOS only) - Double-clickable app

## 🗑️ Uninstall

```bash
rm -rf ~/.quantum-theory-engine
rm /usr/local/bin/qte-retro  # or ~/.local/bin/qte-retro
rm -rf ~/Applications/QTE\ Retro.app  # macOS only
```

## 🌐 Links

- **Full Project:** [Quantum Theory Engine](https://github.com/CFDefi/VanFoCO/tree/main/projects/quantum-theory-engine)
- **Main Repository:** [VanFoCO](https://github.com/CFDefi/VanFoCO)
- **Report Issues:** [GitHub Issues](https://github.com/CFDefi/VanFoCO/issues)

## 📜 License

MIT License - Feel free to use, modify, and distribute!

## 🎮 Pro Tips

1. **Full Screen Mode** - Press `Cmd+Enter` (macOS) or `F11` (Linux) for immersive experience
2. **Darker Background** - Use terminal preferences to set black background for maximum CRT effect
3. **Retro Font** - Install a monospace font like "IBM Plex Mono" or "Source Code Pro"
4. **Sound Effects** - Enable terminal bell in preferences for authentic beeps
5. **Screen Glow** - Some terminals support glow effects (iTerm2 with "Blur" setting)

## 🚀 What's Next?

This retro UI is a standalone interface to explore quantum concepts with style!

For the **full production platform** (Rust engine, Python bindings, Docker deployment):
- See the main [README.md](README.md)
- Follow the [USER_TUTORIAL.md](docs/USER_TUTORIAL.md)
- Build with `cargo build --release`

---

**Made with 💚 for terminal enthusiasts and quantum physicists**

*"Computing like it's 1995, with quantum physics from 2025"*
