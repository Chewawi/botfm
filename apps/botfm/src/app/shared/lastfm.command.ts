import type { LastFMSession } from "@repo/database/schema";
import { getTrackInfo, type Track } from "@repo/lastfm";

import { Command } from "~/core/structures/command";

export class LastFmCommand extends Command {
	async getTrackInfo(session: LastFMSession, track: Track) {
		const trackInfo = await getTrackInfo(
			session.lastfmUsername,
			track.artist?.["#text"] || "",
			track.name || "",
		).then((res) => ({
			duration: res?.track.duration
				? msToReadable(res.track.duration)
				: "Unknown Duration",
			playcount: res?.track?.playcount || "0",
			userplaycount: res?.track?.userplaycount || "0",
		}));

		return trackInfo;
	}
}

function msToReadable(duration: string) {
	const ms = parseInt(duration, 10);
	if (Number.isNaN(ms) || ms <= 0) return "Unknown Duration";
	const totalSeconds = Math.floor(ms / 1000);
	const minutes = Math.floor(totalSeconds / 60);
	const seconds = totalSeconds % 60;
	return `${minutes}:${seconds.toString().padStart(2, "0")}`;
}
