# System Architecture: Quantum Theory Engine with Symbolic Prover

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         USER INTERFACES                                  │
├─────────────┬──────────────┬──────────────┬──────────────┬──────────────┤
│   CLI       │  Web UI      │  Python API  │  REPL        │  VS Code Ext │
│   (clap)    │  (Tauri)     │  (pyo3)      │  (rustyline) │  (LSP)       │
└─────────────┴──────────────┴──────────────┴──────────────┴──────────────┘
       │              │              │              │              │
       └──────────────┴──────────────┴──────────────┴──────────────┘
                                    │
              ┌─────────────────────┴─────────────────────┐
              │         ORCHESTRATION LAYER                │
              ├────────────────────────────────────────────┤
              │  • Job Queue (tokio queue)                 │
              │  • Parameter Sweep Manager                 │
              │  • Live Stream Coordinator                 │
              │  • Template Manager                        │
              │  • Manifest Generator/Verifier             │
              └────────────────────────────────────────────┘
                                    │
       ┌────────────────────────────┼────────────────────────────┐
       │                            │                            │
┌──────▼────────┐         ┌────────▼────────┐         ┌────────▼────────┐
│  COMPILER     │         │  PROVER         │         │  EXECUTOR       │
│  PIPELINE     │         │  MODULE         │         │  RUNTIME        │
├───────────────┤         ├─────────────────┤         ├─────────────────┤
│ 1.Parser      │◄────────┤ Rewrite Engine  │         │ Backend Manager │
│ 2.Name Resolve│         │ Canonicalizer   │         │ State Tracker   │
│ 3.Type Check  │────────►│ Proof Search    │         │ Measurement Eng │
│ 4.Validation  │         │ Certificate Gen │         │ Diagnostics     │
│ 5.Optimizer   │         │ Counterexample  │         │ Result Cache    │
│ 6.IR Lowering │         │ Verifier        │         │                 │
└───────┬───────┘         └─────────┬───────┘         └────────┬────────┘
        │                           │                          │
        │                           │                          │
        └───────────────────────────┼──────────────────────────┘
                                    │
              ┌─────────────────────┴─────────────────────┐
              │         CORE SERVICES                      │
              ├────────────────────────────────────────────┤
              │  • Kernels (CPU/GPU)                       │
              │  • ODE Integrators                         │
              │  • Statistics Library                      │
              │  • IO (CSV/HDF5/Parquet)                   │
              │  • Error Reporting                         │
              └────────────────────────────────────────────┘
```

---

## Module Specifications

### 1. PARSER MODULE

**Responsibility**: Convert DSL text → AST

**Public API**:
```rust
pub fn parse_dsl(source: &str) -> Result<Ast, ParseError>;
pub fn parse_expr(source: &str) -> Result<Expr, ParseError>;
pub fn format_parse_error(err: &ParseError, source: &str) -> String;
```

**Internal Components**:
- Pest PEG parser (existing)
- Span tracking for all nodes
- Error recovery for partial parses

**Dependencies**: `pest`, `pest_derive`

**Outputs**: `Ast` with full source location info

---

### 2. NAME RESOLUTION MODULE

**Responsibility**: Resolve all identifiers to definitions, build symbol table

**Public API**:
```rust
pub struct NameResolver {
    scopes: Vec<Scope>,
    symbol_table: SymbolTable,
}

impl NameResolver {
    pub fn new() -> Self;
    pub fn resolve(&mut self, ast: &Ast) -> Result<ResolvedAst, NameError>;
    pub fn lookup(&self, name: &str) -> Option<&Symbol>;
}

pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,  // Const, Matrix, Hamiltonian, Measurement, Function
    pub span: Span,
    pub type_hint: Option<Type>,
}
```

**Error Cases**:
- Undefined symbol
- Redefinition
- Circular dependency

---

### 3. TYPE CHECKER MODULE

**Responsibility**: Infer and check shapes/types, ensure dimensional consistency

**Public API**:
```rust
pub struct TypeChecker {
    symbol_table: SymbolTable,
    type_cache: HashMap<NodeId, Type>,
}

