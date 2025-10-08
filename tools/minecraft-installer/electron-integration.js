const { spawn } = require('child_process')
const path = require('path')

/**
 * Electron integration for Minecraft Updater
 * This provides a Node.js interface for your Electron app to interact with the updater
 */

class MinecraftUpdaterIntegration {
  constructor(updaterPath) {
    this.updaterPath = updaterPath || path.join(__dirname, 'minecraft-updater')
  }

  /**
   * Scan all Minecraft instances and return detailed information
   * @returns {Promise<Array>} Array of instance information objects
   */
  async scanInstances() {
    return new Promise((resolve, reject) => {
      const process = spawn(this.updaterPath, ['scan', '--format', 'json'])
      let output = ''
      let error = ''

      process.stdout.on('data', (data) => {
        output += data.toString()
      })

      process.stderr.on('data', (data) => {
        error += data.toString()
      })

      process.on('close', (code) => {
        if (code === 0) {
          try {
            const instances = JSON.parse(output)
            resolve(instances)
          } catch (e) {
            reject(new Error(`Failed to parse JSON: ${e.message}`))
          }
        } else {
          reject(new Error(`Updater process failed: ${error}`))
        }
      })
    })
  }

  /**
   * Update mods for a specific instance
   * @param {string} instancePath - Path to the instance directory
   * @param {string} modpackType - Type of modpack ('neoforge' or 'fabric')
   * @returns {Promise<Object>} Update result object
   */
  async updateInstance(instancePath, modpackType) {
    return new Promise((resolve, reject) => {
      const process = spawn(this.updaterPath, [
        'update',
        '--instance-path',
        instancePath,
        '--modpack-type',
        modpackType,
        '--format',
        'json',
      ])
      let output = ''
      let error = ''

      process.stdout.on('data', (data) => {
        output += data.toString()
      })

      process.stderr.on('data', (data) => {
        error += data.toString()
      })

      process.on('close', (code) => {
        if (code === 0) {
          try {
            const result = JSON.parse(output)
            resolve(result)
          } catch (e) {
            reject(new Error(`Failed to parse JSON: ${e.message}`))
          }
        } else {
          reject(new Error(`Update process failed: ${error}`))
        }
      })
    })
  }

  /**
   * Update all instances of a specific modpack type
   * @param {string} modpackType - Type of modpack ('neoforge' or 'fabric')
   * @returns {Promise<Array>} Array of update result objects
   */
  async updateAllInstances(modpackType) {
    return new Promise((resolve, reject) => {
      const process = spawn(this.updaterPath, [
        'update-all',
        '--modpack-type',
        modpackType,
        '--format',
        'json',
      ])
      let output = ''
      let error = ''

      process.stdout.on('data', (data) => {
        output += data.toString()
      })

      process.stderr.on('data', (data) => {
        error += data.toString()
      })

      process.on('close', (code) => {
        if (code === 0) {
          try {
            const results = JSON.parse(output)
            resolve(results)
          } catch (e) {
            reject(new Error(`Failed to parse JSON: ${e.message}`))
          }
        } else {
          reject(new Error(`Update all process failed: ${error}`))
        }
      })
    })
  }

  /**
   * Get a summary of all instances for display in your Electron app
   * @returns {Promise<Object>} Summary object with counts and basic info
   */
  async getInstancesSummary() {
    const instances = await this.scanInstances()

    const summary = {
      total: instances.length,
      byLauncher: {},
      byModLoader: {},
      withAutomodpack: 0,
      totalMods: 0,
    }

    instances.forEach((instance) => {
      // Count by launcher
      if (!summary.byLauncher[instance.launcher_type]) {
        summary.byLauncher[instance.launcher_type] = 0
      }
      summary.byLauncher[instance.launcher_type]++

      // Count by mod loader
      if (!summary.byModLoader[instance.mod_loader]) {
        summary.byModLoader[instance.mod_loader] = 0
      }
      summary.byModLoader[instance.mod_loader]++

      // Count automodpack instances
      if (instance.has_automodpack) {
        summary.withAutomodpack++
      }

      // Sum total mods
      summary.totalMods += instance.mod_count
    })

    return summary
  }
}

// Example usage for your Electron app
async function exampleUsage() {
  const updater = new MinecraftUpdaterIntegration()

  try {
    // Get summary for dashboard
    const summary = await updater.getInstancesSummary()
    console.log('Instances Summary:', summary)

    // Get detailed instance information
    const instances = await updater.scanInstances()
    console.log('Found instances:', instances.length)

    // Update a specific instance
    if (instances.length > 0) {
      const firstInstance = instances[0]
      console.log(`Updating instance: ${firstInstance.name}`)

      const result = await updater.updateInstance(
        firstInstance.instance_path,
        'neoforge', // or 'fabric'
      )

      console.log('Update result:', result)
    }

    // Update all NeoForge instances
    console.log('Updating all NeoForge instances...')
    const updateResults = await updater.updateAllInstances('neoforge')
    console.log('Update results:', updateResults)
  } catch (error) {
    console.error('Error:', error.message)
  }
}

// Export for use in your Electron app
module.exports = { MinecraftUpdaterIntegration }

// Run example if this file is executed directly
if (require.main === module) {
  exampleUsage()
}




