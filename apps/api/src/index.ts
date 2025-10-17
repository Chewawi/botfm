import { env } from "@repo/common/config";
import { app } from "./server.ts";

const signals = ["SIGINT", "SIGTERM"];

for (const signal of signals) {
	process.on(signal, async () => {
		console.log(`Received ${signal}. Initiating graceful shutdown...`);
		await app.stop();
		process.exit(0);
	});
}

process.on("uncaughtException", (error) => {
	console.error(error);
});

process.on("unhandledRejection", (error) => {
	console.error(error);
});

app.listen(env.API_PORT, () =>
	console.log(`🦊 Server started at ${app.server?.url.origin}`),
);

export type ElysiaApp = typeof app;