impl TypeChecker {
    pub fn check(&mut self, ast: &ResolvedAst) -> Result<TypedAst, TypeError>;
    pub fn infer_type(&mut self, expr: &Expr) -> Result<Type, TypeError>;
}

pub enum Type {
    Scalar(NumericKind),  // Real, Complex
    Vector(usize),        // dimension
    Matrix(usize, usize), // rows, cols
    Operator(usize),      // Hermitian operator on dim-d Hilbert space
    Density(usize),       // Density matrix (PSD, trace=1)
    Channel(usize),       // CPTP map
    Measurement(usize, usize),  // POVM with n_outcomes, dimension
}

pub enum NumericKind {
    Real,
    Complex,
    Unknown,
}
```

**Type Inference Rules**:
- `+`, `-`: requires same shape → same shape
- `*`: scalar × anything → broadcast; matrix × matrix → matmul rules
- `tensor(A, B)`: Matrix(m,n) × Matrix(p,q) → Matrix(m*p, n*q)
- `dagger(A)`: Matrix(m,n) → Matrix(n,m)
- `trace(A)`: Matrix(n,n) → Scalar
- `commutator(A,B)`: Matrix(n,n) × Matrix(n,n) → Matrix(n,n)

---

### 4. QUANTUM VALIDATOR MODULE

**Responsibility**: Enforce quantum mechanical constraints

**Public API**:
```rust
pub struct QuantumValidator {
    tolerance: f64,
    strict_mode: bool,
}

impl QuantumValidator {
    pub fn validate(&mut self, ast: &TypedAst) -> Result<ValidatedAst, ValidationError>;
    pub fn check_hermitian(&self, mat: &Array2<C64>) -> Result<(), ValidationError>;
    pub fn check_psd(&self, mat: &Array2<C64>) -> Result<(), ValidationError>;
    pub fn check_unitary(&self, mat: &Array2<C64>) -> Result<(), ValidationError>;
    pub fn check_trace_one(&self, mat: &Array2<C64>) -> Result<(), ValidationError>;
    pub fn check_cptp(&self, kraus_ops: &[Array2<C64>]) -> Result<(), ValidationError>;
    pub fn check_povm(&self, effects: &[Array2<C64>]) -> Result<(), ValidationError>;
}

pub struct ValidationWarning {
    pub kind: WarningKind,
    pub severity: Severity,
    pub message: String,
    pub span: Span,
}

pub enum WarningKind {
    TraceDrift(f64),       // |Tr(ρ) - 1| = δ
    Negativity(f64),       // min(eigenvalues) = -ε
    NonHermiticity(f64),   // ||H - H†|| = δ
    UnitarityViolation(f64), // ||U†U - I|| = δ
}
```

**Validation Checks**:
- Hamiltonians: `||H - H†|| < tol`
- Density matrices: `eigenvalues ≥ -tol`, `|Tr(ρ) - 1| < tol`
- Unitaries: `||U†U - I|| < tol`
- POVMs: `Σ E_i = I`, `E_i ≥ 0`
- CPTP: Choi matrix is PSD and trace-preserving

---

### 5. PROVER MODULE (NEW)

**Responsibility**: Symbolic proof of identities and properties

**Public API**:
```rust
pub struct Prover {
    rewrite_rules: RewriteSystem,
    assumptions: AssumptionContext,
    proof_cache: ProofCache,
}

impl Prover {
    pub fn new() -> Self;
    pub fn add_assumption(&mut self, assumption: Assumption);
    pub fn prove_identity(&mut self, lhs: &Expr, rhs: &Expr) -> ProofResult;
    pub fn prove_property(&mut self, prop: Property) -> PropertyProof;
    pub fn find_counterexample(&self, lhs: &Expr, rhs: &Expr) -> Option<Counterexample>;
    pub fn verify_proof(&self, proof: &Proof) -> bool;
}

