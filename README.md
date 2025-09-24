# NAHA MC Helper

An Electron application built with Svelte, Tailwind CSS, DaisyUI, and TypeScript.

## Features

- ⚡ **Vite** - Fast development server and build tool
- 🎯 **Svelte** - Reactive frontend framework with TypeScript support
- 🎨 **Tailwind CSS** - Utility-first CSS framework
- 🌈 **DaisyUI** - Beautiful component library for Tailwind CSS
- 📦 **Electron** - Cross-platform desktop application framework
- 🔧 **TypeScript** - Type-safe development
- 🚀 **ESM** - Modern JavaScript modules

## Prerequisites

- Node.js (v18 or higher)
- npm or yarn

## Installation

1. Clone the repository or navigate to the project directory
2. Install dependencies:

   ```bash
   npm install
   ```

## Development

To run the application in development mode:

1. Start the Vite development server:

   ```bash
   npm run dev
   ```

2. In a separate terminal, start Electron:

   ```bash
   npm start
   ```

The Vite dev server will run on `http://localhost:5173` and Electron will load this URL in development mode.

## Building

To build the application for production:

1. Build the Vite project:

   ```bash
   npm run build
   ```

2. Package the Electron app:

   ```bash
   npm run make
   ```

## Project Structure

```text
├── src/
│   ├── App.svelte          # Main Svelte component
│   ├── main.ts             # Application entry point
│   └── input.css           # Tailwind CSS imports
├── main.js                 # Electron main process
├── index.html              # HTML template
├── vite.config.ts          # Vite configuration
├── tailwind.config.js      # Tailwind CSS configuration
├── tsconfig.json           # TypeScript configuration
├── forge.config.cjs        # Electron Forge configuration
└── package.json            # Project dependencies and scripts
```

## Available Scripts

- `npm run dev` - Start Vite development server
- `npm run build` - Build for production
- `npm start` - Start Electron application
- `npm run make` - Package the application using Electron Forge

## Technologies Used

- **Electron** ^38.1.2 - Desktop application framework
- **Vite** ^7.1.6 - Build tool and dev server
- **Svelte** ^5.39.2 - Frontend framework
- **TypeScript** ^5.9.2 - Type safety
- **Tailwind CSS** ^3.4.0 - Utility-first CSS
- **DaisyUI** ^5.1.13 - Component library
- **Electron Forge** ^7.9.0 - Application packaging

## Security

This application follows Electron security best practices:

- `nodeIntegration: false` - Prevents Node.js access in renderer
- `contextIsolation: true` - Isolates the main world from the isolated world

## License

ISC
