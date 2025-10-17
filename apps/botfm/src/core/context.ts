import { db } from "@repo/database";
import { LastFmClient } from "@repo/lastfm";
import { extendContext } from "seyfert";

export const context = extendContext((i) => {
	return {
		db: db,
		lastFm: new LastFmClient(i.client.cache.sessions),
	};
});
