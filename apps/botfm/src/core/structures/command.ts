import { type CommandContext, Command as SeCommand } from "seyfert";
import { MessageFlags } from "seyfert/lib/types";

export class Command extends SeCommand {
	async replyError(ctx: CommandContext, message: string) {
		await ctx.editOrReply({
			content: message,
			flags: MessageFlags.Ephemeral + MessageFlags.IsComponentsV2,
		});
	}
}
