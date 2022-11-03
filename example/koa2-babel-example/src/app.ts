
import { Request, Response } from "koa";
export default class AppController {
    constructor(private request: Request, private response: Response) { }

    async index(): Promise<string> {
        return 'Hello world';
    }
}