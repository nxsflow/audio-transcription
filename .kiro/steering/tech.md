# Technology Stack

## Applications

### Desktop App (Tauri)
- **Framework**: Tauri (Rust)
- **Purpose**: Native desktop application with Python sidecars

### Tauri Web App (TanStack Start)
- **Framework**: TanStack Start (meta-framework)
- **UI Library**: React with TypeScript
- **Component Library**: shadcn/ui (centralized in packages/ui)
- **Styling**: Tailwind CSS
- **State Management & Routing**: TanStack ecosystem
  - TanStack Start (meta-framework)
  - TanStack Router (type-safe routing)
  - TanStack Query (data fetching & caching)
- **Build System**: Vite (via TanStack Start)
- **Purpose**: Web application which is used in the Tauri desktop app

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
# Start desktop Tauri development server (start web app as well)
pnpm run dev

# Start web development server
pnpm run dev:tauri-react

# Build for production
pnpm run build  # Desktop
pnpm run build:tauri-react    # Web

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
python apps/desktop/sidecars/sidecar.py
```

### UI Components (shadcn/ui)
```bash
# Add shadcn/ui components (run from project root)
pnpm shadcn:add button input card alert accordion

# Add individual components
pnpm shadcn:add button
pnpm shadcn:add input
pnpm shadcn:add card

# Components are installed to: packages/ui/src/components/ui/
# Import in apps: 
#   import { Button, Input } from '@workspace/ui'
#   import { cn } from '@workspace/ui/lib/utils'
#   import type { AudioLevel } from '@workspace/shared-types'
```

### Workspace Aliases
- `@/` - Local app files (e.g., `@/components`, `@/lib`)
- `@workspace/ui` - Shared UI components and utilities
- `@workspace/shared-types` - Shared TypeScript types