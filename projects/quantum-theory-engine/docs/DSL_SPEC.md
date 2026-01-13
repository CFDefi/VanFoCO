# DSL Specification: Quantum Theory Language with Proof Support

## Complete EBNF Grammar

```ebnf
(* ========== TOP-LEVEL STRUCTURE ========== *)

program         = { statement } ;

statement       = constant_decl
                | matrix_decl
                | vector_decl
                | hamiltonian_decl
                | measurement_decl
                | experiment_decl
                | assume_block
                | prove_stmt
                | show_stmt
                | function_def
                | comment ;

(* ========== DECLARATIONS ========== *)

constant_decl   = "const", identifier, "=", expr, ";" ;

matrix_decl     = "matrix", identifier, "=", matrix_literal, ";" ;

vector_decl     = "vector", identifier, "=", vector_literal, ";" ;

hamiltonian_decl = "Hamiltonian", identifier, "=", expr, ";" ;

measurement_decl = "measure", identifier, ":", measurement_type, ";" ;

measurement_type = "Projective", "(", "[", matrix_list, "]", ")"
                 | "POVM", "(", "[", matrix_list, "]", ")" ;

experiment_decl  = "experiment", identifier, "{",
                      "init", ":", state_spec, ";",
                      "evolution", ":", evolution_spec, ";",
                      "measurements", ":", "[", measurement_schedule, "]", ";",
                      [ "shots", ":", integer, ";" ],
                   "}" ;

function_def    = "fn", identifier, "(", param_list, ")", "->", type_spec, "{",
                     { statement },
                     "return", expr, ";",
                  "}" ;

(* ========== PROOF CONSTRUCTS ========== *)

assume_block    = "assume", "{", { assumption }, "}" ;

assumption      = identifier, "is", property_kind, ";"
                | expr, relop, expr, ";" ;

property_kind   = "real" | "complex" | "Hermitian" | "unitary" | "PSD" 
                | "positive" | "trace_one" ;

prove_stmt      = "prove", proof_goal, [ "using", identifier_list ], ";" ;

proof_goal      = identity_goal | property_goal ;

identity_goal   = expr, "==", expr ;

property_goal   = property_kind, "(", expr, ")" ;

show_stmt       = "show", expr, [ "as", show_format ], ";" ;

show_format     = "canonical" | "expanded" | "factored" | "matrix" ;

(* ========== EXPRESSIONS ========== *)

expr            = sum_expr ;

sum_expr        = product_expr, { ("+" | "-"), product_expr } ;

product_expr    = unary_expr, { ("*" | "/"), unary_expr } ;

unary_expr      = [ "-" | "+" ], power_expr ;

power_expr      = postfix_expr, [ "^", power_expr ] ;

postfix_expr    = primary_expr, { "†" | "'", } ;  (* dagger *)

primary_expr    = literal
                | identifier
                | builtin_function
                | function_call
                | "(", expr, ")"
                | matrix_literal
                | vector_literal ;

(* ========== BUILT-IN FUNCTIONS ========== *)

builtin_function = tensor_product
                 | dagger
                 | trace_op
                 | commutator
                 | anticommutator
                 | matrix_exp
                 | sin_cos_exp
                 | sqrt_op
                 | eigenvalues
                 | eigenvectors
                 | det
                 | transpose ;

tensor_product  = "tensor", "(", expr, ",", expr, ")" ;

dagger          = "dagger", "(", expr, ")" ;

trace_op        = "trace", "(", expr, ")" ;

commutator      = "commutator", "(", expr, ",", expr, ")"
                | "[", expr, ",", expr, "]" ;

anticommutator  = "anticommutator", "(", expr, ",", expr, ")"
                | "{", expr, ",", expr, "}" ;

matrix_exp      = "exp", "(", expr, ")" | "matrix_exp", "(", expr, ")" ;

sin_cos_exp     = ( "sin" | "cos" | "exp" | "log" | "sqrt" ), "(", expr, ")" ;

eigenvalues     = "eigenvalues", "(", expr, ")" ;

eigenvectors    = "eigenvectors", "(", expr, ")" ;

det             = "det", "(", expr, ")" ;

transpose       = "transpose", "(", expr, ")" | "T", "(", expr, ")" ;

function_call   = identifier, "(", [ expr_list ], ")" ;

(* ========== TIME-DEPENDENT FUNCTIONS ========== *)

time_function   = identifier, "(", "t", ")" ;

(* ========== EVOLUTION SPECIFICATIONS ========== *)

state_spec      = ket_spec | rho_spec ;

ket_spec        = "ket", "(", vector_literal, ")" ;

rho_spec        = "rho", "(", matrix_literal, ")" 
                | "pure", "(", vector_literal, ")" ;  (* |ψ⟩⟨ψ| *)

evolution_spec  = schrodinger_evol | lindblad_evol ;

schrodinger_evol = "evolve", "(", state_spec, ",", identifier, ",", timegrid, ")" ;

lindblad_evol    = "lindblad", "(", state_spec, ",", identifier, ",", 
                      lindblad_ops, ",", timegrid, ")" ;

lindblad_ops    = "[", { lindblad_term, "," }, "]" ;

lindblad_term   = "(", expr, ",", number, ")" ;  (* (L_i, γ_i) *)

timegrid        = "timegrid", "=", "(", number, ",", number, ",", integer, ")"
                | "times", "=", "[", number_list, "]" ;

measurement_schedule = { measurement_event, "," } ;

measurement_event    = "(", number, ",", identifier, ")" ;  (* (time, measurement_id) *)

(* ========== LITERALS ========== *)

matrix_literal  = "[", { row, "," }, "]" ;

row             = "[", expr_list, "]" ;

vector_literal  = "vec", "(", expr_list, ")" 
                | "[", expr_list, "]" ;

(* ========== BUILT-IN CONSTANTS ========== *)

builtin_const   = "I" | "identity"      (* Identity matrix *)
                | "sigma_x" | "σ_x"     (* Pauli X *)
                | "sigma_y" | "σ_y"     (* Pauli Y *)
                | "sigma_z" | "σ_z"     (* Pauli Z *)
                | "sigma_plus" | "σ_+" (* Raising operator *)
                | "sigma_minus" | "σ_-" (* Lowering operator *)
                | "pi" | "π"
                | "e"
                | "i" ;                 (* Imaginary unit *)

(* ========== PRIMITIVES ========== *)

identifier      = letter, { letter | digit | "_" } ;

number          = real_number | complex_number ;

real_number     = [ "-" ], digit, { digit }, [ ".", { digit } ], 
                  [ ( "e" | "E" ), [ "+" | "-" ], digit, { digit } ] ;

complex_number  = real_number, [ ( "+" | "-" ), real_number ], "i" ;

integer         = digit, { digit } ;

letter          = "a" .. "z" | "A" .. "Z" ;

digit           = "0" .. "9" ;

relop           = "==" | "!=" | "<" | ">" | "<=" | ">=" ;

comment         = "//", { any_char - newline }, newline
                | "/*", { any_char }, "*/" ;

(* ========== LISTS ========== *)

expr_list       = expr, { ",", expr } ;

matrix_list     = matrix_literal, { ",", matrix_literal } ;

number_list     = number, { ",", number } ;

identifier_list = identifier, { ",", identifier } ;

param_list      = identifier, ":", type_spec, { ",", identifier, ":", type_spec } ;

(* ========== TYPE ANNOTATIONS ========== *)

type_spec       = "Scalar" | "Real" | "Complex"
                | "Vector", "<", integer, ">"
                | "Matrix", "<", integer, ",", integer, ">"
                | "Operator", "<", integer, ">"
                | "Density", "<", integer, ">"
                | "Unitary", "<", integer, ">" ;
```

