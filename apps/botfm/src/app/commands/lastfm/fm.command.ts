import { getImageDominantColor } from "@repo/common/utils";
import { userPreferences } from "@repo/database/models";
import { type CommandContext, Container, Declare, TextDisplay } from "seyfert";
import { MessageFlags } from "seyfert/lib/types";
import { LastFmCommand } from "~/app/shared/lastfm.command";
import { buildNowPlayingView } from "./fm.views";

@Declare({
	name: "nowplaying",
	description: "Display your current song from Last.fm",
	aliases: ["np", "fm", "now"],
	integrationTypes: ["GuildInstall", "UserInstall"],
	contexts: ["BotDM", "Guild", "PrivateChannel"],
})
export default class NowPlayingCommand extends LastFmCommand {
	async run(ctx: CommandContext) {
		const session = await ctx.lastFm.getUserSession(ctx.author.id);
		if (!session) {
			return this.replyError(ctx, "Link your account with /login");
		}

		const [track, user] = await Promise.all([
			ctx.lastFm.getCurrentTrack(session),
			ctx.lastFm.getUserInfo(ctx.author.id),
		]);

		if (!track) {
			return this.replyError(ctx, "No music currently playing");
		}

		const trackInfo = await this.getTrackInfo(session, track);

		const pref = await userPreferences.get(ctx.author.id);
		const style = pref?.style ?? "Detailed";

		const [_, largeImage] = ctx.lastFm.getImageUrls(track);

		const dominantColor = await getImageDominantColor(largeImage);

		const viewData = { track, user, trackInfo, largeImage };

		const components = buildNowPlayingView(style, viewData);

		if (track["@attr"]?.nowplaying === "true" && style !== "Detailed") {
			components.unshift(new TextDisplay().setContent("## Now Playing"));
		}

		const container = new Container()
			.setComponents(components)
			.setColor(dominantColor);

		await ctx.editOrReply({
			components: [container],
			flags: MessageFlags.IsComponentsV2,
		});
	}
}
