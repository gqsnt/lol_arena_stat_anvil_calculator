use std::collections::HashMap;

use rayon::prelude::*;

use lol_stat_shard::player_stats::PlayerStats;
use lol_stat_shard::stat::Stat;
use lol_stat_shard::stat_shard::StatShard;
use lol_stat_shard::trim_float;

struct Config {
    priority_stats: Vec<(StatShard, f64)>,
    stat_shard_per_round: usize,
    n_round: usize,
}

fn main() {
    let config = Config {
        priority_stats: vec![
            (StatShard::Pristine, 1.0),
            (StatShard::Faith, 0.8),
            (StatShard::SimpleStat(Stat::AttackDamage), 0.7),
            (StatShard::SimpleStat(Stat::AttackSpeed), 0.6),
            (StatShard::SimpleStat(Stat::MoveSpeed), 0.6),
            (StatShard::DoubleStat(Stat::AttackSpeed, Stat::AbilityHaste), 0.5),
            (StatShard::SimpleStat(Stat::CritChance), 0.3),
            (StatShard::SimpleStat(Stat::AbilityHaste), 0.3),
            (StatShard::SimpleStat(Stat::Lethality), 0.3),
            (StatShard::SimpleStat(Stat::Omnivamp), 0.3),
            (StatShard::DoubleStat(Stat::AttackDamage, Stat::AbilityPower), 0.2),
            (StatShard::SimpleStat(Stat::Armor), 0.1),
            (StatShard::SimpleStat(Stat::MagicResist), 0.1),
        ], // Example priorities
        stat_shard_per_round: 14,
        n_round: 1_000_000,
    };

    let all_stats: Vec<_> = (0..config.n_round).into_par_iter().map(|_| {
        let mut stats = PlayerStats::new();
        let mut rng = rand::thread_rng();
        for _ in 0..config.stat_shard_per_round {
            let choices = stats.get_3_choices(&mut rng);
            let selected_shard = select_high_priority_shard(&choices, &config.priority_stats);
            stats.insert_shard(selected_shard, &mut rng);
        }
        stats
    }).collect();
    analyze_results(&all_stats, &config.priority_stats);
}

fn select_high_priority_shard(choices: &[(StatShard, f64)], priority_stats: &[(StatShard, f64)]) -> (StatShard, f64) {
    let mut best_shard = choices[0];
    let mut highest_score = 0.0;

    for (shard, percent) in choices {
        for (priority_shard, weight) in priority_stats {
            if shard == priority_shard {
                let score = percent * weight;
                if score > highest_score {
                    highest_score = score;
                    best_shard = (*shard, *percent);
                }
            }
        }
    }
    best_shard
}

fn analyze_results(all_stats: &[PlayerStats], priority_stats: &[(StatShard, f64)]) {
    let mut total_values: HashMap<Stat, f64> = HashMap::new();
    let mut total_gold_value = 0.0;
    for stats in all_stats {
        total_gold_value += stats.calculate_total_gold_value();
        for (stat, value) in &stats.hash_map {
            let entry = total_values.entry(*stat).or_insert(0.0);
            *entry += value;
        }
    }

    let n_rounds = all_stats.len() as f64;
    println!("Average gold value after {} rounds: {}", n_rounds, trim_float(total_gold_value / n_rounds));
    println!("Average values after {} rounds:", n_rounds);
    for stat in total_values.keys() {
        let avg_value = total_values.get(stat).unwrap_or(&0.0) / n_rounds;
        println!("{:?}: {}", stat, trim_float(avg_value));
    }
}