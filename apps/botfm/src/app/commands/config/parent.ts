import { AutoLoad, Command, Declare } from "seyfert";

@Declare({
	name: "config",
	description: "command",
	integrationTypes: ["GuildInstall", "UserInstall"],
	contexts: ["BotDM", "Guild", "PrivateChannel"],
})
@AutoLoad()
export default class CommandParent extends Command {}