pub enum ProofResult {
    Proven(Proof),          // Successfully proved with trace
    Refuted(Counterexample), // Found counterexample
    Unknown(Reason),        // Cannot determine
}

pub struct Proof {
    pub statement: Statement,
    pub steps: Vec<ProofStep>,
    pub assumptions_used: Vec<Assumption>,
    pub certificate: Certificate,
}

pub struct ProofStep {
    pub rule: RewriteRule,
    pub before: Expr,
    pub after: Expr,
    pub justification: String,
}

pub enum Property {
    Hermitian(Expr),
    Unitary(Expr),
    PSD(Expr),
    TraceOne(Expr),
    CPTP(Vec<Expr>),
    Commutes(Expr, Expr),
    Idempotent(Expr),
}

pub struct PropertyProof {
    pub property: Property,
    pub result: PropertyResult,
    pub certificate: Option<Certificate>,
}

pub enum PropertyResult {
    SymbolicProof(Vec<ProofStep>),  // Proven by construction
    NumericCertificate(NumericProof), // Verified at bound parameters
    Failed(Reason),
}

pub struct Certificate {
    pub hash: String,
    pub timestamp: DateTime<Utc>,
    pub engine_version: String,
    pub assumptions: Vec<Assumption>,
    pub verification_steps: Vec<VerificationStep>,
}
```

**Rewrite Rules** (built-in):
```rust
pub enum RewriteRule {
    // Dagger properties
    DaggerDagger,        // (A†)† = A
    DaggerSum,           // (A + B)† = A† + B†
    DaggerProduct,       // (AB)† = B†A†
    DaggerScalar,        // (cA)† = c* A†
    
    // Trace properties
    TraceCyclic,         // Tr(ABC) = Tr(CAB) = Tr(BCA)
    TraceLinear,         // Tr(A + B) = Tr(A) + Tr(B)
    TraceScalar,         // Tr(cA) = c Tr(A)
    
    // Commutator properties
    CommutatorSelf,      // [A, A] = 0
    CommutatorAnti,      // [A, B] = -[B, A]
    CommutatorLinear,    // [A, B + C] = [A, B] + [A, C]
    JacobiIdentity,      // [A, [B, C]] + [B, [C, A]] + [C, [A, B]] = 0
    
    // Tensor product properties
    TensorDistribute,    // (A+B) ⊗ C = A⊗C + B⊗C
    TensorAssoc,         // (A⊗B)⊗C = A⊗(B⊗C)
    TensorDagger,        // (A⊗B)† = A†⊗B†
    
    // Pauli algebra
    PauliSquare,         // σ_i² = I
    PauliCommutator,     // [σ_i, σ_j] = 2i ε_ijk σ_k
    PauliAnticommutator, // {σ_i, σ_j} = 2δ_ij I
    
    // Simplification
    MultiplyZero,        // 0 * A = 0
    AddZero,             // A + 0 = A
    MultiplyIdentity,    // I * A = A
    
    // Custom rules (user-defined)
    Custom(Box<dyn Fn(&Expr) -> Option<Expr>>),
}
```

**Canonicalization**:
```rust
pub fn canonicalize(expr: &Expr) -> Expr {
    // 1. Flatten nested sums/products
    // 2. Sort commutative operations (by hash)
    // 3. Apply idempotent rules (σ² = I)
    // 4. Normalize coefficients
    // 5. Simplify trivial expressions (0*A, A+0)
}
```

**Proof Search Strategy**:
1. Canonicalize both LHS and RHS
2. If equal → trivially proven
3. Apply rewrite rules breadth-first from LHS
4. Check if any intermediate form matches canonical RHS
5. Timeout after N steps or depth D
6. If no proof found, attempt counterexample via random sampling

---

### 6. OPTIMIZER MODULE

**Responsibility**: AST/IR optimization for performance

**Public API**:
```rust
pub struct Optimizer {
    passes: Vec<Box<dyn OptimizationPass>>,
    stats: OptimizationStats,
}

