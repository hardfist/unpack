# Unpack - Claude Context File

## Project Overview

**Unpack** is an experimental bundler architecture project written in Rust. This project explores modern bundler design patterns and incremental compilation approaches using frameworks like Turbo Tasks and Salsa.

- **Repository**: https://github.com/hardfist/unpack
- **License**: Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International
- **Language**: Rust (Edition 2024)

## Project Structure

```
unpack/
├── crates/
│   ├── unpack_core/       # Core bundler logic
│   ├── unpack_turbo/      # Turbo Tasks integration
│   ├── unpack_salsa/      # Salsa framework integration
│   └── node_binding/      # Node.js bindings (N-API)
├── packages/              # JavaScript/TypeScript packages
├── benchmark/             # Performance benchmarks
│   └── build-tools-performance/
└── .github/               # GitHub workflows and configurations
```

## Technology Stack

### Core Technologies
- **Rust 2024 Edition**: Modern Rust with latest language features
- **Turbo Tasks**: Incremental computation framework (from Next.js)
- **Salsa**: Incremental compilation framework
- **SWC**: JavaScript/TypeScript parsing and transformation
- **Rspack**: Resolver and source handling

### Key Dependencies
- **Parser/AST**: `swc_core` for parsing JavaScript/TypeScript
- **Resolver**: `rspack_resolver` for module resolution
- **Async Runtime**: `tokio` and `async-std`
- **Parallel Processing**: `rayon` for data parallelism
- **Memory Management**: `mimalloc` allocator
- **Graph Operations**: `petgraph` for dependency graphs

### Node.js Integration
- N-API bindings for Node.js interoperability
- Located in `crates/node_binding/`

## Build and Development

### Prerequisites
- Rust toolchain (see `rust-toolchain.toml`)
- Node.js and pnpm (for JavaScript packages)

### Common Commands
```bash
# Build the project
cargo build

# Run tests
cargo test

# Build in release mode
cargo build --release

# Build with debug symbols (release-debug profile)
cargo build --profile release-debug

# Run benchmarks
cd benchmark/build-tools-performance
npm run bench
```

### Development Profiles
- **Debug**: Standard development build
- **Release**: Optimized build with thin LTO, codegen-units=1
- **Release-Debug**: Release optimizations + debug symbols

## Architecture Insights

### Incremental Compilation
The project explores two incremental compilation frameworks:

1. **Turbo Tasks** (`unpack_turbo/`): Used in Next.js, provides fine-grained incremental computation
2. **Salsa** (`unpack_salsa/`): Query-based incremental compilation framework from rust-analyzer

### Core Components
- **Module Resolution**: Integration with rspack_resolver
- **Source Handling**: Uses rspack_sources for source map support
- **Parallel Processing**: Rayon for CPU-bound parallelism, async runtime for I/O
- **Graph Management**: Dependency graph tracking with petgraph

## Workspace Configuration

The project uses Cargo workspace with:
- Shared dependency versions in `[workspace.dependencies]`
- Consistent edition (2024) and license across crates
- Centralized linting configuration
- Optimized release profiles

## Agent Usage Guidelines

See `Agents.md` for detailed information about using AI agents with this project.

### Quick Reference
- **Explore Agent**: Use for codebase exploration, finding patterns
- **General Agent**: Use for complex multi-step tasks
- **Direct Tools**: Use Read/Glob for specific file lookups

## Development Notes

### Performance Considerations
- Uses `mimalloc` for better allocation performance
- Release builds use thin LTO for balance of compilation time and runtime performance
- Parallel processing via rayon for CPU-intensive operations
- Async runtime (tokio) for I/O-bound operations

### Code Style
- Clippy linting enabled with `new_without_default` allowed
- Follows Rust 2024 edition conventions
- See `clippy.toml` for linting configuration

## Testing

Run tests with:
```bash
cargo test
```

For specific crates:
```bash
cargo test -p unpack_core
cargo test -p node_binding
```

## Contributing

This is an experimental project exploring bundler architecture. When contributing:
1. Follow the existing code structure
2. Ensure tests pass
3. Use the established incremental computation patterns
4. Consider performance implications of changes

## Related Documentation
- `Agents.md`: AI agent usage guidelines
- `README.md`: Basic project information
- Cargo documentation: Run `cargo doc --open`
