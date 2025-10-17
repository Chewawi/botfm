import { pgEnum, pgTable, varchar } from "drizzle-orm/pg-core";

// Enum for output style preferences
export const outputStyleEnum = pgEnum("output_style", [
	"Compact",
	"Detailed",
	"Minimal",
]);

/**
 * User preferences schema keyed by Discord user id.
 */
export const userPreferences = pgTable("user_preferences", {
	userId: varchar("user_id", { length: 18 }).primaryKey(),
	style: outputStyleEnum("style").notNull().default("Compact"),
});

export interface UserPreferencesRecord {
	userId: string;
	style: "Compact" | "Detailed" | "Minimal";
}
