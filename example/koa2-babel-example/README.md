# koa2-babel-example

## env setup

    1. npm init
    2. npm install koa
    3. npm install -D @babel/core @babel/cli @babel/preset-env
    4. npm install -D @babel/node
    5. npm install -D nodemon
    6. npm install -D @babel/preset-typescript
    7. npm install -D @babel/plugin-transform-runtime
    8. npm install @babel/runtime
    9. npm install -D typescript

@babel/core是Bable进行代码转换的核心，@babel/cli,@babel/node都依赖他


@babel/cli 是一个内置的 CLI，可以通过命令行编译文件


@babel/preset-env 是一个预设集合，允许您使用最新的 JavaScript，他会根据目标环境对代码降级处理（这里说的不严谨，具体可以去看《手把手教你如何配置Babel(3)—真实项目中如何去打补丁》）


@babel/node 是一个与 Node.js CLI 完全相同的 CLI，在运行之前使用 Babel 预设和插件进行编译，执行的时候会占用大量内存空间，Babel官方不建议在生产环境使用

## ERROR
1. Q: `SyntaxError: Cannot use import statement outside a module`
1. A: `"type": "module",` to package.json

