use stat_shard::StatShard;

pub mod stat;
pub mod stat_shard;
pub mod player_stats;

pub type StatShardPercent = (StatShard, f64);
