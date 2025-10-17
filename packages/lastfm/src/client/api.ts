import { env } from "@repo/common/config";
import { LASTFM_BASE_URL } from "../constants";
import type {
	LastFmArtistInfoResponse,
	LastFmRecentTracksResponse,
	LastFmSessionResponse,
	LastFmTrackInfoResponse,
	LastFmUserInfoResponse,
	WeeklyTrackChartResponse,
} from "../types/api.ts";
import { buildQueryString, fetchJson, generateSignature } from "./helpers";

const BASE_PARAMS: [string, string][] = [
	["api_key", env.LASTFM_KEY],
	["format", "json"],
] as const;

export async function getSession(token: string) {
	const params: [string, string][] = [
		["method", "auth.getSession"],
		["api_key", env.LASTFM_KEY],
		["token", token],
	];
	const query = buildQueryString([
		...params,
		["api_sig", generateSignature(params)],
		["format", "json"],
	]);
	return fetchJson<LastFmSessionResponse>(`${LASTFM_BASE_URL}?${query}`);
}

export async function getRecentTracks(username: string, limit: number = 1) {
	const query = buildQueryString([
		["method", "user.getRecentTracks"],
		["user", username],
		["limit", String(limit)],
		...BASE_PARAMS,
	]);
	return fetchJson<LastFmRecentTracksResponse>(`${LASTFM_BASE_URL}?${query}`);
}

export async function getTrackInfo(
	username: string,
	artist: string,
	track: string,
) {
	const query = buildQueryString([
		["method", "track.getInfo"],
		["artist", artist],
		["track", track],
		["username", username],
		["autocorrect", "1"],
		...BASE_PARAMS,
	]);
	return fetchJson<LastFmTrackInfoResponse>(`${LASTFM_BASE_URL}?${query}`);
}

export async function getArtistInfo(artist: string) {
	const query = buildQueryString([
		["method", "artist.getInfo"],
		["artist", artist],
		["autocorrect", "1"],
		...BASE_PARAMS,
	]);
	return fetchJson<LastFmArtistInfoResponse>(`${LASTFM_BASE_URL}?${query}`);
}

export async function getUserInfo(username: string) {
	const query = buildQueryString([
		["method", "user.getInfo"],
		["user", username],
		...BASE_PARAMS,
	]);
	return fetchJson<LastFmUserInfoResponse>(`${LASTFM_BASE_URL}?${query}`);
}

export async function getWeeklyTrackChart(
	username: string,
	limit: number = 100,
) {
	const query = buildQueryString([
		["method", "user.getWeeklyTrackChart"],
		["user", username],
		["limit", String(limit)],
		...BASE_PARAMS,
	]);
	return fetchJson<WeeklyTrackChartResponse>(`${LASTFM_BASE_URL}?${query}`);
}
