import { eq } from "drizzle-orm";
import { db } from "../db";
import { type UserPreferencesRecord, userPreferences } from "../schema";

export type OutputStyle = UserPreferencesRecord["style"];

export async function set(userId: string, style: OutputStyle): Promise<void> {
	await db
		.insert(userPreferences)
		.values({ userId, style })
		.onConflictDoUpdate({
			target: userPreferences.userId,
			set: { style },
		});
}

export async function get(
	userId: string,
): Promise<UserPreferencesRecord | null> {
	const rows = await db
		.select()
		.from(userPreferences)
		.where(eq(userPreferences.userId, userId))
		.limit(1);
	return rows[0] ?? null;
}
