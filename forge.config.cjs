module.exports = {
  packagerConfig: {
    asar: true,
    ignore: [
      /^\/tools\//,
      /^\/\.git/,
      /^\/\.vscode/,
      /^\/src/,
      /^\/\.eslintrc/,
      /^\/eslint\.config\.js$/,
      /^\/independentModules\.jsonc$/,
      /^\/postcss\.config\.js$/,
      /^\/svelte\.config\.js$/,
      /^\/tailwind\.config\.js$/,
      /^\/tsconfig\.json$/,
      /^\/vite\.config\.ts$/,
      /^\/package-app\.bat$/,
      /^\/ARCHITECTURE\.md$/,
      /^\/README\.md$/,
    ],
    extraResource: [
      // Minecraft Installer binaries for all platforms
      {
        from: 'tools/minecraft-installer-releases',
        to: 'binaries',
        filter: ['**/*']
      }
    ],
  },
  makers: [
    {
      name: '@electron-forge/maker-squirrel',
      config: {
        authors: 'NAHA Team',
        description: 'NAHA Minecraft Helper - Automated modloader installation tool',
        name: 'NAHA MC Helper'
      },
    },
    {
      name: '@electron-forge/maker-zip',
      platforms: ['darwin'],
    },
    {
      name: '@electron-forge/maker-deb',
      config: {},
    },
    {
      name: '@electron-forge/maker-rpm',
      config: {},
    },
  ],
};

