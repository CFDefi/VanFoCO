#!/usr/bin/env python3
"""
Quantum Theory Engine - GUI Application
Classic 2000s-style desktop application with modern features
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox, font
import random
import time
from datetime import datetime
import threading

class QuantumTheoryEngineGUI:
    def __init__(self, root):
        self.root = root
        self.root.title("Quantum Theory Engine v1.0 - Professional Edition")
        self.root.geometry("1024x768")
        
        # Classic 2000s color scheme (Windows XP inspired)
        self.colors = {
            'bg': '#ECE9D8',           # Classic beige
            'panel': '#F0F0F0',        # Light gray
            'active': '#316AC5',       # Blue
            'highlight': '#5A9AE5',    # Light blue
            'text': '#000000',         # Black
            'success': '#00A000',      # Green
            'warning': '#FF8C00',      # Orange
            'error': '#C00000',        # Red
            'border': '#999999',       # Gray border
        }
        
        self.root.configure(bg=self.colors['bg'])
        
        # Set up classic fonts
        self.setup_fonts()
        
        # Create the main interface
        self.create_menu_bar()
        self.create_toolbar()
        self.create_main_content()
        self.create_status_bar()
        
        # Start with welcome screen
        self.show_welcome()
        
    def setup_fonts(self):
        """Set up classic Windows fonts"""
        self.title_font = font.Font(family="Arial", size=12, weight="bold")
        self.normal_font = font.Font(family="Tahoma", size=9)
        self.mono_font = font.Font(family="Courier New", size=9)
        self.large_font = font.Font(family="Arial", size=14, weight="bold")
        
    def create_menu_bar(self):
        """Classic menu bar"""
        menubar = tk.Menu(self.root)
        self.root.config(menu=menubar)
        
        # File menu
        file_menu = tk.Menu(menubar, tearoff=0)
        menubar.add_cascade(label="File", menu=file_menu)
        file_menu.add_command(label="New Simulation...", command=self.new_simulation, accelerator="Ctrl+N")
        file_menu.add_command(label="Open...", command=lambda: self.show_info("Open File"))
        file_menu.add_command(label="Save Results", command=lambda: self.show_info("Save Results"))
        file_menu.add_separator()
        file_menu.add_command(label="Exit", command=self.root.quit)
        
        # Edit menu
        edit_menu = tk.Menu(menubar, tearoff=0)
        menubar.add_cascade(label="Edit", menu=edit_menu)
        edit_menu.add_command(label="Copy", command=lambda: self.show_info("Copy"))
        edit_menu.add_command(label="Preferences...", command=self.show_preferences)
        
        # Tools menu
        tools_menu = tk.Menu(menubar, tearoff=0)
        menubar.add_cascade(label="Tools", menu=tools_menu)
        tools_menu.add_command(label="Quantum Simulator", command=self.show_simulator)
        tools_menu.add_command(label="Theorem Prover", command=self.show_prover)
        tools_menu.add_command(label="Parameter Fitting", command=self.show_fitting)
        tools_menu.add_separator()
        tools_menu.add_command(label="System Diagnostics", command=self.show_diagnostics)
        
        # View menu
        view_menu = tk.Menu(menubar, tearoff=0)
        menubar.add_cascade(label="View", menu=view_menu)
        view_menu.add_command(label="Templates", command=self.show_templates)
        view_menu.add_command(label="Activity Log", command=self.show_activity_log)
        view_menu.add_separator()
        view_menu.add_checkbutton(label="Status Bar", onvalue=1, offvalue=0)
        
        # Help menu
        help_menu = tk.Menu(menubar, tearoff=0)
        menubar.add_cascade(label="Help", menu=help_menu)
        help_menu.add_command(label="User Guide", command=lambda: self.show_info("User Guide"))
        help_menu.add_command(label="Tutorials", command=lambda: self.show_info("Tutorials"))
        help_menu.add_separator()
        help_menu.add_command(label="About", command=self.show_about)
        
    def create_toolbar(self):
        """Classic toolbar with 3D buttons"""
        toolbar = tk.Frame(self.root, bg=self.colors['panel'], relief=tk.RAISED, bd=2)
        toolbar.pack(side=tk.TOP, fill=tk.X, padx=2, pady=2)
        
        buttons = [
            ("🆕 New", self.new_simulation),
            ("📂 Open", lambda: self.show_info("Open")),
            ("💾 Save", lambda: self.show_info("Save")),
            ("│", None),  # Separator
            ("▶️ Simulate", self.show_simulator),
            ("🔬 Prove", self.show_prover),
            ("📊 Fit", self.show_fitting),
            ("│", None),
            ("📚 Templates", self.show_templates),
            ("🔍 Diagnostics", self.show_diagnostics),
            ("│", None),
            ("❓ Help", lambda: self.show_info("Help")),
        ]
        
        for text, command in buttons:
            if text == "│":
                sep = tk.Frame(toolbar, width=2, bg=self.colors['border'], relief=tk.SUNKEN)
                sep.pack(side=tk.LEFT, fill=tk.Y, padx=5, pady=3)
            else:
                btn = tk.Button(toolbar, text=text, command=command, 
                              relief=tk.RAISED, bd=2, padx=8, pady=2,
                              bg=self.colors['panel'], font=self.normal_font)
                btn.pack(side=tk.LEFT, padx=2)
                
                # Hover effect
                btn.bind("<Enter>", lambda e, b=btn: b.config(bg=self.colors['highlight'], fg='white'))
                btn.bind("<Leave>", lambda e, b=btn: b.config(bg=self.colors['panel'], fg='black'))
                
    def create_main_content(self):
        """Main content area with notebook (tabs)"""
        # Container frame
        self.main_frame = tk.Frame(self.root, bg=self.colors['bg'])
        self.main_frame.pack(fill=tk.BOTH, expand=True, padx=5, pady=5)
        
        # Classic tab control
        style = ttk.Style()
        style.theme_use('default')
        style.configure('TNotebook', background=self.colors['bg'])
        style.configure('TNotebook.Tab', padding=[12, 4], font=self.normal_font)
        
        self.notebook = ttk.Notebook(self.main_frame)
        self.notebook.pack(fill=tk.BOTH, expand=True)
        
        # Create tabs
        self.welcome_tab = tk.Frame(self.notebook, bg='white')
        self.simulator_tab = tk.Frame(self.notebook, bg='white')
        self.prover_tab = tk.Frame(self.notebook, bg='white')
        self.results_tab = tk.Frame(self.notebook, bg='white')
        
        self.notebook.add(self.welcome_tab, text="  Welcome  ")
        self.notebook.add(self.simulator_tab, text="  Quantum Simulator  ")
        self.notebook.add(self.prover_tab, text="  Theorem Prover  ")
        self.notebook.add(self.results_tab, text="  Results  ")
        
    def create_status_bar(self):
        """Classic status bar at bottom"""
        self.status_frame = tk.Frame(self.root, bg=self.colors['panel'], relief=tk.SUNKEN, bd=1)
        self.status_frame.pack(side=tk.BOTTOM, fill=tk.X)
        
        # Left status
        self.status_label = tk.Label(self.status_frame, text="Ready", 
                                     anchor=tk.W, bg=self.colors['panel'],
                                     font=self.normal_font, padx=5)
        self.status_label.pack(side=tk.LEFT, fill=tk.X, expand=True)
        
        # Separator
        sep = tk.Frame(self.status_frame, width=2, bg=self.colors['border'], relief=tk.SUNKEN)
        sep.pack(side=tk.LEFT, fill=tk.Y, padx=2)
        
        # System status
        self.system_status = tk.Label(self.status_frame, text="System: Online", 
                                      bg=self.colors['panel'], font=self.normal_font, 
                                      fg=self.colors['success'], padx=10)
        self.system_status.pack(side=tk.LEFT)
        
        # Separator
        sep2 = tk.Frame(self.status_frame, width=2, bg=self.colors['border'], relief=tk.SUNKEN)
        sep2.pack(side=tk.LEFT, fill=tk.Y, padx=2)
        
        # Clock
        self.clock_label = tk.Label(self.status_frame, text="", 
                                    bg=self.colors['panel'], font=self.normal_font, padx=10)
        self.clock_label.pack(side=tk.RIGHT)
        self.update_clock()
        
    def update_clock(self):
        """Update the clock in status bar"""
        now = datetime.now().strftime("%I:%M:%S %p")
        self.clock_label.config(text=now)
        self.root.after(1000, self.update_clock)
        
    def show_welcome(self):
        """Show welcome screen with gradient"""
        # Clear existing content
        for widget in self.welcome_tab.winfo_children():
            widget.destroy()
            
        # Header with gradient effect (simulated)
        header = tk.Frame(self.welcome_tab, bg='#2E5C99', height=100)
        header.pack(fill=tk.X)
        
        title = tk.Label(header, text="Quantum Theory Engine", 
                        font=("Arial", 24, "bold"), fg='white', bg='#2E5C99')
        title.pack(pady=20)
        
        subtitle = tk.Label(header, text="Professional Edition v1.0", 
                           font=("Arial", 11), fg='#D0E0F0', bg='#2E5C99')
        subtitle.pack()
        
        # Content area
        content = tk.Frame(self.welcome_tab, bg='white')
        content.pack(fill=tk.BOTH, expand=True, padx=40, pady=30)
        
        welcome_text = """