impl Optimizer {
    pub fn optimize(&mut self, ast: &ValidatedAst) -> Result<OptimizedAst>;
    pub fn optimize_ir(&mut self, ir: &IrProgram) -> Result<IrProgram>;
}

pub trait OptimizationPass {
    fn name(&self) -> &str;
    fn run(&self, ast: &Ast) -> Result<Ast>;
}

// Built-in passes
pub struct ConstantFolding;
pub struct CommonSubexprElimination;
pub struct DeadCodeElimination;
pub struct AlgebraicSimplification;
pub struct SparsityDetection;
```

**Optimization Passes**:
- **Constant Folding**: Evaluate pure expressions at compile time
- **CSE**: Deduplicate repeated subexpressions
- **Dead Code Elimination**: Remove unused symbols
- **Algebraic Simplification**: Apply rewrite rules for simplification
- **Sparsity Detection**: Tag sparse matrices for specialized kernels

---

### 7. IR MODULE

**Responsibility**: Backend-agnostic intermediate representation

**Public API**:
```rust
pub struct IrProgram {
    pub nodes: Vec<IrNode>,
    pub experiments: Vec<IrExperiment>,
    pub proofs: Vec<IrProofRequest>,
    pub metadata: IrMetadata,
}

pub enum IrNode {
    // Data loading
    LoadMatrix { id: NodeId, data: Array2<C64> },
    LoadVector { id: NodeId, data: Array1<C64> },
    LoadScalar { id: NodeId, value: C64 },
    
    // Binary operations
    MatrixAdd { id: NodeId, lhs: NodeId, rhs: NodeId },
    MatrixMul { id: NodeId, lhs: NodeId, rhs: NodeId },
    ScalarMul { id: NodeId, scalar: NodeId, matrix: NodeId },
    TensorProduct { id: NodeId, lhs: NodeId, rhs: NodeId },
    Commutator { id: NodeId, lhs: NodeId, rhs: NodeId },
    Anticommutator { id: NodeId, lhs: NodeId, rhs: NodeId },
    
    // Unary operations
    Dagger { id: NodeId, operand: NodeId },
    Trace { id: NodeId, operand: NodeId },
    MatrixExp { id: NodeId, operand: NodeId },
    
    // Evolution
    UnitaryPropagator { id: NodeId, hamiltonian: NodeId, time: f64 },
    ApplyUnitaryKet { id: NodeId, unitary: NodeId, ket: NodeId },
    ApplyUnitaryRho { id: NodeId, unitary: NodeId, rho: NodeId },
    IntegrateLindblad { id: NodeId, hamiltonian: NodeId, lindblad_ops: Vec<(NodeId, f64)>, 
                        initial_rho: NodeId, times: Vec<f64> },
    
    // Measurement
    MeasureProjective { id: NodeId, rho: NodeId, projectors: Vec<NodeId> },
    MeasurePovm { id: NodeId, rho: NodeId, effects: Vec<NodeId> },
    MeasureExpectation { id: NodeId, rho: NodeId, observable: NodeId },
    
    // Metadata
    Tag { id: NodeId, operand: NodeId, tag: String },  // For debugging/profiling
}

pub struct IrProofRequest {
    pub kind: ProofKind,
    pub expr: NodeId,
    pub expected: Option<NodeId>,  // For identity proofs
}

pub enum ProofKind {
    Identity,
    Hermitian,
    Unitary,
    PSD,
    CPTP,
}
```

---

### 8. EXECUTOR MODULE

**Responsibility**: Execute IR on selected backend

**Public API**:
```rust
pub struct Executor {
    backend: Box<dyn Backend>,
    cache: ExecutionCache,
    diagnostics: Diagnostics,
}

impl Executor {
    pub fn new(config: BackendConfig) -> Self;
    pub fn execute(&mut self, ir: &IrProgram) -> Result<ExecutionResult>;
    pub fn execute_experiment(&mut self, exp: &IrExperiment) -> Result<ExperimentResult>;
    pub fn get_diagnostics(&self) -> &Diagnostics;
}

