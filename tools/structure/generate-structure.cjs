#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

/**
 * Script to generate a comprehensive folder structure summary
 * Excludes node_modules, .git, and other common build/cache directories
 */

// Directories and files to ignore
const IGNORE_PATTERNS = [
    'node_modules',
    '.git',
    '.svn',
    '.hg',
    'dist',
    'build',
    'out',
    '.cache',
    '.tmp',
    '.temp',
    'coverage',
    '.nyc_output',
    '.DS_Store',
    'Thumbs.db',
    '*.log',
    '.env.local',
    '.env.*.local'
];

// File extensions to categorize
const FILE_CATEGORIES = {
    'Source Code': ['.js', '.ts', '.jsx', '.tsx', '.vue', '.svelte', '.py', '.java', '.cpp', '.c', '.cs', '.php', '.rb', '.go', '.rs'],
    'Markup & Styles': ['.html', '.htm', '.css', '.scss', '.sass', '.less', '.xml', '.svg'],
    'Configuration': ['.json', '.yml', '.yaml', '.toml', '.ini', '.conf', '.config', '.cjs', '.mjs'],
    'Documentation': ['.md', '.txt', '.rst', '.adoc'],
    'Images': ['.png', '.jpg', '.jpeg', '.gif', '.bmp', '.webp', '.ico', '.svg'],
    'Archives & Executables': ['.zip', '.tar', '.gz', '.exe', '.dll', '.so', '.dylib', '.pak'],
    'Data': ['.csv', '.xlsx', '.xls', '.pdf', '.asar']
};

class ProjectStructureAnalyzer {
    constructor(rootPath = '.') {
        this.rootPath = path.resolve(rootPath);
        this.stats = {
            totalFiles: 0,
            totalDirectories: 0,
            filesByCategory: {},
            largestFiles: [],
            deepestPath: '',
            maxDepth: 0
        };
        
        // Initialize file categories
        Object.keys(FILE_CATEGORIES).forEach(category => {
            this.stats.filesByCategory[category] = 0;
        });
        this.stats.filesByCategory['Other'] = 0;
    }

    shouldIgnore(itemPath, itemName) {
        return IGNORE_PATTERNS.some(pattern => {
            if (pattern.includes('*')) {
                // Simple glob pattern matching
                const regex = new RegExp(pattern.replace(/\*/g, '.*'));
                return regex.test(itemName);
            }
            return itemName === pattern || itemPath.includes(pattern);
        });
    }

    categorizeFile(filePath) {
        const ext = path.extname(filePath).toLowerCase();
        
        for (const [category, extensions] of Object.entries(FILE_CATEGORIES)) {
            if (extensions.includes(ext)) {
                this.stats.filesByCategory[category]++;
                return category;
            }
        }
        
        this.stats.filesByCategory['Other']++;
        return 'Other';
    }

