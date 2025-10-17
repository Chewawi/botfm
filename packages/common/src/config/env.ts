import path from "node:path";
import { fileURLToPath } from "node:url";
import { createEnv } from "@t3-oss/env-core";
import dotenv from "dotenv";
import { sync as findUpSync } from "find-up";
import { z } from "zod";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Look for the .env file starting from the current directory and moving up to the root
const rootEnvPath = findUpSync(".env", { cwd: __dirname });

if (rootEnvPath) {
	dotenv.config({ path: rootEnvPath });
} else {
	console.warn("⚠️ .env file not found in any parent directory");
}

// Create env with validation using zod
export const env = createEnv({
	server: {
		BOT_ID: z.string().transform(Number),
		DISCORD_TOKEN: z.string(),
		LASTFM_KEY: z.string(),
		LASTFM_SECRET: z.string(),
		LASTFM_REDIRECT_URI: z.string().url(),

		API_PORT: z.string().transform(Number).default(3000),
		API_HOST: z.string().default("0.0.0.0"),

		DB_HOST: z.string(),
		DB_USER: z.string(),
		DB_PASSWORD: z.string(),
		DB_NAME: z.string(),
		DB_PORT: z.string().transform(Number).default(5432),

		REDIS_HOST: z.string(),
		REDIS_PORT: z.string().transform(Number).default(6379),

		PREFIX_DEFAULT: z.string(),
		PREFIX_DEV: z.string(),

		LOG_PANIC_TOKEN: z.string(),
		LOG_PANIC_ID: z.string().transform(Number),
		LOG_ERROR_TOKEN: z.string(),
		LOG_ERROR_ID: z.string().transform(Number),
		ENABLE_WEBHOOKS: z
			.enum(["true", "false"])
			.default("false")
			.transform((val) => val === "true"),

		NODE_ENV: z
			.enum(["production", "development", "test"])
			.default("development"),
	},

	runtimeEnv: process.env,
});