pub trait Backend {
    fn name(&self) -> &str;
    fn supports_sparse(&self) -> bool;
    fn execute_node(&self, node: &IrNode, cache: &Cache) -> Result<NodeValue>;
}

pub struct CpuDenseBackend;  // Existing
pub struct CpuSparseBackend; // TODO
pub struct GpuBackend;       // TODO

pub struct Diagnostics {
    pub trace_drift: Vec<(f64, f64)>,  // (time, drift)
    pub min_eigenvalue: Vec<(f64, f64)>,
    pub norm_drift: Vec<(f64, f64)>,
    pub execution_time: HashMap<String, Duration>,
}
```

---

### 9. MEASUREMENT ENGINE MODULE

**Responsibility**: Compute measurement probabilities and sample outcomes

**Public API**:
```rust
pub struct MeasurementEngine {
    rng: StdRng,
}

impl MeasurementEngine {
    pub fn new(seed: u64) -> Self;
    pub fn measure_projective(&mut self, rho: &Array2<C64>, projectors: &[Array2<C64>], 
                              n_shots: usize) -> Result<MeasurementOutcome>;
    pub fn measure_povm(&mut self, rho: &Array2<C64>, effects: &[Array2<C64>], 
                        n_shots: usize) -> Result<MeasurementOutcome>;
    pub fn expectation_value(&self, rho: &Array2<C64>, observable: &Array2<C64>) 
                            -> Result<f64>;
}

pub struct MeasurementOutcome {
    pub probabilities: Vec<f64>,
    pub outcomes: Vec<usize>,  // Sampled outcomes (length = n_shots)
    pub counts: Vec<usize>,    // Histogram
    pub expectation: f64,      // Weighted average
    pub variance: f64,         // Shot noise
}
```

---

### 10. STATISTICS MODULE

**Responsibility**: Parameter fitting and hypothesis testing

**Public API**:
```rust
pub struct StatisticsEngine {
    optimizer: Box<dyn Optimizer>,
}

impl StatisticsEngine {
    pub fn fit_mle(&self, model: &IrProgram, data: &MeasurementData, 
                   initial_guess: &[f64]) -> Result<FitResult>;
    pub fn bootstrap_ci(&self, model: &IrProgram, data: &MeasurementData,
                        params: &[f64], n_bootstrap: usize) -> Result<ConfidenceIntervals>;
    pub fn likelihood_ratio_test(&self, model_null: &IrProgram, model_alt: &IrProgram,
                                 data: &MeasurementData) -> Result<LRTestResult>;
    pub fn model_comparison(&self, models: &[IrProgram], data: &MeasurementData) 
                           -> Result<ModelComparison>;
}

pub struct FitResult {
    pub best_params: Vec<f64>,
    pub log_likelihood: f64,
    pub hessian: Option<Array2<f64>>,  // For Fisher information
    pub uncertainties: Vec<f64>,        // From Fisher info or bootstrap
    pub convergence: ConvergenceInfo,
}

pub struct TestResult {
    pub chi_square: f64,
    pub dof: usize,
    pub p_value: f64,
    pub decision: Decision,  // Accept or Reject
    pub residuals: Residuals,
}

pub enum Decision {
    Accept { confidence: f64 },
    Reject { reason: String },
    Inconclusive { reason: String },
}
```

---

### 11. JOB QUEUE MODULE (NEW)

**Responsibility**: Manage batch execution and parameter sweeps

**Public API**:
```rust
pub struct JobQueue {
    jobs: VecDeque<Job>,
    workers: Vec<Worker>,
    results: HashMap<JobId, JobResult>,
}

impl JobQueue {
    pub fn new(n_workers: usize) -> Self;
    pub fn submit(&mut self, job: Job) -> JobId;
    pub fn submit_sweep(&mut self, base_job: Job, params: ParameterGrid) -> Vec<JobId>;
    pub fn status(&self, job_id: JobId) -> JobStatus;
    pub fn cancel(&mut self, job_id: JobId) -> Result<()>;
    pub fn get_result(&self, job_id: JobId) -> Option<&JobResult>;
    pub fn aggregate_sweep(&self, job_ids: &[JobId]) -> Result<SweepResult>;
}

