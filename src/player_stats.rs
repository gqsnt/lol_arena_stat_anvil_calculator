use std::collections::HashMap;

use rand::prelude::{IndexedRandom, SliceRandom};
use rand::Rng;

use crate::stat::Stat;
use crate::stat_shard::StatShard;
use crate::StatShardPercent;

#[derive(Debug, Clone)]
pub struct PlayerStats {
    pub hash_map: HashMap<Stat, f64>,
    pub used_pristine: bool,
    pub total_shard: u32,
    rng: rand::rngs::ThreadRng,
}

impl PlayerStats {
    pub fn new() -> Self {
        PlayerStats {
            hash_map: HashMap::new(),
            used_pristine: false,
            total_shard: 0,
            rng: rand::thread_rng(),
        }
    }

    pub fn get_3_choices(&mut self) -> Vec<StatShardPercent> {
        let mut shards = StatShard::all_variants(self.total_shard > 10 && !self.used_pristine);
        shards.shuffle(&mut self.rng);
        shards.truncate(3);
        let mut shards_percent = Vec::new();
        for shard in shards {
            let percent = match shard {
                StatShard::SimpleStat(_) | StatShard::DoubleStat(_, _) => {
                    self.rng.gen_range(0..=10) as f64 / 10.0
                }
                StatShard::Faith => {
                    (self.rng.gen_range(0..=10) as f64 / 10.0) + 1.0
                }
                StatShard::Pristine => {
                    self.rng.gen_range(2..=8) as f64 / 10.0
                }
            };
            shards_percent.push((shard, percent));
        }
        shards_percent
    }

    pub fn calculate_total_gold_value(&self) -> f64{
        let mut total_gold_value = 0.0;
        for (stat, value) in &self.hash_map {
            total_gold_value += value * stat.get_gold_value();
        }
        total_gold_value
    }


    pub fn get_stat(&self, stat: Stat) -> f64 {
        *self.hash_map.get(&stat).unwrap_or(&0.0)
    }

    pub fn get_all_stats(&self) -> HashMap<Stat, f64> {
        self.hash_map.clone()
    }

    pub fn insert_stat(&mut self, stat: Stat, value: f64) {
        let entry = self.hash_map.entry(stat).or_insert(0.0);
        *entry += value;
    }

    pub fn insert_shard(&mut self, shard: StatShardPercent) {
        let (shard, percent) = shard;
        match shard {
            StatShard::SimpleStat(stat) => {
                self.insert_stat(stat, stat.get_min_max(false).0 * (percent + 1.0));
            }
            StatShard::DoubleStat(stat1, stat2) => {
                self.insert_stat(stat1, stat1.get_min_max(true).0 * (percent + 1.0));
                self.insert_stat(stat2, stat2.get_min_max(true).0 * (percent + 1.0));
            }
            StatShard::Faith => {
                let all_stats = Stat::all_variants();
                let stat1 = all_stats.choose(&mut self.rng).unwrap();
                let  stat2 = all_stats.choose(&mut self.rng).unwrap();
                self.insert_stat(*stat1, stat1.get_min_max(false).1 * (percent * 1.5));
                self.insert_stat(*stat2, stat2.get_min_max(false).1 * (percent * 1.5));
            }
            StatShard::Pristine => {
                for (_, value) in self.hash_map.iter_mut() {
                    *value *= percent + 1.0;
                }
            }
        }
        self.total_shard += 1;
        if let StatShard::Pristine = shard {
            self.used_pristine = true;
        }
    }

    pub fn reset(&mut self) {
        self.hash_map.clear();
        self.total_shard = 0;
        self.used_pristine = false;
    }
}