---

## AST Node Types

### Core Nodes

```rust
pub struct Ast {
    pub statements: Vec<Statement>,
    pub span: Span,
}

pub enum Statement {
    ConstDecl { name: String, value: Expr, span: Span },
    MatrixDecl { name: String, value: MatrixLiteral, span: Span },
    VectorDecl { name: String, value: VectorLiteral, span: Span },
    HamiltonianDecl { name: String, value: Expr, span: Span },
    MeasurementDecl { name: String, kind: MeasurementKind, span: Span },
    ExperimentDecl { name: String, body: ExperimentBody, span: Span },
    AssumeBlock { assumptions: Vec<Assumption>, span: Span },
    ProveStmt { goal: ProofGoal, using: Vec<String>, span: Span },
    ShowStmt { expr: Expr, format: Option<ShowFormat>, span: Span },
    FunctionDef { name: String, params: Vec<Parameter>, return_type: Type, 
                  body: Vec<Statement>, return_expr: Expr, span: Span },
}

pub enum Expr {
    // Literals
    Number(Complex64, Span),
    Identifier(String, Span),
    MatrixLit(MatrixLiteral, Span),
    VectorLit(VectorLiteral, Span),
    
    // Binary operations
    Add(Box<Expr>, Box<Expr>, Span),
    Sub(Box<Expr>, Box<Expr>, Span),
    Mul(Box<Expr>, Box<Expr>, Span),
    Div(Box<Expr>, Box<Expr>, Span),
    Pow(Box<Expr>, Box<Expr>, Span),
    
    // Unary operations
    Neg(Box<Expr>, Span),
    Dagger(Box<Expr>, Span),
    
    // Built-in functions
    Tensor(Box<Expr>, Box<Expr>, Span),
    Trace(Box<Expr>, Span),
    Commutator(Box<Expr>, Box<Expr>, Span),
    Anticommutator(Box<Expr>, Box<Expr>, Span),
    MatrixExp(Box<Expr>, Span),
    Sin(Box<Expr>, Span),
    Cos(Box<Expr>, Span),
    Sqrt(Box<Expr>, Span),
    Eigenvalues(Box<Expr>, Span),
    Eigenvectors(Box<Expr>, Span),
    Det(Box<Expr>, Span),
    Transpose(Box<Expr>, Span),
    
    // Function call
    Call { name: String, args: Vec<Expr>, span: Span },
    
    // Time-dependent
    TimeFunction { name: String, time_var: String, span: Span },
}

pub struct MatrixLiteral {
    pub rows: Vec<Vec<Expr>>,
    pub span: Span,
}

pub struct VectorLiteral {
    pub elements: Vec<Expr>,
    pub span: Span,
}

pub enum MeasurementKind {
    Projective(Vec<MatrixLiteral>),
    Povm(Vec<MatrixLiteral>),
}

pub struct ExperimentBody {
    pub init: StateSpec,
    pub evolution: EvolutionSpec,
    pub measurements: Vec<(f64, String)>,  // (time, measurement_id)
    pub shots: Option<usize>,
}

pub enum StateSpec {
    Ket(VectorLiteral),
    Rho(MatrixLiteral),
    Pure(VectorLiteral),  // Converts to |ψ⟩⟨ψ|
}

pub enum EvolutionSpec {
    Schrodinger { 
        initial: StateSpec, 
        hamiltonian: String, 
        times: TimeGrid 
    },
    Lindblad { 
        initial: StateSpec, 
        hamiltonian: String, 
        lindblad_ops: Vec<(Expr, f64)>,  // (L_i, γ_i)
        times: TimeGrid 
    },
}

pub enum TimeGrid {
    Regular { t0: f64, dt: f64, n_steps: usize },
    Explicit(Vec<f64>),
}

pub struct Assumption {
    pub kind: AssumptionKind,
    pub span: Span,
}

pub enum AssumptionKind {
    TypeProperty { name: String, property: PropertyKind },
    Relation { lhs: Expr, op: RelOp, rhs: Expr },
}

pub enum PropertyKind {
    Real,
    Complex,
    Hermitian,
    Unitary,
    PSD,
    Positive,
    TraceOne,
}

pub enum ProofGoal {
    Identity { lhs: Expr, rhs: Expr },
    Property { kind: PropertyKind, expr: Expr },
}

pub enum ShowFormat {
    Canonical,
    Expanded,
    Factored,
    Matrix,
}

pub enum RelOp {
    Eq, Ne, Lt, Gt, Le, Ge,
}

pub struct Parameter {
    pub name: String,
    pub ty: Type,
}

pub struct Span {
    pub start: usize,
    pub end: usize,
    pub file: Option<String>,
}
```

