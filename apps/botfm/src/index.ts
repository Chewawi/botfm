import { Prefixes } from "@repo/common/config";
import { SessionsResource } from "@repo/database/cache";
import { Client, type UsingClient } from "seyfert";
import { context } from "./core/context";

const client = new Client({
	context,
	commands: {
		prefix: (_) => {
			return [Prefixes.get()];
		},
		reply: () => true,
	},
	allowedMentions: {
		parse: [],
	},
}) as Client<true> & UsingClient;

client.cache.sessions = new SessionsResource(client.cache, client);

client
	.start()
	.then(() => client.uploadCommands({ cachePath: "./commands.json" }));
