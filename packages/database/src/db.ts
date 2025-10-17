import { Database } from "@repo/common/config";
import { SQL } from "bun";
import { drizzle } from "drizzle-orm/bun-sql";
import * as schema from "./schema";

/**
 * Database client configured with Drizzle ORM.
 */
export const sql = new SQL(Database.toUrl());

export const db = drizzle({
	client: sql,
	schema,
	casing: "snake_case",
});
