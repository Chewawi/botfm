import { getImageDominantColor, truncateText } from "@repo/common/utils";
import {
	type CommandContext,
	Container,
	createStringOption,
	Declare,
	Options,
	Section,
	TextDisplay,
	Thumbnail,
} from "seyfert";
import { LastFmCommand } from "~/app/shared/lastfm.command";
import { Paginator } from "~/core/structures/paginator";

const options = {
	query: createStringOption({
		description:
			"The track you want to search for (defaults to currently playing)",
	}),
};

@Declare({
	name: "lyrics",
	description: "Shows the lyrics of a track on Last.fm",
	aliases: ["lr", "ly"],
	integrationTypes: ["GuildInstall", "UserInstall"],
	contexts: ["BotDM", "Guild", "PrivateChannel"],
})
@Options(options)
export default class LyricsCommand extends LastFmCommand {
	async run(ctx: CommandContext<typeof options>) {
		const query = ctx.options.query;
		const session = await ctx.lastFm.getUserSession(ctx.author.id);

		if (!query && !session) {
			return this.replyError(
				ctx,
				"Please provide a track name or link your account with `/login`, to get the last played track lyrics for you",
			);
		}

		await ctx.deferReply();

		const track = await ctx.lastFm.getCurrentTrack(session);

		if (!track) {
			return this.replyError(ctx, "No music currently playing found.");
		}

		const artistName = track.artist["#text"];
		const trackName = track.name;

		const requestUrl = query
			? `https://lrclib.net/api/search?q=${encodeURIComponent(query)}`
			: `https://lrclib.net/api/search?artist_name=${encodeURIComponent(artistName)}&track_name=${encodeURIComponent(trackName)}`;

		const request = await Bun.fetch(requestUrl, {
			headers: {
				"User-Agent": "BotFM (https://github.com/chewawi/botfm)",
				Accept: "application/json",
			},
		});

		if (!request.ok) {
			return this.replyError(
				ctx,
				"Sorry, we don't have the lyrics for this track.",
			);
		}

		const response = await request.json();

		if (!response || response.length === 0) {
			return this.replyError(
				ctx,
				"Sorry, we don't have the lyrics for this track.",
			);
		}

		const lyricsData = response[0];
		const plainLyrics = lyricsData.plainLyrics;

		if (!plainLyrics) {
			return this.replyError(
				ctx,
				"Sorry, we don't have the lyrics for this track.",
			);
		}

		// Split lyrics into pages
		const allLines = plainLyrics.split("\n");
		const linesPerPage = 15;
		const lyricsPages: string[] = [];

		for (let i = 0; i < allLines.length; i += linesPerPage) {
			const pageLines = allLines.slice(i, i + linesPerPage);
			lyricsPages.push(pageLines.join("\n"));
		}

		const [_, largeImage] = ctx.lastFm.getImageUrls(track);
		const dominantColor = await getImageDominantColor(largeImage);

		// Create a container for each page
		const containers = lyricsPages.map((pageContent, page) => {
			return new Container()
				.setComponents([
					new Section()
						.setComponents(
							new TextDisplay().setContent(
								`### [${truncateText(track.name, 40)}](${track.url})`,
							),
							new TextDisplay().setContent(
								`-# ${truncateText(artistName || "", 30)} - ${truncateText(
									track.album?.["#text"] || "Unknown Album",
									50,
								)}\n\n_ _`,
							),
							new TextDisplay().setContent(
								`${pageContent}\n\n_ _\n-# Page: ${page + 1}/${lyricsPages.length}`,
							),
						)
						.setAccessory(new Thumbnail().setMedia(largeImage || "")),
				])
				.setColor(dominantColor);
		});

		const paginator = new Paginator({
			ctx,
			containers,
		});

		await paginator.reply();
	}
}
