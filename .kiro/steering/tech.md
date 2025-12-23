# Technology Stack

## Frontend
- **Framework**: Tauri (Rust + Web technologies)
- **UI Library**: React with TypeScript
- **State Management & Routing**: TanStack ecosystem
  - TanStack Start (meta-framework)
  - TanStack Router (type-safe routing)
  - TanStack Query (data fetching & caching)
- **Build System**: Vite (via TanStack Start)

## Sidecars
- **Language**: Python
- **AI/ML**: HuggingFace Transformers
- **Audio Processing**: Python audio libraries

## Backend
- **Language**: TypeScript
- **Infrastructure**: AWS CDK

## Development Tools
- **Package Manager**: pnpm (frontend), uv (Python)
- **CI/CD**: GitHub Actions
- **Environment**: dotenv for configuration

## Common Commands

### Development
```bash
# Start Tauri development server
npm run tauri dev

# Build for production
npm run tauri build

# Install dependencies
pnpm install
uv pip install -r requirements.txt
```

### Backend/Infrastructure
```bash
# Deploy infrastructure
cd backend && cdk deploy

# Synthesize CloudFormation
cd backend && cdk synth

# Bootstrap CDK (first time)
cd backend && cdk bootstrap
```

### Python Sidecars
```bash
# Run Python sidecar
python src-tauri/sidecars/sidecar.py
```