# Professional UI/UX Design Specification

## Overview

The Quantum Theory Engine features a modern, professional scientific UI built with:
- **Tauri** - Lightweight Rust-powered desktop application framework
- **React + TypeScript** - Component-based UI with type safety
- **Monaco Editor** - VSCode-quality code editing experience
- **TailwindCSS** - Professional, consistent styling
- **Plotly.js** - Scientific visualization
- **Lucide React** - Professional icon system

---

## Application Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    DESKTOP APPLICATION (Tauri)               │
├────────────────────────┬────────────────────────────────────┤
│   Frontend (React)     │    Backend (Rust)                  │
│                        │                                     │
│  • Monaco Editor       │    • Quantum Engine Core           │
│  • Visualization       │    • File System Access            │
│  • Proof Viewer        │    • Job Queue Manager             │
│  • Dashboard           │    • Websocket Server              │
│  • Settings            │    • State Management              │
└────────────────────────┴────────────────────────────────────┘
```

---

## Main Interface Layout

```
┌──────────────────────────────────────────────────────────────────┐
│  ⚛  Quantum Theory Engine            [_] [□] [×]                │
├────────┬─────────────────────────────────────────────────────────┤
│        │  📄 model.phys                     ⚙ Settings  ▶ Run   │
│  FILE  ├─────────────────────────────────────────────────────────┤
│  TREE  │  1  const omega = 1.0;                                  │
│        │  2  const Omega = 0.2;                                  │
│  📁 Pro│  3                                                       │
│  ├─📁 d│  4  matrix sigma_x = [[0, 1], [1, 0]];                 │
│  │ ├─📄│  5  matrix sigma_z = [[1, 0], [0, -1]];                │
│  │ └─📄│  6                                                       │
│  ├─📁 r│  7  Hamiltonian H = (omega/2) * sigma_z +              │
│  │ └─📄│  8                   Omega * sigma_x;                   │
│  └─📁 o│  9                                                       │
│    ├─📄│ 10  experiment rabi {                                   │
│    └─📄│ 11      init: ket(vec(1, 0));                           │
│        │ 12      evolution: evolve(init, H, timegrid=(...));    │
│        │ 13      measurements: [(0.0, z_basis), (5.0, z_basis)];│
│  [+]   │ 14  }                                                    │
├────────┼─────────────────────────────────────────────────────────┤
│ OUTPUT │  ✓ Parse successful                                     │
│  [×]   │  ✓ Type check passed                                    │
│        │  ✓ Quantum validation passed                            │
│  ⚠ Pro │  ▶ Executing simulation...                              │
│  ℹ Info│    Progress: ████████░░ 80% (400/500 timesteps)        │
│  ✓ Succ│                                                          │
│        │                                                          │
└────────┴─────────────────────────────────────────────────────────┘
```

---

## Component Specifications

### 1. Monaco Editor Integration

**Features**:
- Syntax highlighting for `.phys` DSL files
- Autocomplete for built-in functions and symbols
- Inline error diagnostics with quick fixes
- Hover tooltips for documentation
- Jump-to-definition for symbols
- Code folding and minimap
- Multiple themes (Light, Dark, High Contrast)

**Configuration**:
```typescript
const editorConfig = {
  language: 'quantum-dsl',
  theme: 'vs-dark',
  minimap: { enabled: true },
  fontSize: 14,
  lineNumbers: 'on',
  renderWhitespace: 'selection',
  bracketPairColorization: { enabled: true },
  inlineSuggest: { enabled: true },
  quickSuggestions: {
    other: true,
    comments: false,
    strings: false
  }
};
```

### 2. File Explorer

**Features**:
- Tree view of project structure
- File/folder create, rename, delete
- Drag-and-drop to organize
- Recent files quick access
- Search in files
- Workspace management

**Icons** (Lucide React):
- 📁 `FolderIcon` - Directories
- 📄 `FileTextIcon` - `.phys` DSL files
- 📊 `FileSpreadsheetIcon` - CSV data
- 📈 `BarChartIcon` - HDF5 results
- 🔬 `FlaskConicalIcon` - Experiments
- 🧮 `BracesIcon` - Proofs

### 3. Output Panel

**Tabs**:
- **Problems**: Error and warning list with quick navigation
- **Terminal**: Command output from CLI
- **Proof Trace**: Step-by-step derivation viewer
- **Diagnostics**: Trace drift, positivity checks

**Problem List Format**:
```
┌──────┬─────────┬──────────┬─────────────────────────────────┐
│ Sev  │ File    │ Line:Col │ Message                         │
├──────┼─────────┼──────────┼─────────────────────────────────┤
│ ⚠    │model.phys│ 12:18   │ Dimension mismatch: expected    │
│      │         │          │ 2×2, found 2×3                  │
│      │         │          │ 💡 Hint: Try transpose(B)       │
├──────┼─────────┼──────────┼─────────────────────────────────┤
│ ℹ    │model.phys│ 15:8    │ Trace drift: 1.2e-9 (within tol)│
└──────┴─────────┴──────────┴─────────────────────────────────┘
```

### 4. Visualization Panel

**Components**:

#### A. Bloch Sphere (Qubit State Visualization)
```typescript
interface BlochSphereProps {
  stateVector: Complex[];
  showAxes: boolean;
  showTrajectory: boolean;
  trajectoryData?: BlochPoint[];
}
```

Features:
- Interactive 3D rotation with mouse/touch
- Real-time state evolution animation
- Trajectory tracing for time evolution
- Axis labels (|0⟩, |1⟩, |+⟩, |−⟩, |+i⟩, |−i⟩)

#### B. Population Dynamics Plot
```typescript
interface PopulationPlotProps {
  times: number[];
  populations: number[][];  // [n_states][n_times]
  labels: string[];
  showFit?: boolean;
  experimentalData?: {
    times: number[];
    counts: number[][];
  };
}
```

Features:
- Multi-line plots with legend
- Zoom and pan
- Overlay experimental data points
- Confidence bands for uncertainties
- Export to PNG/SVG

#### C. Density Matrix Heatmap
```typescript
interface DensityMatrixProps {
  rho: Complex[][];
  showPhase: boolean;
  colormap: 'viridis' | 'plasma' | 'rdbu';
}
```

Features:
- Real and imaginary parts
- Phase visualization (HSV colormap)
- Value tooltips on hover
- Eigenvalue display

#### D. Measurement Histogram
```typescript
interface MeasurementHistogramProps {
  outcomes: number[];
  probabilities: number[];
  experimentalCounts?: number[];
  labels: string[];
}
```

Features:
- Bar chart with error bars
- Theory vs experiment overlay
- Chi-square goodness-of-fit display
- Outcome probabilities table

### 5. Proof Viewer

**Layout**:
```
┌─────────────────────────────────────────────────────────────┐
│  Proof: [σx, σy] == 2i σz                              ✓    │
├─────────────────────────────────────────────────────────────┤
│  Step 1: Canonical form                                     │
│  [σx, σy] → σx·σy - σy·σx                                   │
│                                                              │
│  Step 2: Pauli product expansion                            │
│  σx·σy → i·σz                                               │
│  σy·σx → -i·σz                                              │
│                                                              │
│  Step 3: Simplify difference                                │
│  i·σz - (-i·σz) → 2i·σz                                     │
│                                                              │
│  QED  ✓                                                      │
│                                                              │
│  📜 Certificate: a3f2b8c9... (click to verify)              │
│  📋 Copy LaTeX  📄 Export PDF                               │
└─────────────────────────────────────────────────────────────┘
```

**Features**:
- Collapsible proof steps
- LaTeX rendering for equations
- Rule highlighting (color-coded)
- Assumption tracking
- Certificate export and verification
- Share proof as permalink

### 6. Job Queue Dashboard

**Layout**:
```
┌─────────────────────────────────────────────────────────────┐
│  Job Queue                    [Pause] [Clear Completed]     │
├────┬─────────────────┬──────────┬────────────┬──────────────┤
│ ID │ Type            │ Status   │ Progress   │ Actions      │
├────┼─────────────────┼──────────┼────────────┼──────────────┤
│ 1  │ Simulate (rabi) │ Complete │ ████████   │ 📊 View      │
│ 2  │ Fit (amp_damp)  │ Running  │ ██████░░   │ ⏸ Pause      │
│ 3  │ Prove (identity)│ Queued   │ ░░░░░░░░   │ 🗑 Cancel    │
│ 4  │ Sweep (omega)   │ Queued   │ ░░░░░░░░   │ 🗑 Cancel    │
├────┼─────────────────┼──────────┼────────────┼──────────────┤
│                      Completed: 15  Running: 1  Queued: 2   │
└─────────────────────────────────────────────────────────────┘
```

**Features**:
- Real-time progress updates via WebSocket
- Priority adjustment
- Batch operations (pause all, cancel all)
- Result quick view
- Resource usage monitoring (CPU/Memory)
- Time estimates

### 7. Parameter Sweep Configurator

**Interface**:
```
┌─────────────────────────────────────────────────────────────┐
│  Parameter Sweep Configuration                              │
├─────────────────────────────────────────────────────────────┤
│  Base Model: rabi.phys                                      │
│                                                              │
│  Parameters:                                                 │
│  ┌──────────────────────────────────────────────────────┐   │
│  │ omega:  [Start] 0.5  [End] 2.0  [Steps] 10  Linear  │   │
│  │ Omega:  [Start] 0.1  [End] 0.5  [Steps] 5   Linear  │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
│  Total Runs: 50                                              │
│  Estimated Time: ~25 seconds                                 │
│                                                              │
│  Output:                                                     │
│  [x] Save individual results                                 │
│  [x] Generate aggregate report                               │
│  [x] Create heatmap visualization                            │
│                                                              │
│  [Cancel]                          [Preview] [Start Sweep]  │
└─────────────────────────────────────────────────────────────┘
```

### 8. Settings Panel

**Categories**:

#### Editor
- Theme (Light/Dark/High Contrast)
- Font size
- Tab size
- Word wrap
- Minimap enable/disable

#### Execution
- Default backend (CPU Dense/CPU Sparse/GPU)
- Max parallel jobs
- Timeout duration
- Autosave frequency

#### Validation
- Tolerance levels
- Strict mode
- Warning thresholds

#### Visualization
- Default colormap
- Animation speed
- Plot DPI
- Export format

#### Advanced
- Enable SMT solver for proofs
- Proof search depth limit
- Cache size
- Debug logging

---

## Icon System (Lucide React)

| Function | Icon | Component |
|----------|------|-----------|
| Run simulation | `Play` | Main toolbar |
| Stop execution | `Square` | Main toolbar |
| Prove identity | `CheckCircle` | Proof panel |
| Settings | `Settings` | Top bar |
| Save | `Save` | File menu |
| Open | `FolderOpen` | File menu |
| New file | `FilePlus` | File menu |
| Delete | `Trash2` | Context menu |
| Expand | `ChevronRight` | Tree view |
| Collapse | `ChevronDown` | Tree view |
| Warning | `AlertTriangle` | Diagnostics |
| Error | `XCircle` | Diagnostics |
| Info | `Info` | Diagnostics |
| Success | `CheckCircle` | Diagnostics |
| Copy | `Copy` | Code actions |
| Download | `Download` | Export |
| Upload | `Upload` | Import |
| Search | `Search` | Find |
| Refresh | `RefreshCw` | Reload |
| Help | `HelpCircle` | Documentation |
| Quantum | `Atom` | App icon |
| Graph | `TrendingUp` | Plots |
| Table | `Table` | Data view |

---

## Color Scheme

### Light Theme
```css
--background: #ffffff;
--foreground: #1e293b;
--primary: #3b82f6;
--secondary: #64748b;
--accent: #8b5cf6;
--success: #10b981;
--warning: #f59e0b;
--error: #ef4444;
--border: #e2e8f0;
```

### Dark Theme
```css
--background: #0f172a;
--foreground: #f1f5f9;
--primary: #60a5fa;
--secondary: #94a3b8;
--accent: #a78bfa;
--success: #34d399;
--warning: #fbbf24;
--error: #f87171;
--border: #334155;
```

---

## Responsive Breakpoints

```css
/* Mobile: < 768px */
@media (max-width: 767px) {
  /* Single column layout */
  /* Hide file tree by default */
  /* Bottom sheet for output */
}