    formatBytes(bytes) {
        if (bytes === 0) return '0 B';
        const k = 1024;
        const sizes = ['B', 'KB', 'MB', 'GB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
    }

    async analyzeDirectory(dirPath, prefix = '', depth = 0) {
        const items = [];
        
        try {
            const entries = await fs.promises.readdir(dirPath, { withFileTypes: true });
            
            for (const entry of entries) {
                const fullPath = path.join(dirPath, entry.name);
                const relativePath = path.relative(this.rootPath, fullPath);
                
                if (this.shouldIgnore(relativePath, entry.name)) {
                    continue;
                }
                
                if (entry.isDirectory()) {
                    this.stats.totalDirectories++;
                    
                    // Check depth
                    if (depth > this.stats.maxDepth) {
                        this.stats.maxDepth = depth;
                        this.stats.deepestPath = relativePath;
                    }
                    
                    const subItems = await this.analyzeDirectory(fullPath, prefix + '  ', depth + 1);
                    items.push({
                        name: entry.name,
                        type: 'directory',
                        children: subItems,
                        path: relativePath
                    });
                } else {
                    this.stats.totalFiles++;
                    
                    try {
                        const stats = await fs.promises.stat(fullPath);
                        const category = this.categorizeFile(entry.name);
                        
                        // Track largest files
                        this.stats.largestFiles.push({
                            path: relativePath,
                            size: stats.size,
                            category: category
                        });
                        
                        items.push({
                            name: entry.name,
                            type: 'file',
                            size: stats.size,
                            category: category,
                            path: relativePath
                        });
                    } catch (err) {
                        items.push({
                            name: entry.name,
                            type: 'file',
                            error: 'Could not read file stats',
                            path: relativePath
                        });
                    }
                }
            }
        } catch (err) {
            console.error(`Error reading directory ${dirPath}:`, err.message);
        }
        
        return items.sort((a, b) => {
            // Directories first, then files
            if (a.type !== b.type) {
                return a.type === 'directory' ? -1 : 1;
            }
            return a.name.localeCompare(b.name);
        });
    }

    renderTree(items, prefix = '', isLast = true) {
        let output = '';
        
        items.forEach((item, index) => {
            const isLastItem = index === items.length - 1;
            const connector = isLastItem ? '└── ' : '├── ';
            const nextPrefix = prefix + (isLastItem ? '    ' : '│   ');
            
            if (item.type === 'directory') {
                output += `${prefix}${connector}📁 ${item.name}/\n`;
                if (item.children && item.children.length > 0) {
                    output += this.renderTree(item.children, nextPrefix);
                }
            } else {
                const sizeInfo = item.size ? ` (${this.formatBytes(item.size)})` : '';
                const categoryEmoji = this.getCategoryEmoji(item.category);
                output += `${prefix}${connector}${categoryEmoji} ${item.name}${sizeInfo}\n`;
            }
        });
        
        return output;
    }

    getCategoryEmoji(category) {
        const emojis = {
            'Source Code': '💻',
            'Markup & Styles': '🎨',
            'Configuration': '⚙️',
            'Documentation': '📄',
            'Images': '🖼️',
            'Archives & Executables': '📦',
            'Data': '📊',
            'Other': '📄'
        };
        return emojis[category] || '📄';
    }

    generateSummary() {
        // Sort largest files
        this.stats.largestFiles.sort((a, b) => b.size - a.size);
        this.stats.largestFiles = this.stats.largestFiles.slice(0, 10);

        let summary = '\n' + '='.repeat(60) + '\n';
        summary += '📊 PROJECT STRUCTURE SUMMARY\n';
        summary += '='.repeat(60) + '\n\n';
        
        summary += `📁 Total Directories: ${this.stats.totalDirectories}\n`;
        summary += `📄 Total Files: ${this.stats.totalFiles}\n`;
        summary += `📏 Maximum Depth: ${this.stats.maxDepth} levels\n`;
        summary += `🔍 Deepest Path: ${this.stats.deepestPath}\n\n`;
        
        summary += '📂 FILES BY CATEGORY:\n';
        summary += '-'.repeat(30) + '\n';
        
        Object.entries(this.stats.filesByCategory).forEach(([category, count]) => {
            if (count > 0) {
                const emoji = this.getCategoryEmoji(category);
                summary += `${emoji} ${category}: ${count} files\n`;
            }
        });
        
        if (this.stats.largestFiles.length > 0) {
            summary += '\n📦 LARGEST FILES:\n';
            summary += '-'.repeat(30) + '\n';
            
            this.stats.largestFiles.forEach((file, index) => {
                const emoji = this.getCategoryEmoji(file.category);
                summary += `${index + 1}. ${emoji} ${file.path} (${this.formatBytes(file.size)})\n`;
            });
        }
        
        return summary;
    }

    async generateReport() {
        console.log('🔍 Analyzing project structure...\n');
        console.log(`📁 Root Directory: ${this.rootPath}\n`);
        
        const structure = await this.analyzeDirectory(this.rootPath);
        
        let report = '📁 PROJECT FOLDER STRUCTURE\n';
        report += '='.repeat(60) + '\n';
        report += `Root: ${path.basename(this.rootPath)}/\n`;
        report += '='.repeat(60) + '\n\n';
        
        report += this.renderTree(structure);
        report += this.generateSummary();
        
        // Add timestamp
        report += `\n⏰ Generated on: ${new Date().toLocaleString()}\n`;
        report += `🚫 Excluded: ${IGNORE_PATTERNS.join(', ')}\n`;
        
        return report;
    }

    async saveReport(outputPath = 'project-structure.txt') {
        const report = await this.generateReport();
        
        try {
            await fs.promises.writeFile(outputPath, report, 'utf8');
            console.log(`✅ Report saved to: ${path.resolve(outputPath)}`);
            return outputPath;
        } catch (err) {
            console.error('❌ Error saving report:', err.message);
            throw err;
        }
    }
}

// Main execution
async function main() {
    const args = process.argv.slice(2);
    const rootPath = args[0] || '.';
    const outputPath = args[1] || 'project-structure.txt';
    
    console.log('🚀 Project Structure Analyzer');
    console.log('='.repeat(40));
    
    try {
        const analyzer = new ProjectStructureAnalyzer(rootPath);
        
        if (args.includes('--console') || args.includes('-c')) {
            // Output to console only
            const report = await analyzer.generateReport();
            console.log(report);
        } else {
            // Save to file and show summary
            await analyzer.saveReport(outputPath);
            
            // Show quick summary in console
            const structure = await analyzer.analyzeDirectory(analyzer.rootPath);
            console.log('\n📊 Quick Summary:');
            console.log(`├── Directories: ${analyzer.stats.totalDirectories}`);
            console.log(`├── Files: ${analyzer.stats.totalFiles}`);
            console.log(`└── Max Depth: ${analyzer.stats.maxDepth} levels`);
        }
        
    } catch (err) {
        console.error('❌ Error:', err.message);
        process.exit(1);
    }
}

// Run if called directly
if (require.main === module) {
    main();
}

module.exports = ProjectStructureAnalyzer;

