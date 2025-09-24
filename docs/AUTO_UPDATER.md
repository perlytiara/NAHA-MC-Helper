# Auto-Updater Implementation

This document describes the auto-updater system implemented for NAHA MC Helper.

## Overview

The auto-updater provides automatic checking, downloading, and installation of application updates with a beautiful GUI interface for the update process.

## Components

### 1. GitHub Actions Workflow (`.github/workflows/build-and-release.yml`)
- Automatically builds the application when tags are pushed
- Creates GitHub releases with installers
- Supports both manual and automatic releases

### 2. Auto-Updater Service (`src/shared/services/AutoUpdaterService.js`)
- Manages update checking, downloading, and installation
- Integrates with `electron-updater` for GitHub releases
- Provides IPC communication with the renderer process

### 3. Update UI Components

#### UpdateDialog (`src/shared/components/ui/updates/UpdateDialog.svelte`)
- Main update dialog with different states (checking, available, downloading, etc.)
- Beautiful animations and progress indicators
- Handles user interactions for download/install decisions

#### UpdateProgress (`src/shared/components/ui/updates/UpdateProgress.svelte`)
- Shows download progress with animated progress bar
- Displays download speed, transferred bytes, and time remaining
- Real-time progress updates during download

#### UpdateNotification (`src/shared/components/ui/updates/UpdateNotification.svelte`)
- Toast notification for available updates
- Auto-hides after 10 seconds
- Quick actions for download or dismiss

#### ReleaseNotes (`src/shared/components/ui/updates/ReleaseNotes.svelte`)
- Modal displaying release notes and changelog
- Beautiful formatting with feature highlights
- Link to full release notes on GitHub

### 4. Update Store (`src/shared/stores/updateStore.ts`)
- Svelte store managing update state
- Reactive updates across components
- Settings management for auto-update preferences

### 5. Update Manager (`src/shared/components/UpdateManager.svelte`)
- Main component coordinating all update functionality
- Handles IPC communication with main process
- Manages update lifecycle and state

## Features

### Automatic Update Checking
- Checks for updates every 6 hours by default (configurable)
- Checks on application startup (configurable)
- Manual update checking via menu or settings

### Update Process
1. **Check for Updates**: Queries GitHub releases API
2. **Show Notification**: Toast notification when update is available
3. **Download Progress**: Beautiful progress dialog with real-time updates
4. **Install & Restart**: Automatic installation and application restart

### Settings
- Enable/disable auto-updates
- Configure check interval (1-24 hours)
- Auto-download and auto-install options
- Check on startup toggle

### GUI Features
- Smooth animations and transitions
- Progress indicators with download speed
- Release notes with formatted changelog
- Error handling with retry options
- Auto-hiding notifications

## Usage

### Integration in Main App
Add the UpdateManager to your main App.svelte:

```svelte
<script>
  import AppUpdateIntegration from './shared/components/AppUpdateIntegration.svelte';
</script>

<AppUpdateIntegration />
```

### Manual Update Check
```javascript
// From anywhere in the app
window.updateManager?.checkForUpdates();
```

### Settings Integration
Include the AutoUpdateSettings component in your settings page:

```svelte
<script>
  import AutoUpdateSettings from './shared/components/ui/settings/AutoUpdateSettings.svelte';
</script>

<AutoUpdateSettings />
```

## Configuration

### Package.json
The `publish` configuration in package.json enables GitHub releases:

```json
{
  "build": {
    "publish": {
      "provider": "github",
      "owner": "perlytiara",
      "repo": "NAHA-MC-Helper"
    }
  }
}
```

### GitHub Actions
The workflow automatically:
- Builds the application
- Creates installers for Windows
- Publishes to GitHub releases
- Updates version information

## Dependencies

- `electron-updater`: Core auto-updater functionality
- `electron`: Main process integration
- Svelte stores for state management
- Tailwind CSS for styling

## Security

- Uses GitHub's secure release system
- Validates update signatures
- Secure IPC communication
- No external update servers required

## Testing

### Development Testing
1. Build the application: `npm run build`
2. Create a test release with a higher version number
3. Test the update flow in development

### Production Testing
1. Create a release tag: `git tag v1.0.1`
2. Push the tag: `git push origin v1.0.1`
3. GitHub Actions will automatically build and release

## Troubleshooting

### Common Issues

1. **Updates not detected**: Check GitHub repository permissions and release format
2. **Download fails**: Verify network connectivity and GitHub API access
3. **Installation fails**: Check application permissions and antivirus software

### Debug Mode
Enable debug mode in the application menu to see detailed update logs.

## Future Enhancements

- Delta updates for smaller download sizes
- Background update downloading
- Rollback functionality
- Update scheduling
- Multiple update channels (stable, beta, alpha)
