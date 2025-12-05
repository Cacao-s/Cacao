const { getDefaultConfig } = require('expo/metro-config');
const path = require('path');
const { nodeOnlyModules, nodeOnlyPackages } = require('./metro.blocklist');

// Find the project and workspace directories
const projectRoot = __dirname;
const workspaceRoot = path.resolve(projectRoot, '../..');

const config = getDefaultConfig(projectRoot);

// 1. Watch all files within the monorepo
config.watchFolders = [workspaceRoot];

// 2. Let Metro know where to resolve packages and in what order
config.resolver.nodeModulesPaths = [
  path.resolve(projectRoot, 'node_modules'),
  path.resolve(workspaceRoot, 'node_modules'),
];

// 3. Resolve symlinks to prevent issues with pnpm
config.resolver.resolveRequest = (context, moduleName, platform) => {
  // Block Node.js-only modules on web
  if (platform === 'web') {
    // Check if module is in blocklist
    if (nodeOnlyModules.includes(moduleName)) {
      return { type: 'empty' };
    }
    
    // Check if module path includes blocked packages
    if (nodeOnlyPackages.some(pkg => moduleName.includes(pkg))) {
      return { type: 'empty' };
    }
  }

  // Special handling for expo-router entry
  if (moduleName === 'expo-router/entry') {
    const expoRouterPath = require.resolve('expo-router/entry', {
      paths: [projectRoot, workspaceRoot],
    });
    return {
      filePath: expoRouterPath,
      type: 'sourceFile',
    };
  }
  
  // Otherwise, use default resolution
  return context.resolveRequest(context, moduleName, platform);
};

// 4. Force Metro to process all source files
config.resolver.sourceExts = [
  'expo.ts',
  'expo.tsx',
  'expo.js',
  'expo.jsx',
  'ts',
  'tsx',
  'js',
  'jsx',
  'json',
  'wasm',
  'mjs',
];

module.exports = config;
