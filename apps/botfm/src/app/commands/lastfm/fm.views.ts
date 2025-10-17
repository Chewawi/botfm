import { truncateText } from "@repo/common/utils";
import type { Track, UserInfo } from "@repo/lastfm";
import {
	MediaGallery,
	MediaGalleryItem,
	Section,
	Separator,
	TextDisplay,
	Thumbnail,
} from "seyfert";
import { Spacing } from "seyfert/lib/types";

export interface NowPlayingData {
	track: Track;
	user: UserInfo;
	trackInfo: {
		duration: string;
		playcount: string;
		userplaycount: string;
	};
	largeImage: string;
}

function createCompactView({
	track,
	user,
	trackInfo,
	largeImage,
}: NowPlayingData) {
	return [
		new TextDisplay().setContent(
			`**[${truncateText(track.name, 40)}](${track.url})**`,
		),
		new TextDisplay().setContent(
			`-# ${truncateText(track.artist?.["#text"] || "", 30)} - ${truncateText(
				track.album?.["#text"] || "Unknown Album",
				50,
			)}`,
		),
		new MediaGallery().addItems(
			new MediaGalleryItem().setMedia(largeImage || ""),
		),
		new Separator().setDivider(true).setSpacing(Spacing.Small),
		new TextDisplay().setContent(
			`-# plays: \`${trackInfo.userplaycount}\` | scrobbles: \`${user.playcount}\``,
		),
	];
}

function createDetailedView({
	track,
	user,
	trackInfo,
	largeImage,
}: NowPlayingData) {
	return [
		new Section()
			.setComponents([
				new TextDisplay().setContent(
					`### [${truncateText(track.name, 70)}](${track.url})`,
				),
				new TextDisplay().setContent(
					`-# ${truncateText(track.artist?.["#text"] || "", 60)}`,
				),
				new TextDisplay().setContent(
					`-# **${truncateText(track.album?.["#text"] || "Unknown Album", 60)}**` +
						`\n-# Duration: **${truncateText(trackInfo.duration || "Unknown Duration", 60)}**`,
				),
			])
			.setAccessory(new Thumbnail().setMedia(largeImage || "")),
		new Separator().setDivider(true).setSpacing(Spacing.Small),
		new TextDisplay().setContent(
			`-# Track plays: \`${trackInfo.userplaycount}\` · Total scrobbles: \`${user.playcount}\``,
		),
	];
}

function createMinimalView({ track }: NowPlayingData) {
	return [
		new TextDisplay().setContent(
			`[${truncateText(track.name, 50)}](${track.url}) — ${truncateText(track.artist?.["#text"] || "", 40)}`,
		),
	];
}

const styleBuilders = {
	Compact: createCompactView,
	Detailed: createDetailedView,
	Minimal: createMinimalView,
};

export function buildNowPlayingView(
	style: "Compact" | "Detailed" | "Minimal",
	data: NowPlayingData,
) {
	// If the style does not exist, use 'Detailed' as fallback.
	const builder = styleBuilders[style] || styleBuilders.Detailed;
	return builder(data);
}