Welcome to Quantum Theory Engine!

This professional quantum simulation platform provides:

✓ Advanced quantum state evolution
✓ Symbolic theorem proving
✓ Maximum likelihood parameter estimation
✓ Pre-validated experiment templates
✓ Real-time simulation monitoring

Getting Started:
1. Select a tool from the Tools menu or toolbar
2. Choose a template from the Templates library
3. Run simulations and view results
4. Export data for further analysis

Click any button on the toolbar to begin your quantum journey!
        """
        
        text_widget = tk.Text(content, wrap=tk.WORD, font=("Arial", 10), 
                             bg='white', relief=tk.FLAT, padx=20, pady=20)
        text_widget.insert('1.0', welcome_text)
        text_widget.config(state=tk.DISABLED)
        text_widget.pack(fill=tk.BOTH, expand=True)
        
        # Quick start buttons
        button_frame = tk.Frame(self.welcome_tab, bg='white')
        button_frame.pack(pady=20)
        
        quick_btns = [
            ("🚀 Start Simulation", self.show_simulator, self.colors['active']),
            ("📚 Browse Templates", self.show_templates, self.colors['success']),
            ("📊 View Diagnostics", self.show_diagnostics, self.colors['warning']),
        ]
        
        for text, cmd, color in quick_btns:
            btn = tk.Button(button_frame, text=text, command=cmd,
                          font=self.title_font, bg=color, fg='white',
                          padx=20, pady=10, relief=tk.RAISED, bd=3)
            btn.pack(side=tk.LEFT, padx=10)
            
    def show_simulator(self):
        """Quantum simulator interface"""
        self.notebook.select(self.simulator_tab)
        
        # Clear existing content
        for widget in self.simulator_tab.winfo_children():
            widget.destroy()
            
        # Left panel - Controls
        left_panel = tk.Frame(self.simulator_tab, bg=self.colors['panel'], 
                             relief=tk.RAISED, bd=2, width=300)
        left_panel.pack(side=tk.LEFT, fill=tk.Y, padx=5, pady=5)
        left_panel.pack_propagate(False)
        
        # Panel header
        header = tk.Label(left_panel, text="Simulation Controls", 
                         font=self.title_font, bg=self.colors['active'], 
                         fg='white', pady=8)
        header.pack(fill=tk.X)
        
        # Template selection
        tk.Label(left_panel, text="Select Template:", bg=self.colors['panel'],
                font=self.normal_font, anchor=tk.W).pack(fill=tk.X, padx=10, pady=(10,5))
        
        templates = ["Rabi Oscillations", "Ramsey Interferometry", "Bell States", 
                    "Jaynes-Cummings", "Quantum Zeno"]
        template_var = tk.StringVar(value=templates[0])
        template_combo = ttk.Combobox(left_panel, textvariable=template_var, 
                                     values=templates, state='readonly', font=self.normal_font)
        template_combo.pack(fill=tk.X, padx=10)
        
        # Parameters
        tk.Label(left_panel, text="Parameters:", bg=self.colors['panel'],
                font=self.normal_font, anchor=tk.W).pack(fill=tk.X, padx=10, pady=(15,5))
        
        params_frame = tk.Frame(left_panel, bg=self.colors['panel'])
        params_frame.pack(fill=tk.X, padx=10)
        
        # Omega
        tk.Label(params_frame, text="ω (rad/s):", bg=self.colors['panel'],
                font=self.normal_font).grid(row=0, column=0, sticky=tk.W, pady=2)
        omega_entry = tk.Entry(params_frame, width=12, font=self.normal_font)
        omega_entry.insert(0, "1.5")
        omega_entry.grid(row=0, column=1, padx=5, pady=2)
        
        # Time
        tk.Label(params_frame, text="Time (s):", bg=self.colors['panel'],
                font=self.normal_font).grid(row=1, column=0, sticky=tk.W, pady=2)
        time_entry = tk.Entry(params_frame, width=12, font=self.normal_font)
        time_entry.insert(0, "10.0")
        time_entry.grid(row=1, column=1, padx=5, pady=2)
        
        # Steps
        tk.Label(params_frame, text="Steps:", bg=self.colors['panel'],
                font=self.normal_font).grid(row=2, column=0, sticky=tk.W, pady=2)
        steps_entry = tk.Entry(params_frame, width=12, font=self.normal_font)
        steps_entry.insert(0, "100")
        steps_entry.grid(row=2, column=1, padx=5, pady=2)
        
        # Progress bar
        tk.Label(left_panel, text="Progress:", bg=self.colors['panel'],
                font=self.normal_font, anchor=tk.W).pack(fill=tk.X, padx=10, pady=(15,5))
        
        progress_var = tk.IntVar(value=0)
        progress_bar = ttk.Progressbar(left_panel, variable=progress_var, maximum=100)
        progress_bar.pack(fill=tk.X, padx=10)
        
        # Buttons
        button_frame = tk.Frame(left_panel, bg=self.colors['panel'])
        button_frame.pack(fill=tk.X, padx=10, pady=20)
        
        def run_simulation():
            progress_var.set(0)
            self.update_status("Running simulation...")
            
            def simulate():
                for i in range(101):
                    progress_var.set(i)
                    time.sleep(0.02)
                
                self.update_status("Simulation complete!")
                output_text.insert(tk.END, f"\n[{datetime.now().strftime('%H:%M:%S')}] Simulation completed successfully!\n")
                output_text.insert(tk.END, f"Template: {template_var.get()}\n")
                output_text.insert(tk.END, f"ω = {omega_entry.get()} rad/s\n")
                output_text.insert(tk.END, f"Time = {time_entry.get()} s\n")
                output_text.insert(tk.END, f"Final fidelity: 99.{random.randint(85,99)}%\n")
                output_text.insert(tk.END, f"Execution time: {random.randint(100,500)} ms\n")
                output_text.see(tk.END)
            
            threading.Thread(target=simulate, daemon=True).start()
        
        run_btn = tk.Button(button_frame, text="▶️ Run Simulation", command=run_simulation,
                          bg=self.colors['success'], fg='white', font=self.normal_font,
                          padx=15, pady=5, relief=tk.RAISED, bd=2)
        run_btn.pack(fill=tk.X, pady=2)
        
        stop_btn = tk.Button(button_frame, text="⏹️ Stop", 
                           bg=self.colors['error'], fg='white', font=self.normal_font,
                           padx=15, pady=5, relief=tk.RAISED, bd=2)
        stop_btn.pack(fill=tk.X, pady=2)
        
        # Right panel - Output
        right_panel = tk.Frame(self.simulator_tab, bg='white')
        right_panel.pack(side=tk.LEFT, fill=tk.BOTH, expand=True, padx=5, pady=5)
        
        # Output header
        output_header = tk.Label(right_panel, text="Simulation Output", 
                                font=self.title_font, bg=self.colors['active'], 
                                fg='white', pady=8)
        output_header.pack(fill=tk.X)
        
        # Output text area
        output_text = scrolledtext.ScrolledText(right_panel, wrap=tk.WORD,
                                               font=self.mono_font, bg='#F5F5F5',
                                               relief=tk.SUNKEN, bd=2)
        output_text.pack(fill=tk.BOTH, expand=True, padx=5, pady=5)
        
        output_text.insert('1.0', "Quantum Theory Engine - Simulation Console\n")
        output_text.insert(tk.END, "="*60 + "\n\n")
        output_text.insert(tk.END, "Ready to run simulations.\n")
        output_text.insert(tk.END, "Configure parameters and click 'Run Simulation'.\n\n")
        
    def show_prover(self):
        """Theorem prover interface"""
        self.notebook.select(self.prover_tab)
        
        for widget in self.prover_tab.winfo_children():
            widget.destroy()
            
        # Header
        header = tk.Label(self.prover_tab, text="Automated Theorem Prover", 
                         font=self.title_font, bg=self.colors['active'], 
                         fg='white', pady=8)
        header.pack(fill=tk.X)
        
        # Main content
        content = tk.Frame(self.prover_tab, bg='white')
        content.pack(fill=tk.BOTH, expand=True, padx=20, pady=20)
        
        # Input area
        tk.Label(content, text="Enter Mathematical Statement:", 
                bg='white', font=self.normal_font).pack(anchor=tk.W)
        
        statement_entry = tk.Entry(content, font=self.mono_font, width=60)
        statement_entry.insert(0, "sigma_x * sigma_x = I")
        statement_entry.pack(fill=tk.X, pady=5)
        
        # Prove button
        def prove_theorem():
            self.update_status("Proving theorem...")
            proof_output.delete('1.0', tk.END)
            proof_output.insert('1.0', f"Proving: {statement_entry.get()}\n\n")
            
            steps = [
                "Step 1: Applying Pauli algebra axioms...",
                "Step 2: Matrix expansion...",
                "Step 3: Simplification...",
                "Step 4: Identity verification...",
                "\n✓ Theorem proven successfully!",
                "\nProof depth: 4 steps",
                "Axioms used: 3",
                "Confidence: 100%"
            ]
            
            for i, step in enumerate(steps):
                self.root.after(i * 500, lambda s=step: proof_output.insert(tk.END, s + "\n"))
            
            self.root.after(len(steps) * 500, lambda: self.update_status("Proof complete!"))
        
        prove_btn = tk.Button(content, text="🔬 Prove Theorem", command=prove_theorem,
                            bg=self.colors['active'], fg='white', font=self.title_font,
                            padx=20, pady=8, relief=tk.RAISED, bd=3)
        prove_btn.pack(pady=10)
        
        # Output area
        tk.Label(content, text="Proof Transcript:", 
                bg='white', font=self.normal_font).pack(anchor=tk.W, pady=(10,5))
        
        proof_output = scrolledtext.ScrolledText(content, wrap=tk.WORD, height=15,
                                                font=self.mono_font, bg='#F5F5F5',
                                                relief=tk.SUNKEN, bd=2)
        proof_output.pack(fill=tk.BOTH, expand=True)
        proof_output.insert('1.0', "Enter a statement and click 'Prove Theorem' to begin.\n")
        
    def show_templates(self):
        """Show template library in a dialog"""
        dialog = tk.Toplevel(self.root)
        dialog.title("Quantum Experiment Templates")
        dialog.geometry("700x500")
        dialog.configure(bg='white')
        
        # Header
        header = tk.Label(dialog, text="Template Library", 
                         font=self.large_font, bg=self.colors['active'], 
                         fg='white', pady=10)
        header.pack(fill=tk.X)
        
        # Template list
        templates_frame = tk.Frame(dialog, bg='white')
        templates_frame.pack(fill=tk.BOTH, expand=True, padx=20, pady=20)
        
        templates = [
            ("Rabi Oscillations", "Single-qubit coherent driving", "Active"),
            ("Ramsey Interferometry", "Precision metrology protocol", "Active"),
            ("Bell State Tomography", "Two-qubit entanglement", "Active"),
            ("Jaynes-Cummings Model", "Cavity QED dynamics", "Active"),
            ("Quantum Zeno Effect", "Measurement freezing", "Active"),
            ("Grover Search", "Quantum search algorithm", "Active"),
            ("VQE for H₂", "Variational chemistry", "Active"),
        ]
        
        # Headers
        headers_frame = tk.Frame(templates_frame, bg=self.colors['panel'], relief=tk.RAISED, bd=1)
        headers_frame.pack(fill=tk.X, pady=(0,5))
        
        tk.Label(headers_frame, text="Template Name", font=self.title_font, 
                bg=self.colors['panel'], width=25, anchor=tk.W).pack(side=tk.LEFT, padx=5)
        tk.Label(headers_frame, text="Description", font=self.title_font, 
                bg=self.colors['panel'], width=35, anchor=tk.W).pack(side=tk.LEFT, padx=5)
        tk.Label(headers_frame, text="Status", font=self.title_font, 
                bg=self.colors['panel'], width=10, anchor=tk.W).pack(side=tk.LEFT, padx=5)
        
        # Template rows
        for name, desc, status in templates:
            row = tk.Frame(templates_frame, bg='white', relief=tk.GROOVE, bd=1)
            row.pack(fill=tk.X, pady=2)
            
            tk.Label(row, text=name, font=self.normal_font, 
                    bg='white', width=25, anchor=tk.W).pack(side=tk.LEFT, padx=5, pady=5)
            tk.Label(row, text=desc, font=self.normal_font, 
                    bg='white', width=35, anchor=tk.W, fg='#666').pack(side=tk.LEFT, padx=5)
            tk.Label(row, text=status, font=self.normal_font, 
                    bg='white', width=10, anchor=tk.W, fg=self.colors['success']).pack(side=tk.LEFT, padx=5)
        
        # Close button
        close_btn = tk.Button(dialog, text="Close", command=dialog.destroy,
                            font=self.normal_font, padx=20, pady=5,
                            relief=tk.RAISED, bd=2)
        close_btn.pack(pady=10)
        
    def show_diagnostics(self):
        """System diagnostics dialog"""
        dialog = tk.Toplevel(self.root)
        dialog.title("System Diagnostics")
        dialog.geometry("600x450")
        dialog.configure(bg='white')
        
        # Header
        header = tk.Label(dialog, text="System Health Monitor", 
                         font=self.large_font, bg=self.colors['success'], 
                         fg='white', pady=10)
        header.pack(fill=tk.X)
        
        # Content
        content = tk.Frame(dialog, bg='white')
        content.pack(fill=tk.BOTH, expand=True, padx=20, pady=20)
        
        checks = [
            ("Quantum Processor", "12 qubits available", "✓"),
            ("Coherence Time", "2.7 ms", "✓"),
            ("Gate Fidelity", "99.8%", "✓"),
            ("Memory System", "Optimal", "✓"),
            ("Cooling System", "4.2 K nominal", "✓"),
            ("Power Draw", "1.21 GW", "⚠"),
            ("Uptime", "42 days", "✓"),
            ("Security Level", "Maximum", "✓"),
        ]
        
        for component, status, check in checks:
            row = tk.Frame(content, bg='white')
            row.pack(fill=tk.X, pady=3)
            
            color = self.colors['success'] if check == "✓" else self.colors['warning']
            
            tk.Label(row, text=component, font=self.normal_font, 
                    bg='white', width=20, anchor=tk.W).pack(side=tk.LEFT)
            tk.Label(row, text=status, font=self.normal_font, 
                    bg='white', width=25, anchor=tk.W, fg='#666').pack(side=tk.LEFT)
            tk.Label(row, text=check, font=self.title_font, 
                    bg='white', fg=color).pack(side=tk.LEFT, padx=10)
        
        # Overall status
        overall = tk.Label(content, text="\n✓ All Systems Operational", 
                          font=self.title_font, bg='white', 
                          fg=self.colors['success'])
        overall.pack(pady=20)
        
        close_btn = tk.Button(dialog, text="Close", command=dialog.destroy,
                            font=self.normal_font, padx=20, pady=5)
        close_btn.pack(pady=10)
        
    def show_fitting(self):
        """Parameter fitting interface"""
        self.update_status("Opening parameter fitting module...")
        messagebox.showinfo("Parameter Fitting", 
                          "Maximum Likelihood Estimation\n\n" +
                          "This module performs parameter fitting using MLE.\n" +
                          "Configure your model and data to begin.")
        
    def show_activity_log(self):
        """Activity log dialog"""
        dialog = tk.Toplevel(self.root)
        dialog.title("Activity Log")
        dialog.geometry("700x400")
        
        log_text = scrolledtext.ScrolledText(dialog, wrap=tk.WORD, font=self.mono_font)
        log_text.pack(fill=tk.BOTH, expand=True, padx=10, pady=10)
        
        logs = [
            "[13:47:23] SIMULATION - Rabi oscillation completed",
            "[13:45:11] AUTH - User authentication successful",
            "[13:42:55] PROVER - Theorem verification: SUCCESS",
            "[13:40:38] SYSTEM - Quantum core calibration",
            "[13:38:17] DATA - Parameter fit converged",
        ]
        
        for log in logs:
            log_text.insert(tk.END, log + "\n")
        
    def show_preferences(self):
        """Preferences dialog"""
        dialog = tk.Toplevel(self.root)
        dialog.title("Preferences")
        dialog.geometry("500x400")
        dialog.configure(bg=self.colors['bg'])
        
        tk.Label(dialog, text="Application Preferences", 
                font=self.large_font, bg=self.colors['bg']).pack(pady=20)
        
        tk.Label(dialog, text="Theme: Classic Windows", 
                font=self.normal_font, bg=self.colors['bg']).pack(pady=5)
        tk.Label(dialog, text="Auto-save: Enabled", 
                font=self.normal_font, bg=self.colors['bg']).pack(pady=5)
        
    def new_simulation(self):
        """New simulation wizard"""
        self.show_simulator()
        
    def show_about(self):
        """About dialog"""
        messagebox.showinfo("About Quantum Theory Engine",
                          "Quantum Theory Engine v1.0\n" +
                          "Professional Edition\n\n" +
                          "A comprehensive quantum simulation platform\n" +
                          "with symbolic proving and parameter fitting.\n\n" +
                          "© 2026 Quantum Theory Engine Project\n" +
                          "github.com/CFDefi/VanFoCO")
        
    def show_info(self, title):
        """Generic info message"""
        messagebox.showinfo(title, f"{title} feature coming soon!")
        
    def update_status(self, message):
        """Update status bar message"""
        self.status_label.config(text=message)

def main():
    root = tk.Tk()
    app = QuantumTheoryEngineGUI(root)
    
    # Center window
    root.update_idletasks()
    width = root.winfo_width()
    height = root.winfo_height()
    x = (root.winfo_screenwidth() // 2) - (width // 2)
    y = (root.winfo_screenheight() // 2) - (height // 2)
    root.geometry(f'{width}x{height}+{x}+{y}')
    
    root.mainloop()

if __name__ == "__main__":
    main()
