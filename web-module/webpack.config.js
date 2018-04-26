// webpack.config.js
const path = require('path')
const webpack = require('webpack')

module.exports = {
  entry: './index.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'index.js'
  },
  module: {
    rules: [
      {
        test: [/\.vert$/, /\.frag$/],
        use: 'raw-loader'
      }
    ]
  },
  plugins: [
    new webpack.DefinePlugin({
      'CANVAS_RENDERER': JSON.stringify(true),
      'WEBGL_RENDERER': JSON.stringify(true)
    })
  ],
  mode: 'development'
}
