const {CracoAliasPlugin} = require('react-app-alias')

module.exports = {
  // webpack: {
  //   resolve: {
  //     alias: {
  //       '@api': path.resolve(__dirname, './src/api/'),
  //       '@components': path.resolve(__dirname, './src/components/'),
  //       '@hooks': path.resolve(__dirname, './src/hooks/'),
  //       '@pages': path.resolve(__dirname, './src/pages/'),
  //       '@router': path.resolve(__dirname, './src/router/'),
  //       '@utils': path.resolve(__dirname, './src/utils/'),
  //       '@': path.resolve(__dirname, './src/'),
  //     }
  //   }
  // },
  plugins: [
    {
      plugin: CracoAliasPlugin,
      options: {}
    },
  ],
  devServer: {
    port: 3721
  }
}