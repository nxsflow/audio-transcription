# Web Application

Modern web application built with TanStack Start, featuring advanced routing, state management, and data fetching capabilities.

## Features

- **TanStack Start**: Meta-framework for React applications
- **TanStack Router**: Type-safe routing with code splitting
- **TanStack Query**: Powerful data fetching and caching
- **React 19**: Latest React features and performance improvements
- **TypeScript**: Full type safety across the application
- **Tailwind CSS**: Utility-first CSS framework
- **Vite**: Fast build tool and development server

## Development

```bash
# Start development server
pnpm run dev

# Build for production
pnpm run build

# Preview production build
pnpm run preview

# Run tests
pnpm run test

# Type checking
pnpm run type-check

# Linting and formatting
pnpm run lint
pnpm run format
```

## Project Structure

```
src/
├── components/     # Reusable React components
├── data/           # Data fetching and presentation logic
├── db-collections/ # Local database collections
├── hooks/          # Custom React hooks
├── integrations/   # Third-party library integrations
├── lib/            # Utility functions and helpers
├── routes/         # Application pages and routing
└── router.tsx      # Main router configuration
```

## Shared Types

This application uses shared TypeScript types from the `shared-types` package, ensuring consistency across the monorepo.