---

## Type System

### Type Hierarchy

```
Type
├── Scalar
│   ├── Real
│   └── Complex
├── Vector<n: usize>
├── Matrix<m: usize, n: usize>
├── Operator<d: usize>      // Hermitian operator on d-dim Hilbert space
├── Density<d: usize>       // Density matrix (PSD, Tr=1)
├── Unitary<d: usize>       // Unitary operator
├── Channel<d: usize>       // CPTP map (represented as Kraus operators or Choi matrix)
└── Measurement<n_outcomes: usize, d: usize>  // POVM or Projective
```

### Type Inference Rules

| Expression | Type Rule | Result Type |
|------------|-----------|-------------|
| `a + b` | `T + T → T` | `T` |
| `a - b` | `T - T → T` | `T` |
| `c * M` | `Scalar * Matrix<m,n> → Matrix<m,n>` | Broadcast |
| `A * B` | `Matrix<m,k> * Matrix<k,n> → Matrix<m,n>` | Matmul |
| `A ⊗ B` | `Matrix<m,n> ⊗ Matrix<p,q> → Matrix<m*p,n*q>` | Tensor |
| `A†` | `Matrix<m,n>† → Matrix<n,m>` | Conjugate transpose |
| `Tr(A)` | `Tr: Matrix<n,n> → Scalar` | Trace |
| `[A,B]` | `Matrix<n,n> × Matrix<n,n> → Matrix<n,n>` | Commutator |
| `exp(A)` | `Matrix<n,n> → Matrix<n,n>` | Matrix exponential |

