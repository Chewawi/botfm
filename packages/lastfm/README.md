# @repo/lastfm

Integration client for the [Last.fm API](https://www.last.fm/api).

## 📦 Structure

```

src/
├── client/
│ ├── index.ts # Main client logic
│ ├── api.ts # Low-level API requests
│ ├── helpers.ts # Utilities (signature, fetch, query)
│ └── index.ts
├── types/ # API and model definitions
├── constants.ts # Shared constants
└── index.ts # Package entry point

```

## ⚙️ Configuration

The client relies on environment variables defined in `@repo/common/config`:

```

LASTFM_KEY=
LASTFM_SECRET=
LASTFM_REDIRECT_URI=

```

These must be available in your runtime environment.

## 🚀 Usage

```ts
import { LastFmClient } from "@repo/lastfm";
import { db } from "@repo/database";

async function example() {
  const client = new LastFmClient(db.sessions);

  // Step 1: Generate an auth URL for a Discord user
  const authUrl = client.generateAuthUrl("123456789012345678");
  console.log("Authorize here:", authUrl);

  // Step 2: After OAuth callback, store the session
  const token = "callback_token_from_lastfm";
  await LastFmClient.handleCallback(token, BigInt("123456789012345678"));

  // Step 3: Retrieve current or recent track
  const session = await client.getUserSession(BigInt("123456789012345678"));
  const track = await client.getCurrentTrack(session);

  if (track) {
    console.log(`🎵 Now playing: ${track.artist["#text"]} - ${track.name}`);
  } else {
    console.log("No recent track found.");
  }
}

example().catch(console.error);
```

## 🧩 Internal Dependencies

- `@repo/common` → Environment variables
- `@repo/database` → User session storage and retrieval

---

## 🧰 Scripts

| Script     | Description                   |
| ---------- | ----------------------------- |
| `lint`     | Run Biome linter              |
| `lint:fix` | Automatically fix lint issues |

---

---

> Part of a **monorepo**, intended to be used alongside `@repo/common` and other `@repo/*` packages.
