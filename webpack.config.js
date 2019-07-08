const path = require('path');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const WorkerPlugin = require('worker-plugin');

module.exports = {
  entry: {
    app: './src/index.ts'
  },
  output: {
    filename: '[name].[hash].js',
    globalObject: 'self'
  },
  resolve: {
    extensions: ['.ts', '.js', '.wasm']
  },
  module: {
    rules: [
      {
        test: /\.ts$/,
        loader: 'ts-loader',
      },
      {
        test: /\.css$/,
        loader: [
          MiniCssExtractPlugin.loader,
          'css-loader'
        ]
      }
    ]
  },
  plugins: [
    new CleanWebpackPlugin(),
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, 'src/index.html')
    }),
    new MiniCssExtractPlugin({
      filename: 'style.[hash].css'
    }),
    new WasmPackPlugin({
      crateDirectory: __dirname,
      extraArgs: '--out-dir wasm-pkg --out-name index'
    }),
    new WorkerPlugin()
  ]
};
