import Koa from 'koa'
import pkg from '../package.json'
import config from '../config.json'

const app = new Koa()
const PORT = 3000

app.use(async ctx => {
    ctx.body = 'Hello World 2'
})

app.listen(PORT, () => {
    console.log(`[${pkg.name}] started at port: ${config.server.port}`)
})