/* Tablet: 768px - 1024px */
@media (min-width: 768px) and (max-width: 1023px) {
  /* Two column layout */
  /* Collapsible file tree */
}

/* Desktop: >= 1024px */
@media (min-width: 1024px) {
  /* Full three-panel layout */
}
```

---

## Keyboard Shortcuts

| Action | Shortcut (Mac) | Shortcut (Windows/Linux) |
|--------|----------------|--------------------------|
| Run | ⌘ + Enter | Ctrl + Enter |
| Save | ⌘ + S | Ctrl + S |
| Open | ⌘ + O | Ctrl + O |
| Find | ⌘ + F | Ctrl + F |
| Replace | ⌘ + H | Ctrl + H |
| Comment | ⌘ + / | Ctrl + / |
| Format | ⌥ + ⇧ + F | Alt + Shift + F |
| Settings | ⌘ + , | Ctrl + , |
| Toggle sidebar | ⌘ + B | Ctrl + B |
| Toggle terminal | ⌃ + ` | Ctrl + ` |
| New file | ⌘ + N | Ctrl + N |
| Close file | ⌘ + W | Ctrl + W |

---

## Accessibility

### WCAG 2.1 AA Compliance
- Color contrast ratio ≥ 4.5:1 for text
- All interactive elements keyboard accessible
- ARIA labels for screen readers
- Focus indicators on all focusable elements
- Skip navigation links

### Screen Reader Support
- Semantic HTML
- Alt text for all icons
- Live regions for dynamic content
- Descriptive labels for form inputs

---

## Performance Targets

- **Initial load**: < 2 seconds
- **Editor response**: < 16ms (60 FPS)
- **Syntax highlighting**: < 50ms for 10K line file
- **Plot rendering**: < 100ms for 1000 points
- **Job queue update**: < 50ms via WebSocket
- **Memory usage**: < 500MB for typical session

---

## Next Steps

See [ui/README.md](../ui/README.md) for implementation details and component library.
