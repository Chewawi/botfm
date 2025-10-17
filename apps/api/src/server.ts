import { html } from "@elysiajs/html";
import staticPlugin from "@elysiajs/static";
import { Elysia } from "elysia";
import { autoload } from "elysia-autoload";

export const app = new Elysia()
	.use(staticPlugin({ prefix: "/public" }))
	.use(
		await autoload({
			dir: "./routes",
		}),
	)
	.use(html());
