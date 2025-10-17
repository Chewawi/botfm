# @repo/database

Database layer for the `@repo/*` monorepo.

This package provides:

- **Schema definitions** (`schema/`) for Drizzle ORM
- **Models / helper functions** (`models/`) for interacting with tables
- **Cache resources** (`cache/`) for temporary storage of frequently accessed data

---

## 📦 Folder Structure

```

src/
├── cache/
│   ├── index.ts
│   └── sessions.ts
├── models/
│   ├── index.ts
│   └── lastfmSessions.ts
├── schema/
│   ├── index.ts
│   └── lastfm.ts
├── db.ts
└── index.ts

```

---

## ⚙️ Configuration

Requires environment variables for connecting to the database, provided via `@repo/common/config`:

---

## 🚀 Usage

```ts
import { db, lastfmSessions } from "@repo/database";

// Save or update a Last.fm session
await lastfmSessions.set(
  BigInt("123456789012345678"),
  "LastFmUser",
  "session_key_123",
  "token_abc"
);

// Retrieve a Last.fm session
const session = await lastfmSessions.get(BigInt("123456789012345678"));
console.log(session?.lastfmUsername);
```

---

## 🧰 Scripts

| Script     | Description                          |
| ---------- | ------------------------------------ |
| `lint`     | Run Biome linter                     |
| `lint:fix` | Automatically fix lint issues        |
| `generate` | Generate migration types from schema |
| `push`     | Push schema changes to the database  |
| `migrate`  | Run database migrations              |
| `studio`   | Open Drizzle Studio                  |

---

## 🔗 Notes

- Part of a **monorepo**, intended to be used alongside `@repo/common` and other `@repo/*` packages.
- Schema and model logic are **separated** to make it easy to expand with additional tables or modules (e.g., Spotify, Discord users, etc.).