pub struct Job {
    pub id: JobId,
    pub kind: JobKind,
    pub priority: Priority,
    pub params: HashMap<String, f64>,
}

pub enum JobKind {
    Simulate(IrProgram),
    Prove(IrProofRequest),
    Fit { program: IrProgram, data: MeasurementData },
    Test { program: IrProgram, data: MeasurementData },
}

pub enum JobStatus {
    Queued,
    Running { progress: f64 },
    Complete(JobResult),
    Failed(Error),
    Cancelled,
}
```

---

### 12. STREAMING MODULE (NEW)

**Responsibility**: Live data ingestion and real-time fitting

**Public API**:
```rust
pub struct StreamCoordinator {
    sources: Vec<Box<dyn DataSource>>,
    model: IrProgram,
    rolling_fit: RollingFit,
}

impl StreamCoordinator {
    pub fn new(model: IrProgram, config: StreamConfig) -> Self;
    pub fn add_source(&mut self, source: Box<dyn DataSource>);
    pub fn start(&mut self) -> Result<()>;
    pub fn stop(&mut self);
    pub fn get_current_fit(&self) -> &FitResult;
}

pub trait DataSource: Send {
    fn poll(&mut self) -> Result<Option<MeasurementEvent>>;
}

pub struct FileWatchSource {
    path: PathBuf,
    position: u64,
}

pub struct WebSocketSource {
    url: String,
    client: WebSocketClient,
}

pub struct RollingFit {
    window_size: usize,
    buffer: CircularBuffer<MeasurementEvent>,
    current_fit: FitResult,
}
```

---

### 13. TEMPLATE MANAGER MODULE (NEW)

**Responsibility**: Pre-built experiment templates

**Public API**:
```rust
pub struct TemplateManager {
    templates: HashMap<String, Template>,
}

impl TemplateManager {
    pub fn new() -> Self;
    pub fn get(&self, name: &str) -> Option<&Template>;
    pub fn instantiate(&self, name: &str, params: &HashMap<String, f64>) -> Result<Ast>;
    pub fn list_templates(&self) -> Vec<TemplateInfo>;
}

pub struct Template {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ParameterDef>,
    pub dsl_template: String,  // With {{param}} placeholders
}

pub struct ParameterDef {
    pub name: String,
    pub description: String,
    pub default_value: Option<f64>,
    pub range: Option<(f64, f64)>,
    pub units: Option<String>,
}
```

**Built-in Templates**:
- `rabi`: Two-level system driven oscillations
- `ramsey`: Free precession interferometry
- `spin_echo`: Refocusing pulse sequence
- `t1_decay`: Amplitude damping (energy relaxation)
- `t2_decay`: Phase damping (dephasing)
- `bell_state`: Two-qubit entanglement
- `amplitude_damping`: Open system dissipation
- `phase_damping`: Pure dephasing channel

---

### 14. MANIFEST MODULE

**Responsibility**: Reproducibility and provenance tracking

**Public API**:
```rust
pub struct ManifestGenerator {
    git_repo: Option<GitRepo>,
}

impl ManifestGenerator {
    pub fn generate(&self, run: &ExecutionResult, proofs: &[Proof]) -> Manifest;
    pub fn verify(&self, manifest: &Manifest, result: &ExecutionResult) -> VerificationResult;
}

pub struct Manifest {
    pub version: String,
    pub run_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub engine_version: String,
    pub git_commit: Option<String>,
    pub dsl_file: FileHash,
    pub parameters: HashMap<String, f64>,
    pub backend_config: BackendConfig,
    pub solver_config: SolverConfig,
    pub random_seed: u64,
    pub system_info: SystemInfo,
    pub data_files: Vec<FileHash>,
    pub output_files: Vec<FileHash>,
    pub proofs: Vec<ProofReference>,
    pub execution_time: Duration,
    pub diagnostics_summary: DiagnosticsSummary,
}

