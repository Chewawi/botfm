import { Database } from "@repo/common/config";
import type { Config } from "drizzle-kit";

export default {
	schema: "./src/schema/index.ts",
	out: "./drizzle",
	dialect: "postgresql",
	casing: "snake_case",
	dbCredentials: {
		url: Database.toUrl(),
	},
} satisfies Config;
