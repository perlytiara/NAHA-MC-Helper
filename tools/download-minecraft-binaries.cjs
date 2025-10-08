#!/usr/bin/env node
/* eslint-env node */

/**
 * Download Latest Minecraft Installer & Updater Binaries
 * Downloads all platform binaries from the latest GitHub release
 */

const fs = require('fs');
const path = require('path');
const https = require('https');

const GITHUB_API = 'https://api.github.com/repos/perlytiara/minecraft-installer/releases/tags/latest';
const OUTPUT_DIR = path.join(__dirname, 'minecraft-installer-releases');

/**
 * Fetch JSON from URL
 */
function fetchJSON(url) {
  return new Promise((resolve, reject) => {
    https.get(url, {
      headers: {
        'User-Agent': 'NAHA-MC-Helper-Downloader'
      }
    }, (res) => {
      let data = '';
      res.on('data', chunk => data += chunk);
      res.on('end', () => {
        try {
          resolve(JSON.parse(data));
        } catch (e) {
          reject(e);
        }
      });
    }).on('error', reject);
  });
}

/**
 * Download file from URL (handles redirects)
 */
function downloadFile(url, outputPath) {
  return new Promise((resolve, reject) => {
    const makeRequest = (requestUrl, redirectCount = 0) => {
      if (redirectCount > 5) {
        reject(new Error('Too many redirects'));
        return;
      }
      
      https.get(requestUrl, {
        headers: {
          'User-Agent': 'NAHA-MC-Helper-Downloader'
        }
      }, (res) => {
        // Handle redirects
        if (res.statusCode === 302 || res.statusCode === 301 || res.statusCode === 307 || res.statusCode === 308) {
          const redirectUrl = res.headers.location;
          if (redirectUrl) {
            makeRequest(redirectUrl, redirectCount + 1);
          } else {
            reject(new Error('Redirect without location'));
          }
          return;
        }
        
        if (res.statusCode !== 200) {
          reject(new Error(`HTTP ${res.statusCode}`));
          return;
        }
        
        // Now create the write stream for the final URL
        const file = fs.createWriteStream(outputPath);
        const totalSize = parseInt(res.headers['content-length'], 10);
        let downloaded = 0;
        
        res.on('data', chunk => {
          downloaded += chunk.length;
          const percent = ((downloaded / totalSize) * 100).toFixed(1);
          process.stdout.write(`\r   Downloading: ${percent}% (${(downloaded / 1024 / 1024).toFixed(2)} MB / ${(totalSize / 1024 / 1024).toFixed(2)} MB)`);
        });
        
        res.pipe(file);
        
        file.on('finish', () => {
          file.close(() => {
            process.stdout.write('\n');
            resolve();
          });
        });
        
        file.on('error', (err) => {
          file.close(() => {
            fs.unlink(outputPath, () => reject(err));
          });
        });
      }).on('error', (err) => {
        reject(err);
      });
    };
    
    makeRequest(url, 0);
  });
}

/**
 * Main download function
 */
async function downloadLatestBinaries() {
  try {
    console.log('🔍 Fetching latest release info from GitHub...');
    console.log(`   API: ${GITHUB_API}\n`);
    
    const release = await fetchJSON(GITHUB_API);
    
    console.log(`✅ Found release: ${release.name}`);
    console.log(`   Published: ${release.published_at}`);
    console.log(`   Commit: ${release.target_commitish}`);
    console.log(`   Assets: ${release.assets.length}\n`);
    
    // Create output directory
    if (!fs.existsSync(OUTPUT_DIR)) {
      fs.mkdirSync(OUTPUT_DIR, { recursive: true });
      console.log(`📁 Created directory: ${OUTPUT_DIR}\n`);
    } else {
      console.log(`📁 Using directory: ${OUTPUT_DIR}\n`);
    }
    
    // Filter for binary files only (not checksums)
    const binaries = release.assets.filter(asset => 
      !asset.name.endsWith('.sha256') &&
      (asset.name.includes('minecraft-installer') || asset.name.includes('minecraft-updater'))
    );
    
    console.log(`📦 Downloading ${binaries.length} binaries...\n`);
    
    let count = 0;
    for (const asset of binaries) {
      count++;
      console.log(`[${count}/${binaries.length}] ${asset.name}`);
      console.log(`   Size: ${(asset.size / 1024 / 1024).toFixed(2)} MB`);
      console.log(`   URL: ${asset.browser_download_url}`);
      
      const outputPath = path.join(OUTPUT_DIR, asset.name);
      
      // Check if file already exists with same size
      if (fs.existsSync(outputPath)) {
        const stats = fs.statSync(outputPath);
        if (stats.size === asset.size) {
          console.log(`   ✅ Already exists (same size), skipping\n`);
          continue;
        } else {
          console.log(`   🔄 File exists but size differs, re-downloading...`);
        }
      }
      
      await downloadFile(asset.browser_download_url, outputPath);
      console.log(`   ✅ Downloaded successfully\n`);
    }
    
    console.log('🎉 All binaries downloaded successfully!');
    console.log(`📁 Location: ${OUTPUT_DIR}`);
    console.log('\n📋 Downloaded files:');
    
    const files = fs.readdirSync(OUTPUT_DIR).filter(f => !f.endsWith('.sha256'));
    files.forEach(file => {
      const stats = fs.statSync(path.join(OUTPUT_DIR, file));
      console.log(`   ✓ ${file} (${(stats.size / 1024 / 1024).toFixed(2)} MB)`);
    });
    
  } catch (error) {
    console.error('\n❌ Error:', error.message);
    process.exit(1);
  }
}

// Run if called directly
if (require.main === module) {
  downloadLatestBinaries();
}

module.exports = { downloadLatestBinaries };

