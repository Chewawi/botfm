import { getImageDominantColor } from "@repo/common/utils";
import { userPreferences } from "@repo/database/models";
import { type CommandContext, Container, Declare } from "seyfert";
import { MessageFlags } from "seyfert/lib/types";
import { LastFmCommand } from "~/app/shared/lastfm.command";
import { buildTrackPlaysView } from "./trackplays.views";

@Declare({
	name: "trackplays",
	description: "Shows the number of times a track has been played on Last.fm",
	aliases: ["tp"],
	integrationTypes: ["GuildInstall", "UserInstall"],
	contexts: ["BotDM", "Guild", "PrivateChannel"],
})
export default class NowPlayingCommand extends LastFmCommand {
	async run(ctx: CommandContext) {
		const session = await ctx.lastFm.getUserSession(ctx.author.id);
		if (!session) {
			return this.replyError(ctx, "Link your account with /login");
		}

		const startTime = Date.now();
		const [track] = await Promise.all([ctx.lastFm.getCurrentTrack(session)]);
		const endTime = Date.now();
		console.log(`getCurrentTrack took ${endTime - startTime}ms`);

		if (!track) {
			return this.replyError(ctx, "No music currently playing");
		}

		const startTime2 = Date.now();
		const trackInfo = await this.getTrackInfo(session, track);
		const endTime2 = Date.now();
		console.log(`getTrackInfo took ${endTime2 - startTime2}ms`);

		const artistName = track.artist["#text"];
		const playcount = trackInfo.playcount;
		const userplaycount = trackInfo.userplaycount;

		const pref = await userPreferences.get(ctx.author.id);
		const style = pref?.style ?? "Minimal";

		const [_, largeImage] = ctx.lastFm.getImageUrls(track);

		const dominantColor = await getImageDominantColor(largeImage);

		const viewData = {
			username: session.lastfmUsername,
			track,
			artistName,
			playcount,
			userplaycount,
			largeImage,
		};

		const components = buildTrackPlaysView(style, viewData);

		const container = new Container()
			.setComponents(components)
			.setColor(dominantColor);

		await ctx.editOrReply({
			components: [container],
			flags: MessageFlags.IsComponentsV2,
		});
	}
}
