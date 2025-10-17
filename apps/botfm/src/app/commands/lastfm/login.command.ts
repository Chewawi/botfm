import {
	Button,
	Command,
	type CommandContext,
	Declare,
	Section,
	TextDisplay,
} from "seyfert";
import { ButtonStyle, MessageFlags } from "seyfert/lib/types";

@Declare({
	name: "login",
	description: "Login to Last.fm",

	aliases: ["link"],

	integrationTypes: ["GuildInstall", "UserInstall"],

	contexts: ["BotDM", "Guild", "PrivateChannel"],
})
export default class LoginCommand extends Command {
	async run(ctx: CommandContext) {
		let loggedIn = false;

		const session = await ctx.lastFm
			.getUserSession(ctx.author.id)
			.catch(() => null);
		if (session) {
			loggedIn = true;
		}

		const loginUrl = ctx.lastFm.generateAuthUrl(ctx.author.id);

		const section = new Section()
			.setComponents(
				new TextDisplay().setContent(
					loggedIn
						? "You are already logged in. Click the button below to login to Last.fm."
						: "Click the button below to login to Last.fm.",
				),
			)
			.setAccessory(
				new Button()
					.setLabel("Login to Last.fm")
					.setStyle(ButtonStyle.Link)
					.setURL(loginUrl),
			);

		await ctx.editOrReply({
			components: [section],
			flags: MessageFlags.Ephemeral + MessageFlags.IsComponentsV2,
		});
	}
}
