import { Database } from "@repo/common/config";
import { createEvent } from "seyfert";

export default createEvent({
	data: { once: true, name: "botReady" },
	run(user, client) {
		client.logger.info(`Using database ${Database.toUrlSafe()}`);
		client.logger.info(`${user.username} is ready`);
	},
});
