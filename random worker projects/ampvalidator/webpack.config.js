const path = require("path");
const UglifyJsPlugin = require("uglifyjs-webpack-plugin");
const webpack = require("webpack")

module.exports = {
  target: "webworker",
  mode: "production",
  entry: {
    "worker": "./src/index.js",
    "worker.min": "./src/index.js"
  },
  node: {
    fs: "empty",
    tls: "empty",
    net: "empty"
  },
  output: {
    filename: "[name].js",
    path: path.resolve(__dirname, "target")
  },
  optimization: {
    minimizer: [
      new UglifyJsPlugin({
        cache: true,
        parallel: true,
        include: /\.min\.js$/,
        uglifyOptions: {
          compress: true,
          ecma: 6,
          mangle: true
        },
        sourceMap: true
      })
    ]
  },
  module: {
    rules: [
      {
        test: /\.js$/,
        exclude: /(node_modules)/,
        use: {
          loader: "babel-loader?cacheDirectory=true",
          options: {
            presets: ['@babel/preset-env']
          }
        }
      }
    ]
  }
};
