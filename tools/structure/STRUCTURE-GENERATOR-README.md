# 📁 Project Structure Generator

This tool generates a comprehensive folder structure summary of your project, excluding common build directories like `node_modules`, `dist`, `.git`, etc.

## 🚀 Quick Start

### Windows (Batch Script)
```batch
# Generate structure to file
generate-structure.bat

# Generate structure to console
generate-structure.bat --console

# Generate structure to custom file
generate-structure.bat my-structure.txt
```

### Node.js (Cross-Platform)
```bash
# Generate structure to file (default: project-structure.txt)
node generate-structure.js

# Generate structure to console
node generate-structure.js . --console

# Generate structure to custom file
node generate-structure.js . my-structure.txt

# Analyze different directory
node generate-structure.js /path/to/project
```

## 📊 Features

### 🔍 **Comprehensive Analysis**
- **File Categorization**: Automatically categorizes files by type (Source Code, Documentation, Images, etc.)
- **Size Analysis**: Shows file sizes and identifies largest files
- **Depth Tracking**: Reports maximum directory depth and deepest paths
- **Statistics**: Provides detailed counts and summaries

### 🎨 **Visual Tree Structure**
- **Emoji Icons**: Different emojis for different file types
- **Tree Format**: Clean, readable tree structure with proper indentation
- **Directory Highlighting**: Clearly distinguishes folders from files

### 📂 **File Categories**
- 💻 **Source Code**: `.js`, `.ts`, `.svelte`, `.py`, `.java`, etc.
- 🎨 **Markup & Styles**: `.html`, `.css`, `.scss`, `.svg`, etc.
- ⚙️ **Configuration**: `.json`, `.yml`, `.config`, `.cjs`, etc.
- 📄 **Documentation**: `.md`, `.txt`, `.rst`, etc.
- 🖼️ **Images**: `.png`, `.jpg`, `.gif`, `.ico`, etc.
- 📦 **Archives & Executables**: `.zip`, `.exe`, `.dll`, `.pak`, etc.
- 📊 **Data**: `.csv`, `.pdf`, `.asar`, etc.

### 🚫 **Excluded Directories**
The script automatically excludes:
- `node_modules`
- `.git`, `.svn`, `.hg`
- `dist`, `build`, `out`
- `.cache`, `.tmp`, `.temp`
- `coverage`, `.nyc_output`
- Log files and local environment files

## 📋 Sample Output

```
📁 PROJECT FOLDER STRUCTURE
============================================================
Root: Minecraft-Server-Programs/
============================================================

├── 📁 MinecraftLauncherAutoInstaller/
│   ├── ⚙️ electron-builder.yml
│   ├── ⚙️ electron.vite.config.js
│   ├── ⚙️ package.json (2.1 KB)
│   ├── 📄 README.md (1.5 KB)
│   ├── 📁 src/
│   │   ├── 🎨 index.html
│   │   ├── 📁 main/
│   │   │   ├── 💻 background.cjs
│   │   │   ├── 💻 ipc-handlers.cjs
│   │   │   └── 💻 window.cjs
│   │   ├── 📁 renderer/
│   │   │   ├── 💻 App.svelte
│   │   │   ├── 📁 components/
│   │   │   └── 📁 routes/
│   │   └── 📁 services/
│   └── ⚙️ tailwind.config.cjs
└── 💻 generate-structure.js (8.2 KB)

============================================================
📊 PROJECT STRUCTURE SUMMARY
============================================================

📁 Total Directories: 25
📄 Total Files: 78
📏 Maximum Depth: 6 levels
🔍 Deepest Path: src/renderer/components/ui/forms

📂 FILES BY CATEGORY:
------------------------------
💻 Source Code: 45 files
⚙️ Configuration: 12 files
🎨 Markup & Styles: 8 files
📄 Documentation: 6 files
🖼️ Images: 5 files
📦 Archives & Executables: 2 files

📦 LARGEST FILES:
------------------------------
1. 📦 dist/NAHA-Minecraft-Helper-1.3.48-x64.exe (89.2 MB)
2. 💻 generate-structure.js (8.2 KB)
3. ⚙️ package-lock.json (156.8 KB)
4. 💻 App.svelte (12.4 KB)
5. 💻 Welcome.svelte (11.8 KB)

⏰ Generated on: 9/19/2025, 10:30:15 AM
🚫 Excluded: node_modules, .git, dist, build, out, .cache
```

## 🛠️ Customization

You can modify the `generate-structure.js` file to:

1. **Add/Remove Ignored Patterns**: Edit the `IGNORE_PATTERNS` array
2. **Customize File Categories**: Modify the `FILE_CATEGORIES` object
3. **Change Output Format**: Adjust the rendering functions
4. **Add New Statistics**: Extend the analysis in the `ProjectStructureAnalyzer` class

## 📝 Requirements

- **Node.js**: Version 12 or higher
- **Operating System**: Windows, macOS, or Linux
- **Permissions**: Read access to project directories

## 🔧 Troubleshooting

### "Node.js not found"
- Install Node.js from [nodejs.org](https://nodejs.org/)
- Ensure Node.js is in your system PATH

### "Permission denied"
- Run as administrator (Windows) or with `sudo` (Linux/macOS)
- Check file/directory permissions

### "Cannot read directory"
- Ensure you have read permissions for the target directory
- Some system directories may be restricted

## 📄 License

This tool is part of the Minecraft Server Programs project and follows the same licensing terms.

