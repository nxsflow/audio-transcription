# Project Structure

## Root Directory Layout
```
├── src/                    # Tauri application source
│   ├── components/         # React components
│   ├── pages/             # Application pages/routes
│   ├── hooks/             # Custom React hooks
│   ├── utils/             # Utility functions
│   ├── types/             # TypeScript type definitions
│   ├── assets/            # Static assets (images, icons)
│   └── main.tsx           # Application entry point
├── backend/               # Python sidecars & AWS CDK
│   ├── cdk/              # AWS CDK infrastructure code
│   ├── lambda/           # AWS Lambda functions
│   └── cdk.json          # CDK configuration
├── .github/              # GitHub Actions workflows
│   └── workflows/        # CI/CD pipeline definitions
├── audio/                # Audio assets (gitignored)
├── .kiro/                # Kiro configuration & steering
└── src-tauri/            # Tauri configuration & Rust code
    ├── src/              # Rust backend code
│   ├── sidecars/         # Python sidecar applications
    ├── Cargo.toml        # Rust dependencies
    └── tauri.conf.json   # Tauri configuration
```

## Conventions

### File Naming
- **React Components**: PascalCase (`UserProfile.tsx`)
- **Hooks**: camelCase with `use` prefix (`useAudioProcessor.ts`)
- **Utilities**: camelCase (`formatAudio.ts`)
- **Types**: PascalCase (`AudioConfig.ts`)
- **Python**: snake_case (`audio_processor.py`)

### Import Organization
1. External libraries (React, TanStack, etc.)
2. Internal components and utilities
3. Type imports (with `type` keyword)
4. Relative imports

### Component Structure
- Keep components focused and single-responsibility
- Use TanStack Query for data fetching
- Leverage TanStack Router for navigation
- Co-locate component-specific types and utilities

### Backend Organization
- Separate CDK stacks by domain/feature
- Keep Lambda functions lightweight
- Use Python sidecars for heavy processing
- Maintain clear separation between infrastructure and application code