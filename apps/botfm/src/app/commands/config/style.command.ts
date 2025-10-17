import { userPreferences } from "@repo/database/models";
import {
	type CommandContext,
	createStringOption,
	Declare,
	Options,
	SubCommand,
	TextDisplay,
} from "seyfert";
import { MessageFlags } from "seyfert/lib/types";

const STYLE_CHOICES = ["Compact", "Detailed", "Minimal"] as const;
type StyleChoice = (typeof STYLE_CHOICES)[number];

const options = {
	style: createStringOption({
		description: "Choose your preferred output style (default: Detailed)",
		required: true,
		choices: [
			{ name: "Compact (Less info)", value: "Compact" },
			{ name: "Detailed (More info)", value: "Detailed" },
			{ name: "Minimal (No extra info)", value: "Minimal" },
		],
	}),
};

@Declare({
	name: "style",
	description: "Set your output style for commands",
	integrationTypes: ["GuildInstall", "UserInstall"],
	contexts: ["BotDM", "Guild", "PrivateChannel"],
})
@Options(options)
export default class StyleCommand extends SubCommand {
	async run(ctx: CommandContext<typeof options>) {
		const chosen = ctx.options.style as StyleChoice;
		await userPreferences.set(ctx.author.id, chosen);

		await ctx.editOrReply({
			components: [new TextDisplay().setContent(`Style set to: \`${chosen}\``)],
			flags: MessageFlags.Ephemeral + MessageFlags.IsComponentsV2,
		});
	}
}
