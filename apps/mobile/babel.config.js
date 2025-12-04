module.exports = function (api) {
  api.cache(true);
  return {
    presets: ['babel-preset-expo'],
    plugins: [
      // WatermelonDB decorators support
      ['@babel/plugin-proposal-decorators', { legacy: true }],
      // WatermelonDB babel plugin
      '@nozbe/watermelondb/babel/esm',
    ],
  };
};
