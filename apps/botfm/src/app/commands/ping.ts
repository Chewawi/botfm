import { Command, type CommandContext, Declare } from "seyfert";

@Declare({
	name: "ping",
	description: "Check the bot's latency",

	aliases: ["latency"],

	integrationTypes: ["GuildInstall", "UserInstall"],

	contexts: ["BotDM", "Guild", "PrivateChannel"],
})
export default class PingCommand extends Command {
	async run(ctx: CommandContext) {
		await ctx.deferReply();

		const latency = ctx.client.gateway.latency;
		await ctx.editOrReply({
			content: `Pong! Latency: ${latency}ms`,
		});
	}
}