### Constraints

- **Hermitian**: `A = A†`
- **Unitary**: `U†U = I`
- **PSD**: All eigenvalues ≥ 0
- **Trace One**: `Tr(ρ) = 1`
- **CPTP**: Channel Φ is completely positive and trace-preserving

---

## Semantic Rules

### Assumption Propagation

Assumptions declared in `assume` blocks are available to:
1. **Validator**: Skip certain checks if symbolically proven
2. **Prover**: Use as axioms in proof search
3. **Optimizer**: Enable optimizations (e.g., if `H` is Hermitian, `exp(-iHt)` is unitary)

Example:
```
assume {
    H is Hermitian;
    omega is real;
}

// Now validator doesn't need to numerically check H†=H
// Prover can use this in derivations
```

### Proof Goals

#### Identity Proofs

```
prove [H, I] == 0;
```

Prover will:
1. Canonicalize `[H, I]` → `HI - IH` → `H - H` → `0`
2. Generate proof trace with each step
3. Export certificate

#### Property Proofs

```
prove Hermitian(H);
```

Prover will attempt:
1. **Symbolic**: If `H = A + A†`, it's Hermitian by construction
2. **Numeric**: Compute `||H - H†||` at bound parameters
3. **Certificate**: Package proof with verification steps

### Show Commands

```
show dagger(A * B);  // Displays: B† * A†

show [sigma_x, sigma_y] as canonical;  // Displays: 2i * sigma_z
```

---

## Built-in Constants

| Symbol | Name | Dimension | Definition |
|--------|------|-----------|------------|
| `I`, `identity` | Identity | Context-dependent | Depends on operand |
| `σ_x`, `sigma_x` | Pauli X | 2×2 | `[[0,1],[1,0]]` |
| `σ_y`, `sigma_y` | Pauli Y | 2×2 | `[[0,-i],[i,0]]` |
| `σ_z`, `sigma_z` | Pauli Z | 2×2 | `[[1,0],[0,-1]]` |
| `σ_+`, `sigma_plus` | Raising | 2×2 | `[[0,1],[0,0]]` |
| `σ_-`, `sigma_minus` | Lowering | 2×2 | `[[0,0],[1,0]]` |
| `π`, `pi` | Pi | Scalar | 3.14159... |
| `e` | Euler's number | Scalar | 2.71828... |
| `i` | Imaginary unit | Scalar | √(-1) |

