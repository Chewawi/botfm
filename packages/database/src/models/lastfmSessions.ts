import { eq } from "drizzle-orm";
import type { SessionsResource } from "../cache";
import { db } from "../db";
import { type LastFMSession, lastfmSessions } from "../schema";

/**
 * Database helper functions for managing Last.fm sessions.
 */
export async function set(
	userId: string,
	lastfmUsername: string,
	sessionKey: string,
	token: string,
): Promise<void> {
	await db
		.insert(lastfmSessions)
		.values({ userId, lastfmUsername, sessionKey, token })
		.onConflictDoUpdate({
			target: lastfmSessions.userId,
			set: { lastfmUsername, sessionKey, token },
		});
}

export async function get(
	userId: string,
	cache: SessionsResource,
): Promise<LastFMSession | null> {
	// Check cache first
	const cached = await cache.get(userId);
	if (cached) return cached;

	const result = await db
		.select()
		.from(lastfmSessions)
		.where(eq(lastfmSessions.userId, userId))
		.limit(1);

	return result[0] ?? null;
}
