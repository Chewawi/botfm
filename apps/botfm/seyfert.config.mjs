import { env } from "@repo/common/config";
import { config } from "seyfert";

export default config.bot({
	token: env.DISCORD_TOKEN ?? "",
	locations: {
		base: "src",
		commands: "app/commands",
		events: "app/events",
	},
	intents: ["Guilds", "GuildMessages", "MessageContent"],
});