pub struct FileHash {
    pub path: PathBuf,
    pub sha256: String,
    pub size_bytes: u64,
}

pub struct ProofReference {
    pub statement: String,
    pub certificate_hash: String,
    pub result: ProofResultSummary,
}
```

---

## Data Flow

### Compilation + Proving Flow

```
DSL Text
  │
  ▼
[Parser] ──────────────────► AST (with spans)
  │
  ▼
[Name Resolver] ────────────► ResolvedAst (symbol table)
  │
  ▼
[Type Checker] ─────────────► TypedAst (shapes inferred)
  │
  ├─────────────────► [Prover] ──────────► Proof / Certificate
  │                     │                       │
  │                     ▼                       ▼
  │             Rewrite + Search           (save to disk)
  │                     │
  ▼                     ▼
[Quantum Validator] ◄─── (use proven properties)
  │
  ▼
[Optimizer] ────────────────► OptimizedAst
  │
  ▼
[IR Lowering] ──────────────► IrProgram
  │
  ▼
[Backend Selection] ────────► Configured Executor
  │
  ▼
[Execute] ──────────────────► ExecutionResult
  │
  ▼
[Manifest Generator] ───────► manifest.json
```

### Fitting Flow

```
ExecutionResult + CSV Data
  │
  ▼
[MeasurementData Parser] ──► Structured Data
  │
  ▼
[Statistics Engine]
  │
  ├─► [MLE Optimizer] ──────► Best-fit parameters
  │     │
  │     └─► (re-run simulation with trial params)
  │
  ├─► [Bootstrap] ───────────► Confidence Intervals
  │
  ├─► [Likelihood Ratio Test] ► Hypothesis Decision
  │
  ▼
[Report Generator] ─────────► JSON/PDF Report
```

### Live Streaming Flow

```
Hardware/File
  │
  ▼
[DataSource] (poll loop)
  │
  ▼
[RollingFit Buffer] ────────► Circular buffer (window)
  │
  ▼
[Incremental MLE] ──────────► Updated fit params
  │
  ├─► [Drift Detection] ────► Alerts if params shift
  │
  ▼
[Real-time Dashboard] ──────► WebSocket → UI
```

---

## Error Handling Strategy

### Error Types

```rust
pub enum EngineError {
    Parse(ParseError),
    NameResolution(NameError),
    Type(TypeError),
    Validation(ValidationError),
    Proof(ProofError),
    Execution(ExecutionError),
    IO(IoError),
    Internal(InternalError),
}

pub struct ParseError {
    pub span: Span,
    pub expected: Vec<String>,
    pub found: String,
    pub fix_hint: Option<String>,
}

pub struct TypeError {
    pub span: Span,
    pub expected: Type,
    pub found: Type,
    pub context: String,
    pub fix_hint: Option<String>,
}

pub struct ProofError {
    pub kind: ProofErrorKind,
    pub statement: String,
    pub reason: String,
    pub counterexample: Option<Counterexample>,
}

pub enum ProofErrorKind {
    Timeout,
    InsufficientAssumptions,
    Refuted,
    Internal,
}
```

### Error Codes

All errors have unique codes for documentation lookup:
- `E0001-E0999`: Parse errors
- `E1000-E1999`: Type errors
- `E2000-E2999`: Validation errors
- `E3000-E3999`: Proof errors
- `E4000-E4999`: Execution errors

### Fix Hints

Errors include actionable suggestions:
```
error[E1042]: dimension mismatch in matrix multiplication
  --> model.phys:12:18
   |
12 |   let result = A * B;
   |                ^ cannot multiply 2×3 matrix by 2×2 matrix
   |
   = note: matrix multiplication requires A.cols == B.rows
   = help: did you mean to transpose B? Try `A * transpose(B)`
```

---

See [DSL_SPEC.md](DSL_SPEC.md) for complete grammar and language specification.
