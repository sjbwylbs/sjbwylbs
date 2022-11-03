// External modules
const path = require('path');

// Webpack plugins
const NodemonPlugin = require('nodemon-webpack-plugin');
const ForkTsCheckerPlugin = require('fork-ts-checker-webpack-plugin');

// Environment config
const isDevelopment = process.env.NODE_ENV !== 'production';
const mode = isDevelopment ? 'development' : 'production';

// exclude node modules in webpack
const nodeExternals = require('webpack-node-externals');

// Bundle config options
const BUNDLE = {
  entry: './src/index.ts',
  output: {
    filename: 'app.js',
    path: path.resolve(__dirname, 'dist')
  }
};

module.exports = {
  mode,
  target: 'node',  // in order to ignore built-in modules like path, fs, etc.
  externalsPresets: { node: true }, // in order to ignore built-in modules like path, fs, etc.
  externals: [nodeExternals()], // in order to ignore all modules in node_modules folder
  entry: BUNDLE.entry,
  stats: 'errors-only',
  module: getLoaders(),
  plugins: getPlugins(),
  resolve: {
    extensions: ['.tsx', '.ts', '.js', '.json']
  },
  output: BUNDLE.output
};

/**
 * Loaders used by the application.
 */
function getLoaders() {
  const esbuild = {
    test: /\.(js|jsx|ts|tsx)?$/,
    loader: 'esbuild-loader',
    options: {
      loader: 'tsx',
      target: 'es2015'
    },
    exclude: /node_modules/
  };

  const loaders = {
    rules: [esbuild]
  };

  return loaders;
}

/**
 * Plugins
 */
function getPlugins() {
  const nodemon = new NodemonPlugin();
  const tsChecker = new ForkTsCheckerPlugin();
  return [tsChecker, nodemon];
}