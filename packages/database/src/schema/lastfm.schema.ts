import { pgTable, text, varchar } from "drizzle-orm/pg-core";

/**
 * Database schema for Last.fm sessions.
 */
export const lastfmSessions = pgTable("lastfm_sessions", {
	userId: varchar("user_id", { length: 18 }).primaryKey(),
	lastfmUsername: text("lastfm_username").notNull(),
	sessionKey: varchar("session_key", { length: 255 }).notNull(),
	token: varchar("token", { length: 255 }).notNull(),
});

/**
 * Type representing a Last.fm session record.
 */
export interface LastFMSession {
	userId: string;
	lastfmUsername: string;
	sessionKey: string;
	token: string;
}
