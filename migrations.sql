-- Create table for storing Last.fm sessions for Discord users
CREATE TABLE IF NOT EXISTS lastfm_sessions (
    user_id BIGINT PRIMARY KEY,           -- Discord user ID
    lastfm_username TEXT NOT NULL,        -- Last.fm username
    session_key TEXT NOT NULL,            -- Last.fm session key
    token TEXT NOT NULL,                  -- Last.fm API token
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP  -- Record creation timestamp
);

-- Create table for storing custom command prefixes per guild
CREATE TABLE IF NOT EXISTS prefixes (
    guild BIGINT PRIMARY KEY,             -- Discord guild/server ID
    prefix TEXT NOT NULL,                 -- Custom command prefix
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP  -- Record creation timestamp
);

-- Create table for tracking command usage statistics
CREATE TABLE IF NOT EXISTS command_uses (
    command_name TEXT PRIMARY KEY,        -- Command name
    uses INTEGER NOT NULL DEFAULT 0,      -- Usage counter
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP  -- Record creation timestamp
);

-- Create indices for performance optimization
CREATE INDEX IF NOT EXISTS idx_lastfm_sessions_user_id ON lastfm_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_prefixes_guild ON prefixes(guild);
CREATE INDEX IF NOT EXISTS idx_command_uses_uses ON command_uses(uses DESC);

-- Table comments
COMMENT ON TABLE lastfm_sessions IS 'Stores Last.fm session information for Discord users';
COMMENT ON TABLE prefixes IS 'Stores custom command prefixes for Discord guilds';
COMMENT ON TABLE command_uses IS 'Tracks usage statistics for bot commands';