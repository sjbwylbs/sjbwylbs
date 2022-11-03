# koa2-typescript-eslint

## [TypeScript+koa2+eslint搭建完整nodejs开发环境（自建脚手架）](https://juejin.cn/post/7062239453963616293)

## tsconfig.json template
{
  "compilerOptions": {
    "target": "esnext",
    "module": "commonjs",
    "sourceMap": true,
    "outDir": "./dist",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true
  },
  "include": ["src"],
  "exclude": ["node_modules", "dist", "public"]
}

npx tsc

npx eslint src/index.ts