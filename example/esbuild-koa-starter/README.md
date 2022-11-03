# esbuild-koa-starter

## Reference

1. [esbuild-example-typescript](https://github.com/privatenumber/esbuild-loader-examples/blob/master/examples/typescript/package.json)
2. [Blazing fast TypeScript with Webpack and ESBuild ](https://dev.to/karanpratapsingh/blazing-fast-typescript-with-webpack-and-esbuild-4mhh)

## Env
npm install -D yarn
npm i koa
npm i -D webpack webpack-cli fork-ts-checker-webpack-plugin nodemon-webpack-plugin typescript

npm i -D webpack-node-externals


Configuration

If you have a tsconfig.json file, esbuild-loader will automatically detect it.

Alternatively, you can also pass it in directly via the tsconfigRaw option:

  {
      test: /\.tsx?$/,
      loader: 'esbuild-loader',
      options: {
          loader: 'tsx',
          target: 'es2015',
+         tsconfigRaw: require('./tsconfig.json')
      }
  }


Highly recommended TS configurations to match behavior with esbuild
https://esbuild.github.io/content-types/#typescript