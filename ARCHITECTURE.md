# NAHA MC Helper - Architecture Documentation

## Feature Folder Structure

This project follows a **feature-based architecture** inspired by the WebDevSimplified parity-deals-clone repository. This structure promotes better code organization, maintainability, and scalability.

## Directory Structure

```text
src/
├── app/                    # Application entry points and root components
│   └── App.svelte         # Main application component
├── features/              # Feature modules (business logic)
│   └── user-management/   # Example feature
│       ├── components/    # Feature-specific components
│       ├── stores/        # Feature-specific state management
│       ├── utils/         # Feature-specific utilities
│       ├── types/         # Feature-specific TypeScript types
│       └── index.ts       # Feature public API
├── shared/                # Shared resources across features
│   ├── components/        # Reusable UI components
│   ├── stores/           # Global state management
│   ├── utils/            # Utility functions
│   └── types/            # Common TypeScript types
├── lib/                   # External integrations and constants
│   └── constants.ts       # Application constants
└── main.ts               # Application bootstrap
```

## Architecture Rules

### Import Boundaries

The ESLint configuration enforces strict import boundaries to maintain clean architecture:

- **App layer** (`src/app/`): Can import from `features`, `shared`, and `lib`
- **Features layer** (`src/features/`): Can import from same feature, `shared`, and `lib` only
- **Shared layer** (`src/shared/`): Can import from `shared` and `lib` only  
- **Lib layer** (`src/lib/`): Can only import from `lib` (self-contained)

### Feature Organization

Each feature should be self-contained with:

```text
features/feature-name/
├── components/           # Feature-specific Svelte components
├── stores/              # Svelte stores for feature state
├── utils/               # Business logic and helper functions
├── types/               # TypeScript interfaces and types
└── index.ts             # Public API exports
```

## ESLint Configuration

The project includes two ESLint configurations:

### Option 1: eslint-plugin-boundaries (Current)
- File: `eslint.config.js`
- Enforces architectural boundaries using import patterns
- Supports ESLint v9

### Option 2: eslint-plugin-project-structure (Alternative)
- Files: `.eslintrc.alt.json` + `independentModules.jsonc`  
- More granular folder structure validation
- Includes file naming conventions

To switch to the alternative configuration:

1. Delete `eslint.config.js`
2. Rename `.eslintrc.alt.json` to `.eslintrc.json`

## Path Aliases

TypeScript and Vite are configured with path aliases for clean imports:

```typescript
// Instead of relative imports
import { User } from '../../../shared/types/common';

// Use clean aliases
import { User } from '@/shared/types/common';
```

Available aliases:

- `@/` → `src/`
- `@/app/` → `src/app/`
- `@/features/` → `src/features/`
- `@/shared/` → `src/shared/`
- `@/lib/` → `src/lib/`

## Development Commands

```bash
# Lint code with architectural rules
npm run lint

# Auto-fix linting issues
npm run lint:fix

# Build application
npm run build

# Start development server
npm run dev

# Start Electron app
npm start
```

## Best Practices

### Feature Development

1. Create features in `src/features/feature-name/`
2. Export public APIs through `index.ts`
3. Keep internal implementation private
4. Use TypeScript for type safety

### Component Organization

- **Feature-specific components**: `src/features/[feature]/components/`
- **Reusable components**: `src/shared/components/`
- Use PascalCase for component names

### State Management

- **Feature state**: `src/features/[feature]/stores/`
- **Global state**: `src/shared/stores/`
- Use Svelte stores for reactive state

### Utilities

- **Feature utilities**: `src/features/[feature]/utils/`
- **Shared utilities**: `src/shared/utils/`
- **Constants**: `src/lib/constants.ts`

## Benefits

✅ **Scalability**: Easy to add new features without affecting existing code
✅ **Maintainability**: Clear separation of concerns and responsibilities  
✅ **Team Development**: Multiple developers can work on different features
✅ **Code Quality**: ESLint rules prevent architectural violations
✅ **Reusability**: Shared components and utilities reduce duplication
✅ **Type Safety**: TypeScript with proper module boundaries

## Example Feature Implementation

See `src/features/user-management/` for a complete example of:

- TypeScript interfaces (`types/user.ts`)
- Svelte stores (`stores/userStore.ts`)
- Utility functions (`utils/userValidation.ts`)
- Feature components (`components/UserCard.svelte`)
- Public API exports (`index.ts`)
