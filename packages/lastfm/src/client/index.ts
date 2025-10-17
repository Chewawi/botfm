import { env } from "@repo/common/config";
import type { SessionsResource } from "@repo/database/cache";
import { lastfmSessions } from "@repo/database/models";
import type { LastFMSession } from "@repo/database/schema";
import { DEFAULT_IMAGE_URL, FALLBACK_IMAGES } from "../constants";
import type * as types from "../types";
import {
	getRecentTracks,
	getSession,
	getTrackInfo,
	getUserInfo,
	getWeeklyTrackChart,
} from "./api";

export class LastFmClient {
	private sessionCache = new Map<
		string,
		{ session: LastFMSession; expires: number }
	>();
	private readonly SESSION_CACHE_TTL = 5 * 60 * 1000;

	constructor(private cache: SessionsResource) {}

	static async handleCallback(token: string, userId: string): Promise<void> {
		const response = await getSession(token);
		if (!response.session) throw new Error("Invalid session data from Last.fm");

		await lastfmSessions.set(
			userId,
			response.session.name,
			response.session.key,
			token,
		);
		console.log(
			`[LastFM] ✅ Session stored for ${response.session.name} (${userId})`,
		);
	}

	generateAuthUrl(discordUserId: string): string {
		return `https://www.last.fm/api/auth/?api_key=${env.LASTFM_KEY}&cb=${encodeURIComponent(
			`${env.LASTFM_REDIRECT_URI}/${discordUserId}`,
		)}`;
	}

	async isLoggedIn(userId: string): Promise<boolean> {
		const cached = this.sessionCache.get(userId);
		if (cached && cached.expires > Date.now()) {
			return true;
		}

		const session = await lastfmSessions.get(userId, this.cache);

		if (session) {
			this.sessionCache.set(userId, {
				session,
				expires: Date.now() + this.SESSION_CACHE_TTL,
			});
		}

		return Boolean(session);
	}

	async getCurrentTrack(session: LastFMSession): Promise<types.Track | null> {
		const data = await getRecentTracks(session.lastfmUsername);
		const tracks = Array.isArray(data.recenttracks.track)
			? data.recenttracks.track
			: [data.recenttracks.track];

		if (tracks.length === 0) return null;

		const first = tracks[0];
		if (!first) return null;

		const isAlbumNameMissing = !first.album?.["#text"];

		if (isAlbumNameMissing) {
			try {
				const trackInfo = await getTrackInfo(
					session.lastfmUsername,
					first.artist["#text"],
					first.name,
				);

				if (trackInfo?.track?.album) {
					first.album = {
						...first.album,
						"#text": trackInfo.track.album.title,
						image: trackInfo.track.album.image,
					};
				}
			} catch (error) {
				console.error("[LastFM] Failed to fetch full track info:", error);
			}
		}

		return first;
	}

	async getTrackPlayCounts(
		userId: string,
		artist: string,
		track: string,
	): Promise<[number, number]> {
		const session = await this.getUserSessionCached(userId);

		const artistLower = artist.toLowerCase();
		const trackLower = track.toLowerCase();

		const [weeklyChart, trackInfo] = await Promise.all([
			getWeeklyTrackChart(session.lastfmUsername),
			getTrackInfo(session.lastfmUsername, artist, track),
		]);

		const weeklyTrack = weeklyChart.weeklytrackchart.track?.find(
			(item) =>
				item.artist["#text"].toLowerCase() === artistLower &&
				item.name.toLowerCase() === trackLower,
		);

		const weekly = weeklyTrack ? parseInt(weeklyTrack.playcount, 10) : 0;
		const totalPlays = parseInt(trackInfo.track.userplaycount || "0", 10);
		const monthly = Math.max(weekly, Math.floor(totalPlays / 4));

		return [weekly, monthly];
	}

	async getUserInfo(userId: string): Promise<types.UserInfo> {
		const session = await this.getUserSessionCached(userId);
		const data = await getUserInfo(session.lastfmUsername);
		return data.user;
	}

	async getUserSession(userId: string): Promise<LastFMSession> {
		const session = await lastfmSessions.get(userId, this.cache);
		if (!session) throw new Error("No session found for this user");
		return session;
	}

	private async getUserSessionCached(userId: string): Promise<LastFMSession> {
		const cached = this.sessionCache.get(userId);

		if (cached && cached.expires > Date.now()) {
			return cached.session;
		}

		const session = await lastfmSessions.get(userId, this.cache);
		if (!session) throw new Error("No session found for this user");

		this.sessionCache.set(userId, {
			session,
			expires: Date.now() + this.SESSION_CACHE_TTL,
		});

		return session;
	}

	getImageUrls(track: types.Track): [string, string, string] {
		const trackImages = track.image || [];
		const albumImages = track.album?.image || [];

		const result: [string, string, string] = ["", "", ""];
		const sizes = ["small", "large", "extralarge"] as const;

		for (const img of trackImages) {
			const url = img["#text"];
			if (!url || url === DEFAULT_IMAGE_URL) continue;

			switch (img.size) {
				case "small":
					result[0] = url;
					break;
				case "large":
					result[1] = url;
					break;
				case "extralarge":
					result[2] = url;
					break;
			}
		}

		for (let i = 0; i < 3; i++) {
			if (result[i]) continue;

			for (const img of albumImages) {
				if (
					img.size === sizes[i] &&
					img["#text"] &&
					img["#text"] !== DEFAULT_IMAGE_URL
				) {
					result[i] = img["#text"];
					break;
				}
			}
		}

		return [
			result[0] || FALLBACK_IMAGES.small,
			result[1] || FALLBACK_IMAGES.large,
			result[2] || FALLBACK_IMAGES.xlarge,
		];
	}

	clearSessionCache(userId?: string): void {
		if (userId) {
			this.sessionCache.delete(userId);
		} else {
			this.sessionCache.clear();
		}
	}

	pruneExpiredCache(): void {
		const now = Date.now();
		for (const [userId, cached] of this.sessionCache.entries()) {
			if (cached.expires <= now) {
				this.sessionCache.delete(userId);
			}
		}
	}
}
