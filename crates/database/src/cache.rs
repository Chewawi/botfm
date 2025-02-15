// stealed from assyst owo

use std::hash::Hash;
use std::mem::size_of;
use std::time::Duration;
use std::u64;

use moka::sync::Cache;

use crate::model::lastfm::Lastfm;
use crate::model::prefix::Prefix;

trait TCacheV = Send + Sync + Clone + 'static;
trait TCacheK = Hash + Send + Sync + Eq + Clone + 'static;

fn default_cache<K: TCacheK, V: TCacheV>() -> Cache<K, V> {
    Cache::builder()
        .max_capacity(1000)
        .time_to_idle(Duration::from_secs(60 * 3))
        .build()
}

fn default_cache_sized<K: TCacheK, V: TCacheV>(size: u64) -> Cache<K, V> {
    Cache::builder()
        .max_capacity(size)
        .time_to_idle(Duration::from_secs(60 * 3))
        .build()
}

/// In-memory cache collection for frequently accessed areas of the database.
pub struct DatabaseCache {
    prefixes: Cache<u64, Prefix>,
    sessions: Cache<u64, Lastfm>,
}
impl DatabaseCache {
    pub fn new() -> Self {
        DatabaseCache {
            prefixes: default_cache_sized(100000),
            sessions: default_cache_sized(u64::MAX),
        }
    }

    pub fn get_prefix(&self, guild_id: u64) -> Option<Prefix> {
        self.prefixes.get(&guild_id)
    }

    pub fn set_prefix(&self, guild_id: u64, prefix: Prefix) {
        self.prefixes.insert(guild_id, prefix);
    }

    pub fn get_prefixes_cache_size(&self) -> usize {
        self.prefixes.run_pending_tasks();
        self.prefixes.entry_count() as usize
    }

    pub fn get_session(&self, user_id: u64) -> Option<Lastfm> {
        self.sessions.get(&user_id)
    }

    pub fn set_session(&self, user_id: u64, session: Lastfm) {
        self.sessions.insert(user_id, session);
    }

    pub fn size_of(&self) -> u64 {
        self.prefixes.run_pending_tasks();

        let mut size = 0;

        for prefix in &self.prefixes {
            // add key size
            size += size_of::<u64>() as u64;
            // add value size
            size += prefix.1.size_of();
        }

        size
    }
}

impl Default for DatabaseCache {
    fn default() -> Self {
        Self::new()
    }
}
