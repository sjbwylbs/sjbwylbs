import Koa from 'koa';
import pkg from '../package.json';
import config from '../config.json';
import http from 'http';
import https from 'https';

const app = new Koa();

// logger

app.use(async (ctx: Koa.Context, next): Promise<void> => {
    await next();
    const rt = ctx.response.get('X-Response-Time');
    console.log(`${ctx.method} ${ctx.url} - ${rt}`);
});

// x-response-time

app.use(async (ctx: Koa.Context, next): Promise<void> => {
    const start = Date.now();
    await next();
    const ms = Date.now() - start;
    ctx.set('X-Response-Time', `${ms}ms`);
});

// response

app.use(async (ctx: Koa.Context): Promise<void> => {
    ctx.body = 'Hello World';
});

function logListen(port) {
    console.log(`[${pkg.name}] started at port: ${port}`);
}

app.keys = ["wechat", "koa2", "esbuild", "example"];
app.use(IPFilter)

http.createServer(app.callback()).listen(config.server.httpPort, () => logListen(config.server.httpPort));
https.createServer(app.callback()).listen(config.server.httpsPort, () => logListen(config.server.httpsPort));