---

## Example Programs

### Example 1: Rabi with Proof

```
// Rabi oscillations with formal proof of commutation relation

const omega = 1.0;
const Omega = 0.2;

matrix sigma_x = [[0, 1], [1, 0]];
matrix sigma_y = [[0, -i], [i, 0]];
matrix sigma_z = [[1, 0], [0, -1]];

assume {
    omega is real;
    Omega is real;
}

Hamiltonian H = (omega/2) * sigma_z + Omega * sigma_x;

// Prove commutation relation
prove [sigma_x, sigma_y] == 2*i*sigma_z;

// Prove Hamiltonian is Hermitian
prove Hermitian(H);

// Show canonical form
show H as canonical;

experiment rabi {
    init: ket(vec(1, 0));
    evolution: evolve(init, H, timegrid=(0.0, 0.01, 501));
    measurements: [(0.0, z_basis), (5.0, z_basis)];
    shots: 1000;
}

measure z_basis: Projective([
    [[1, 0], [0, 0]],  // |0⟩⟨0|
    [[0, 0], [0, 1]]   // |1⟩⟨1|
]);
```

### Example 2: Identities

```
// Proof examples for common quantum identities

matrix sigma_x = [[0, 1], [1, 0]];
matrix sigma_y = [[0, -i], [i, 0]];
matrix sigma_z = [[1, 0], [0, -1]];

assume {
    sigma_x is Hermitian;
    sigma_y is Hermitian;
    sigma_z is Hermitian;
}

// Pauli algebra
prove sigma_x * sigma_x == I;
prove sigma_y * sigma_y == I;
prove sigma_z * sigma_z == I;

prove [sigma_x, sigma_y] == 2*i*sigma_z;
prove [sigma_y, sigma_z] == 2*i*sigma_x;
prove [sigma_z, sigma_x] == 2*i*sigma_y;

// Anticommutators
prove {sigma_x, sigma_y} == 0;
prove {sigma_x, sigma_z} == 0;
prove {sigma_y, sigma_z} == 0;

// Dagger properties
prove dagger(sigma_x) == sigma_x;
prove dagger(sigma_y) == sigma_y;
prove dagger(sigma_z) == sigma_z;

// Trace properties
prove trace(sigma_x) == 0;
prove trace(sigma_y) == 0;
prove trace(sigma_z) == 0;

// Tensor product commutativity (of trace)
prove trace(tensor(sigma_x, sigma_y)) == trace(sigma_x) * trace(sigma_y);
```

### Example 3: Property Certificates

```
// Generate property certificates for publication

const gamma = 0.1;

matrix sigma_minus = [[0, 0], [1, 0]];
matrix sigma_plus = [[0, 1], [0, 0]];
matrix sigma_z = [[1, 0], [0, -1]];

Hamiltonian H = 0.5 * sigma_z;

// Lindblad operator
matrix L = sqrt(gamma) * sigma_minus;

// Prove properties
prove Hermitian(H);
prove trace(dagger(L) * L) >= 0;  // Positivity

// Show that dissipator is CPTP
// This generates a certificate that can be verified independently
prove CPTP([L]);

experiment amp_damp {
    init: ket(vec(0, 1));  // Start in |1⟩
    evolution: lindblad(init, H, [(L, gamma)], timegrid=(0.0, 0.01, 501));
    measurements: [(0.0, z_basis), (5.0, z_basis)];
}

measure z_basis: Projective([
    [[1, 0], [0, 0]],
    [[0, 0], [0, 1]]
]);
```

---

## Next Steps

See [PROVER_DESIGN.md](PROVER_DESIGN.md) for symbolic prover implementation details.
