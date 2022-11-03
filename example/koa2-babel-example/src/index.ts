
import * as Koa from "koa";
const app = new Koa();

const msg: string = 'Hello World';

app.use(async (ctx: Koa.Context): Promise<void> => {
  ctx.body = msg;
});

app.listen(7000);
