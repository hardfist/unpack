# CLAUDE.md - AI Assistant Guide for Unpack

This document provides comprehensive guidance for AI assistants working on the Unpack codebase. It covers the project structure, development workflows, conventions, and best practices.

## Table of Contents

1. [Project Overview](#project-overview)
2. [Codebase Structure](#codebase-structure)
3. [Technology Stack](#technology-stack)
4. [Architecture Patterns](#architecture-patterns)
5. [Development Workflows](#development-workflows)
6. [Code Conventions](#code-conventions)
7. [Testing & Benchmarking](#testing--benchmarking)
8. [Build & Deployment](#build--deployment)
9. [AI Assistant Guidelines](#ai-assistant-guidelines)

---

## Project Overview

**Unpack** is an experimental JavaScript/TypeScript bundler written in Rust, exploring next-generation bundler architectures. The project serves as a research platform for evaluating different architectural approaches to building high-performance web application bundlers.

- **Repository**: https://github.com/hardfist/unpack
- **License**: MIT
- **Language**: Rust (Nightly), TypeScript/JavaScript
- **Purpose**: Experimental bundler exploring advanced dependency resolution and compilation patterns

### Key Goals

- Evaluate different incremental computation approaches (Salsa, Turbo-tasks)
- Achieve high-performance module bundling through Rust
- Provide Node.js integration via NAPI bindings
- Explore plugin-based extensibility architecture
- Benchmark against existing build tools

---

## Codebase Structure

```
/home/user/unpack/
├── crates/                          # Rust workspace crates
│   ├── unpack_core/                 # Main bundler engine (~3,032 LOC)
│   │   ├── src/
│   │   │   ├── compiler/            # Compilation orchestration
│   │   │   ├── module/              # Module graph & dependency tracking
│   │   │   ├── chunk/               # Code-splitting & chunk generation
│   │   │   ├── dependency/          # Dependency resolution
│   │   │   ├── task/                # Async task scheduling
│   │   │   ├── memory_manager/      # Arena allocation & caching
│   │   │   ├── plugin/              # Plugin system
│   │   │   └── resolver/            # Module path resolution
│   │   └── benches/                 # Benchmarks
│   │
│   ├── node_binding/                # Node.js N-API bindings
│   │   ├── src/                     # Rust<->JS bridge
│   │   └── npm/                     # Platform-specific packages
│   │       ├── darwin-x64/
│   │       ├── linux-x64-gnu/
│   │       └── win32-x64-msvc/
│   │
│   ├── unpack_salsa/                # Salsa incremental computation impl
│   │   └── src/
│   │       ├── compiler.rs          # Salsa-based bundler
│   │       └── db/                  # Salsa tracked entities
│   │
│   └── unpack_turbo/                # Turbo-tasks based implementation
│       └── src/
│           ├── module.rs            # Module representation
│           ├── chunk/               # Chunk groups & chunking logic
│           └── module_graph/        # Module graph building
│
├── packages/                        # TypeScript/Node.js packages
│   └── unpack/                      # Main JS/TS API wrapper
│       ├── src/
│       │   └── index.ts             # Compiler class wrapper
│       └── tests/                   # Integration tests
│
├── benchmark/                       # Performance benchmarking
│   └── build-tools-performance/     # Cross-tool comparison suite
│
├── .github/workflows/               # CI/CD automation
│   ├── codspeed.yml                 # Performance benchmarking
│   ├── opencode.yml                 # Code analysis
│   └── summary.yml                  # AI issue summarization
│
├── Cargo.toml                       # Rust workspace manifest
├── Cargo.lock                       # Locked dependencies
├── package.json                     # Root npm package
├── pnpm-workspace.yaml              # pnpm monorepo config
├── rust-toolchain.toml              # Rust nightly specification
├── .cargo/config.toml               # Rust compiler flags
└── Agents.md                        # Agent usage guidelines
```

### Key Crates

| Crate | Purpose | Key Files |
|-------|---------|-----------|
| **unpack_core** | Main bundler engine with compilation pipeline | `compiler.rs`, `module_graph.rs`, `chunk_graph.rs` |
| **node_binding** | Node.js integration via NAPI | `js_compiler.rs`, `js_plugin.rs` |
| **unpack_salsa** | Salsa-based incremental compilation | `compiler.rs`, `db/module.rs` |
| **unpack_turbo** | Turbo-tasks based implementation | `module.rs`, `chunk/`, `module_graph/` |

---

## Technology Stack

### Core Technologies

- **Rust**: Nightly (channel: `nightly-2025-06-04`)
- **Edition**: 2024 (workspace), 2021 (core)
- **Async Runtime**: Tokio (multi-threaded, full features)
- **Node.js Integration**: NAPI v4
- **JavaScript**: TypeScript for Node.js wrapper and tests
- **Package Manager**: pnpm (monorepo)

### Major Dependencies

#### AST & Parsing
- **swc_core** (0.106.4) - High-performance JavaScript/TypeScript parser
- **rspack_sources** (0.4.8) - Source manipulation utilities

#### Compilation Frameworks
- **turbo-tasks** - Task execution system from Next.js (custom branch)
- **salsa** - Incremental computation framework (custom revision)
- **turbo-rcstr** - Reference-counted strings

#### Module Resolution
- **rspack_resolver** (0.2.0) - webpack-compatible resolver

#### Data Structures & Memory
- **im** (15.1.0) - Immutable collections
- **slotmap** (1.0.6) - Slot-based storage
- **dashmap** (6.0.1) - Concurrent hash map
- **petgraph** (0.8.2) - Graph algorithms
- **roaring** (0.10.10) - Bitmap operations
- **mimalloc** (0.1.43) - High-performance memory allocator
- **atomic_refcell** (0.1) - Thread-safe interior mutability

#### Async & Concurrency
- **tokio** - Async runtime (with `tokio_unstable` flag)
- **rayon** (1.10.0) - Data parallelism
- **crossbeam-channel** - Multi-producer channels
- **async-std** (1.13.0) - Async standard library

#### Error Handling
- **thiserror** (1.0.63) - Error derivation macros
- **miette** (7.2.0) - Fancy diagnostics
- **anyhow** (1.0.100) - Error handling

---

## Architecture Patterns

### Compilation Pipeline (unpack_core)

The main compilation flow follows this sequence:

```
Compiler.build()
  │
  ├─> scan() [ModuleScanner]
  │    └─> Build module graph with dependencies
  │
  ├─> link() [ChunkLinker]
  │    └─> Generate chunks from module graph
  │
  ├─> code_generation()
  │    └─> Parallel code generation via Rayon
  │
  ├─> create_chunk_asset()
  │    └─> Create chunk assets
  │
  └─> emit_assets()
       └─> Output final bundle files
```

### Module System

**Key Components:**

1. **ModuleGraph**: Central data structure tracking all modules and their dependencies
2. **NormalModule**: Represents a single module with source code and dependencies
3. **ModuleScanner**: Traverses entry points and builds the module graph
4. **DependencyCollector**: Parses AST to extract import/export statements

**Module Resolution:**
- Uses `rspack_resolver` for webpack-compatible resolution
- Supports CommonJS, ESM, and mixed module systems
- Custom resolver configuration via plugin hooks

### Chunk System

**Key Components:**

1. **ChunkGraph**: Graph structure representing chunk relationships
2. **ChunkLinker**: Links modules into chunks based on splitting rules
3. **Chunk**: Contains set of modules that should be bundled together

**Code Splitting:**
- Entry point splitting
- Dynamic import splitting
- Shared module optimization

### Plugin Architecture

**Plugin Hooks:**
- `this_compilation` - Called when compilation starts
- `resolve` - Module resolution interception
- `load` - Module loading interception

**Plugin Types:**
- Rust plugins (native performance)
- JavaScript plugins (via NAPI bridge)

**Example Plugin Flow:**
```rust
// Rust side
impl Plugin for MyPlugin {
    async fn this_compilation(&self, compilation: &Compilation) {
        // Hook implementation
    }
}

// JS side (via NAPI)
class MyJsPlugin {
    this_compilation(compilation) {
        // JavaScript hook implementation
    }
}
```

### Memory Management

**Strategies:**

1. **Arena Allocation**: Custom arena for graph structures to reduce allocation overhead
2. **String Interning**: Deduplication of frequently used strings via intern pool
3. **Reference Counting**: `Arc` and `AtomicRefCell` for shared ownership
4. **MiMalloc**: Custom allocator for improved heap performance

**Key Files:**
- `memory_manager/arena.rs` - Arena allocator
- `memory_manager/intern.rs` - String interning

### Parallelization

**Approaches:**

1. **Rayon**: Data parallelism for code generation
   - Parallel iteration over modules
   - Work-stealing thread pool

2. **Tokio**: Async task scheduling
   - Concurrent I/O operations
   - Async plugin execution

3. **DashMap**: Concurrent data structures
   - Thread-safe hash maps without global locks

### Incremental Compilation

Two experimental approaches:

#### 1. Salsa-based (unpack_salsa)
- Query-based incremental computation
- Automatic dependency tracking
- File watching for dev mode
- Memoization of expensive operations

#### 2. Turbo-tasks (unpack_turbo)
- Task-based incremental system from Next.js
- Fine-grained invalidation
- Persistent caching support

---

## Development Workflows

### Initial Setup

```bash
# Clone repository
git clone https://github.com/hardfist/unpack.git
cd unpack

# Install Rust nightly (automatically via rust-toolchain.toml)
# The project uses: nightly-2025-06-04

# Install Node.js dependencies
pnpm install

# Build Rust crates
cargo build

# Build Node.js bindings
cd crates/node_binding
pnpm build

# Run tests
pnpm test
```

### Development Branch Workflow

**Always work on feature branches:**
- Branch naming: `claude/claude-md-<session-id>`
- Never push directly to main
- Use `git push -u origin <branch-name>` for new branches

### Common Development Tasks

#### Adding a New Feature

1. **Explore the codebase** to understand related code
   ```bash
   # Use Explore agent for codebase analysis
   # See Agents.md for details
   ```

2. **Identify the relevant module**
   - Compiler features → `unpack_core/src/compiler/`
   - Module graph → `unpack_core/src/module/`
   - Chunking → `unpack_core/src/chunk/`
   - Plugins → `unpack_core/src/plugin/`

3. **Write tests first** (when appropriate)
   - Integration tests → `packages/unpack/tests/`
   - Rust unit tests → In the same file as implementation
   - Benchmarks → `crates/unpack_core/benches/`

4. **Implement the feature**
   - Follow existing patterns
   - Keep changes minimal and focused
   - Avoid over-engineering

5. **Run tests and benchmarks**
   ```bash
   cargo test
   cargo bench
   pnpm test
   ```

#### Modifying the Module Graph

**Key Files:**
- `unpack_core/src/module/module_graph.rs` - Core graph structure
- `unpack_core/src/module/module_scanner.rs` - Graph building logic
- `unpack_core/src/module/normal_module.rs` - Module representation

**Pattern:**
1. Read existing module graph code first
2. Understand the data structures (petgraph usage)
3. Make changes preserving graph invariants
4. Update related scanners/linkers if needed

#### Adding a Plugin Hook

**Steps:**
1. Define hook trait method in `plugin/plugin.rs`
2. Add hook call in appropriate compilation phase
3. Implement NAPI bridge in `node_binding/src/js_plugin.rs`
4. Update TypeScript types in `packages/unpack/src/index.ts`
5. Document hook behavior

#### Debugging Performance Issues

**Tools Available:**
1. **CodSpeed Benchmarks** - CI-based performance tracking
2. **Chrome Tracing** - `tracing-chrome` crate support
3. **Heaptrack** - Memory profiling (check .gitignore for traces)
4. **cargo bench** - Local benchmarking

**Process:**
```bash
# Run benchmarks
cargo bench --bench module_graph_bench

# Enable tracing (if implemented)
RUST_LOG=trace cargo run

# Profile with release build
cargo build --profile release-debug
# Use external profilers (perf, valgrind, etc.)
```

---

## Code Conventions

### Rust Code Style

**Formatting:**
- Use `rustfmt` (configured in project)
- Run before committing: `cargo fmt`

**Linting:**
- Clippy enabled with custom rules (see `clippy.toml`)
- Workspace lint: `new_without_default = "allow"`
- Run: `cargo clippy --all-targets`

**Naming:**
- Types: `PascalCase` (e.g., `ModuleGraph`, `ChunkLinker`)
- Functions/variables: `snake_case` (e.g., `build_module_graph`)
- Constants: `SCREAMING_SNAKE_CASE`
- Lifetimes: `'a`, `'b` (single letters)

**Error Handling:**
- Use `Result<T, E>` for recoverable errors
- Use `thiserror` for custom error types
- Use `anyhow` for application errors
- Prefer `?` operator over explicit match

**Async Patterns:**
```rust
// Good: Use async/await
async fn process_module(&self) -> Result<Module> {
    let content = self.load().await?;
    Ok(content)
}

// Avoid: Manual futures
fn process_module(&self) -> impl Future<Output = Result<Module>> {
    // Complex future composition
}
```

**Memory Safety:**
- Prefer owned types over references when possible
- Use `Arc` for shared ownership across threads
- Use `AtomicRefCell` for interior mutability in concurrent contexts
- Avoid `unsafe` unless absolutely necessary (document why)

### TypeScript/JavaScript Style

**Formatting:**
- Follow existing code style
- Use TypeScript for type safety
- Avoid `any` type

**Naming:**
- Classes: `PascalCase` (e.g., `Compiler`, `Plugin`)
- Functions/variables: `camelCase`
- Constants: `SCREAMING_SNAKE_CASE`

**Async Patterns:**
```typescript
// Good: async/await
async build() {
    const result = await this.compiler.build();
    return result;
}

// Avoid: Promise chains when async/await is clearer
```

### File Organization

**Rust:**
- One module per file in most cases
- Use `mod.rs` or `lib.rs` for module roots
- Group related functionality in subdirectories
- Keep files under 500 lines when possible

**Tests:**
- Unit tests in same file as implementation
- Integration tests in separate `tests/` directory
- Use `#[cfg(test)]` for test modules

**Example:**
```rust
// my_module.rs
pub struct MyStruct { }

impl MyStruct {
    pub fn new() -> Self { }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let instance = MyStruct::new();
        // assertions
    }
}
```

### Documentation

**Rust Doc Comments:**
```rust
/// Brief description of function/type
///
/// More detailed explanation if needed.
///
/// # Examples
///
/// ```
/// use unpack_core::Compiler;
/// let compiler = Compiler::new();
/// ```
///
/// # Panics
///
/// Documents when function might panic
///
/// # Errors
///
/// Documents error conditions
pub fn important_function() -> Result<()> {
    // implementation
}
```

**When to Document:**
- Public APIs (always)
- Complex algorithms (explain the approach)
- Non-obvious code (why, not what)
- Plugin hooks (behavior and parameters)

**When Not to Document:**
- Self-explanatory code
- Private implementation details (unless complex)
- Tests (test name should be descriptive)

### Git Commit Conventions

**Commit Message Format:**
```
<type>: <short description>

<optional longer description>
```

**Types:**
- `feat:` - New feature
- `fix:` - Bug fix
- `chore:` - Maintenance (deps, build, etc.)
- `docs:` - Documentation
- `test:` - Tests
- `refactor:` - Code refactoring
- `perf:` - Performance improvements

**Examples:**
```
feat: add dynamic import support in chunk linker

fix: resolve circular dependency in module graph

chore: update swc_core to 0.106.4

perf: optimize module scanning with parallel iteration
```

---

## Testing & Benchmarking

### Test Structure

**Unit Tests:**
- Located in same file as implementation
- Use `#[cfg(test)]` module
- Test individual functions/methods

**Integration Tests:**
- `packages/unpack/tests/` - Node.js integration tests
- Test end-to-end bundling scenarios

**Fixtures:**
- `packages/unpack/tests/fixtures/` - Sample source files
- `crates/unpack_turbo/fixtures/` - Turbo implementation fixtures

### Running Tests

```bash
# All Rust tests
cargo test

# Specific crate
cargo test -p unpack_core

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture

# Node.js tests
cd packages/unpack
pnpm test
```

### Benchmarking

**Available Benchmarks:**
- `crates/unpack_core/benches/module_graph_bench.rs` - Module graph performance
- `benchmark/build-tools-performance/` - Cross-tool comparison

**Running Benchmarks:**
```bash
# Run all benchmarks
cargo bench

# Specific benchmark
cargo bench --bench module_graph_bench

# Cross-tool benchmarks
cd benchmark/build-tools-performance
pnpm benchmark
```

**CI Benchmarks:**
- CodSpeed integration (`.github/workflows/codspeed.yml`)
- Automatic performance regression detection

### Writing Tests

**Unit Test Example:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_graph_add_module() {
        let mut graph = ModuleGraph::new();
        let module = NormalModule::new("./index.js");

        graph.add_module(module);

        assert_eq!(graph.module_count(), 1);
    }

    #[tokio::test]
    async fn test_async_build() {
        let compiler = Compiler::new();
        let result = compiler.build().await;
        assert!(result.is_ok());
    }
}
```

**Integration Test Example:**
```typescript
// packages/unpack/tests/basic.test.mjs
import { Compiler } from '../src/index';

describe('Basic bundling', () => {
    it('should bundle simple entry point', async () => {
        const compiler = new Compiler({
            entry: './fixtures/src/index.js'
        });

        const result = await compiler.build();
        expect(result.assets).toBeDefined();
    });
});
```

---

## Build & Deployment

### Build Configuration

**Cargo Profiles:**

```toml
[profile.release]
codegen-units = 1        # Single codegen unit for optimization
debug = true             # Keep debug info
lto = "thin"            # Thin LTO for faster builds
opt-level = 3           # Maximum optimization
panic = "abort"         # Abort on panic (smaller binary)
strip = false           # Don't strip symbols

[profile.release-debug]
inherits = "release"    # Same as release
debug = true           # Extra debug info
```

**Rust Toolchain:**
- Nightly channel: `nightly-2025-06-04`
- Configured in `rust-toolchain.toml`
- Automatically used by rustup

**Compiler Flags:**
- `tokio_unstable` - Enabled in `.cargo/config.toml`
- Required for Tokio advanced features

### Building from Source

```bash
# Debug build (faster compilation)
cargo build

# Release build (optimized)
cargo build --release

# With specific profile
cargo build --profile release-debug

# Node.js bindings
cd crates/node_binding
pnpm build
```

### Node.js Package Structure

**Platform-specific packages:**
- `npm/darwin-x64/` - macOS x64
- `npm/linux-x64-gnu/` - Linux x64
- `npm/win32-x64-msvc/` - Windows x64

**Package Distribution:**
- NAPI bindings compiled per platform
- Optional dependencies in package.json
- Platform-specific installation

### CI/CD Workflows

**Available Workflows:**

1. **codspeed.yml** - Performance benchmarking
   - Runs benchmarks on PR
   - Tracks performance over time
   - Reports regressions

2. **opencode.yml** - Code analysis
   - Static analysis checks
   - Code quality metrics

3. **summary.yml** - AI issue summarization
   - Automatic issue summaries
   - Context extraction

### Release Process

**Not yet formalized, but typical steps would be:**

1. Update version in `Cargo.toml`
2. Update version in `package.json`
3. Run full test suite
4. Build release artifacts
5. Tag release in git
6. Publish to crates.io (Rust)
7. Publish to npm (Node.js)

---

## AI Assistant Guidelines

### When Working on This Codebase

#### 1. Always Read Before Modifying

**Critical Rule:** Never propose changes to code you haven't read.

```bash
# Good workflow:
1. Read relevant files first
2. Understand existing patterns
3. Make minimal, focused changes

# Bad workflow:
1. Immediately suggest changes
2. Assume code structure
```

#### 2. Use the Right Tools

**For Exploration:**
- Use Task tool with `subagent_type=Explore` for codebase exploration
- See `Agents.md` for detailed agent guidelines
- Thoroughness levels: `quick`, `medium`, `very thorough`

**For File Operations:**
- `Read` tool for specific files
- `Glob` tool for file patterns
- `Grep` tool for code search
- Avoid using bash for file operations

**Examples:**
```bash
# Finding all module-related files
Glob("**/module*.rs")

# Searching for specific function
Grep(pattern="fn build_module_graph", output_mode="content")

# Exploring error handling patterns
Task(subagent_type="Explore", prompt="How is error handling implemented?")
```

#### 3. Avoid Over-Engineering

**Principles:**
- Make only requested changes
- Don't add extra features
- Don't refactor surrounding code unless necessary
- Don't add comments to unchanged code
- Keep solutions simple and focused

**Example:**
```rust
// User asks: "Fix the module path resolution bug"

// Good: Fix only the bug
fn resolve_path(path: &str) -> Result<PathBuf> {
    // Fix the specific bug
    Ok(PathBuf::from(path))
}

// Bad: Add unnecessary features
fn resolve_path(path: &str, options: ResolveOptions) -> Result<PathBuf> {
    // Add caching (not requested)
    // Add validation (not needed)
    // Refactor existing code (not requested)
}
```

#### 4. Follow Existing Patterns

**When adding code:**
- Match existing style in the file
- Use same error handling approach
- Follow existing naming conventions
- Use same async patterns

**Example:**
```rust
// Existing pattern in compiler.rs:
async fn build(&self) -> Result<Compilation> {
    let modules = self.scan().await?;
    Ok(Compilation::new(modules))
}

// Your new code should match:
async fn link(&self) -> Result<ChunkGraph> {
    let chunks = self.analyze().await?;
    Ok(ChunkGraph::new(chunks))
}
```

#### 5. Security Considerations

**Always watch for:**
- Command injection vulnerabilities
- Path traversal issues
- XSS in generated code
- SQL injection (if applicable)
- Unsafe Rust usage without justification

**When you find a security issue:**
- Fix it immediately
- Document the fix
- Consider if similar issues exist elsewhere

#### 6. Task Management

**Use TodoWrite tool for:**
- Complex multi-step tasks (3+ steps)
- Non-trivial implementations
- When user provides multiple tasks
- Progress tracking

**Don't use TodoWrite for:**
- Single, simple tasks
- Trivial operations
- Purely conversational questions

#### 7. Git Workflow

**Branch Requirements:**
- Always work on `claude/claude-md-<session-id>` branch
- Never push to main without explicit permission
- Use descriptive commit messages

**Commit Process:**
```bash
1. git status (check what changed)
2. git diff (review changes)
3. git log (check commit style)
4. Draft commit message
5. git add <files>
6. git commit -m "message"
7. git status (verify)
```

**Push with retry:**
```bash
# Always use -u flag for new branches
git push -u origin claude/claude-md-<session-id>

# Retry on network failures with exponential backoff:
# Try 1: immediate
# Try 2: wait 2s
# Try 3: wait 4s
# Try 4: wait 8s
# Try 5: wait 16s (final attempt)
```

#### 8. Performance Awareness

**This is a performance-focused project:**
- Consider algorithmic complexity
- Use parallel iteration where appropriate (Rayon)
- Leverage async for I/O operations
- Cache expensive computations
- Profile before optimizing (don't guess)

**Optimization Process:**
1. Identify bottleneck (profile/bench)
2. Understand current implementation
3. Propose optimization
4. Implement with benchmarks
5. Verify improvement

#### 9. Testing Requirements

**When to write tests:**
- Adding new features
- Fixing bugs (regression test)
- Modifying critical paths (module graph, chunks)

**When tests are optional:**
- Trivial changes
- Documentation updates
- Build configuration changes

#### 10. Documentation Expectations

**Always document:**
- Public APIs
- Plugin hooks
- Complex algorithms
- Non-obvious design decisions

**Never document:**
- Self-explanatory code
- Private helpers
- Obvious implementations

#### 11. Debugging Approach

**When investigating issues:**

1. **Understand the context**
   ```bash
   # Read error messages carefully
   # Check git history for related changes
   # Understand the data flow
   ```

2. **Reproduce the issue**
   ```bash
   # Create minimal test case
   # Verify the issue exists
   ```

3. **Locate the root cause**
   ```bash
   # Use Explore agent for codebase understanding
   # Add debug logging if needed
   # Check related code paths
   ```

4. **Fix systematically**
   ```bash
   # Make minimal changes
   # Test the fix
   # Ensure no regressions
   ```

#### 12. Working with the Module System

**Key Concepts:**

The bundler uses a graph-based module system:

```
Entry Point
    ↓
ModuleScanner (builds graph)
    ↓
ModuleGraph (nodes = modules, edges = dependencies)
    ↓
ChunkLinker (groups modules into chunks)
    ↓
ChunkGraph (nodes = chunks, edges = chunk dependencies)
    ↓
Code Generation (parallel per module)
    ↓
Asset Creation (combine generated code)
```

**When modifying:**
- Understand graph invariants
- Preserve topological ordering
- Handle circular dependencies
- Update related components

#### 13. Working with Async Code

**Patterns in this codebase:**

```rust
// Tokio runtime (I/O, async tasks)
#[tokio::test]
async fn test_async() {
    let result = some_async_fn().await;
}

// Rayon (CPU-bound parallelism)
modules.par_iter()
    .map(|m| generate_code(m))
    .collect()

// Mixing both
async fn build() -> Result<()> {
    let modules = scan_modules().await?; // Async I/O
    let code = modules.par_iter()        // Parallel CPU
        .map(generate)
        .collect();
    Ok(())
}
```

#### 14. Memory Management Tips

**Patterns to follow:**

```rust
// Use Arc for shared ownership
let graph = Arc::new(ModuleGraph::new());
let graph_clone = Arc::clone(&graph);

// Use AtomicRefCell for interior mutability
let cache = AtomicRefCell::new(HashMap::new());
cache.borrow_mut().insert(key, value);

// Use arena allocation for temporary graphs
let arena = Arena::new();
let nodes = arena.alloc_many(modules);
```

#### 15. Plugin Development

**When adding plugin hooks:**

1. Define Rust trait method
2. Call hook at appropriate point
3. Add NAPI bridge
4. Update TypeScript types
5. Document behavior

**Example:**
```rust
// 1. In plugin.rs
#[async_trait]
pub trait Plugin {
    async fn before_build(&self, compiler: &Compiler) -> Result<()> {
        Ok(())
    }
}

// 2. In compiler.rs
for plugin in &self.plugins {
    plugin.before_build(self).await?;
}

// 3. In js_plugin.rs (NAPI bridge)
// Bridge implementation

// 4. In index.ts
interface Plugin {
    beforeBuild?(compiler: Compiler): Promise<void>;
}
```

---

## Quick Reference

### File Locations Cheat Sheet

| Need to modify | Look in |
|----------------|---------|
| Compilation pipeline | `crates/unpack_core/src/compiler/` |
| Module graph | `crates/unpack_core/src/module/` |
| Chunk generation | `crates/unpack_core/src/chunk/` |
| Dependency resolution | `crates/unpack_core/src/dependency/` |
| Plugin system | `crates/unpack_core/src/plugin/` |
| Node.js bindings | `crates/node_binding/src/` |
| TypeScript API | `packages/unpack/src/` |
| Integration tests | `packages/unpack/tests/` |
| Benchmarks | `crates/unpack_core/benches/` |

### Common Commands

```bash
# Build
cargo build                          # Debug build
cargo build --release                # Release build

# Test
cargo test                           # All Rust tests
cargo test -p unpack_core           # Specific crate
pnpm test                           # Node.js tests

# Lint
cargo fmt                            # Format code
cargo clippy                         # Lint code

# Benchmark
cargo bench                          # Run benchmarks

# Clean
cargo clean                          # Clean build artifacts
```

### Key Dependencies to Know

- **swc_core**: JavaScript/TypeScript parsing
- **rspack_resolver**: Module resolution
- **tokio**: Async runtime
- **rayon**: Parallel iteration
- **salsa**: Incremental computation
- **turbo-tasks**: Task system from Next.js

---

## Additional Resources

- **Agents.md** - Detailed agent usage guidelines
- **crates/*/README.md** - Per-crate documentation
- **Rust Book**: https://doc.rust-lang.org/book/
- **Tokio Documentation**: https://tokio.rs/
- **NAPI-RS**: https://napi.rs/

---

## Revision History

- **2026-01-11**: Initial CLAUDE.md creation based on comprehensive codebase analysis
- Repository state: Git commit `4147626` (chore: add oc)

---

**Note**: This document should be updated as the codebase evolves. When making significant architectural changes, update relevant sections to keep this guide accurate for future AI assistants.
