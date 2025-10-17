// biome-ignore lint/correctness/noUnusedImports: we need this to be on scope
import html, { Html } from "@elysiajs/html";
import { LastFmClient } from "@repo/lastfm";
import type { ElysiaApp } from "..";

export default (app: ElysiaApp) =>
	app.use(html()).get(":userId", async ({ params, query }) => {
		const { userId } = params;
		const { token } = query as { token?: string };

		if (!token) {
			return (
				<html lang="en">
					<head>
						<meta charset="UTF-8" />
						<title>Authentication Error</title>
						<link rel="icon" href="/public/favicon.ico" type="image/x-icon" />
						<link rel="stylesheet" href="/public/style.css" />
					</head>
					<body>
						<div>❌ An error occurred: Missing token</div>
					</body>
				</html>
			);
		}

		try {
			await LastFmClient.handleCallback(token, BigInt(userId));

			return (
				<html lang="en">
					<head>
						<meta charset="UTF-8" />
						<title>Authentication Successful</title>
						<link rel="icon" href="/public/favicon.ico" type="image/x-icon" />
						<link rel="stylesheet" href="/public/style.css" />
					</head>
					<body>
						<div>✅ Authentication completed successfully! 🎉</div>
						<div>You can now return to Discord!</div>
					</body>
				</html>
			);
		} catch (err: unknown) {
			return (
				<html lang="en">
					<head>
						<meta charset="UTF-8" />
						<title>Authentication Error</title>
						<link rel="icon" href="/public/favicon.ico" type="image/x-icon" />
						<link rel="stylesheet" href="/public/style.css" />
					</head>
					<body>
						<div>
							❌ An error occurred:{" "}
							{err instanceof Error ? err.message : "Unknown error"}
						</div>
					</body>
				</html>
			);
		}
	});
