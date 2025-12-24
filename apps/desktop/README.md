# Desktop Application

Cross-platform desktop application built with Tauri, combining Rust performance with React frontend.

## Features

- **Tauri**: Rust-based desktop framework
- **React**: Simple React frontend for UI
- **Python Sidecars**: Heavy processing tasks handled by Python
- **Native Performance**: Direct system integration capabilities
- **Cross-Platform**: Windows, macOS, and Linux support

## Development

```bash
# Start Tauri development server
pnpm run tauri:dev

# Build for production
pnpm run tauri:build

# Start React development server only
pnpm run dev

# Build React frontend only
pnpm run build

# Run tests
pnpm run test

# Type checking
pnpm run type-check
```

## Project Structure

```
src/
├── src/             # main.rs and modules
├── sidecars/        # Python sidecar applications
├── Cargo.toml       # Rust dependencies
└── tauri.conf.json  # Tauri configuration
```

## Python Sidecars

The desktop application can leverage Python sidecars for:
- AI/ML processing with HuggingFace Transformers
- Audio processing tasks
- Heavy computational workloads

## Shared Types

This application uses shared TypeScript types from the `shared-types` package, ensuring consistency across the monorepo.