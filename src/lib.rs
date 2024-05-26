use stat_shard::StatShard;

pub mod stat;
pub mod stat_shard;
pub mod player_stats;

pub type StatShardPercent = (StatShard, f64);


pub fn trim_float(f: f64) -> String {
    format!("{:.2}", f).trim_end_matches('0').trim_end_matches('.').to_string()
}
