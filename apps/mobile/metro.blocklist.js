// Metro blocklist for platform-specific modules
// These modules should not be bundled for web platform

const nodeOnlyModules = [
  'better-sqlite3',
  'fs',
  'path',
  'crypto',
  'stream',
  'util',
];

const nodeOnlyPackages = [
  '@nozbe/watermelondb/adapters/sqlite/sqlite-node',
];

module.exports = {
  nodeOnlyModules,
  nodeOnlyPackages,
};
