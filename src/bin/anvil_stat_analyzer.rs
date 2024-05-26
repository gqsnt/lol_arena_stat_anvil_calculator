use lol_stat_shard::player_stats::PlayerStats;
use lol_stat_shard::trim_float;
use lol_stat_shard::stat::Stat;
use lol_stat_shard::stat_shard::StatShard;
use std::collections::HashMap;

struct Config {
    priority_stats: Vec<StatShard>,
    stat_shard_per_round: usize,
    n_round: usize,
}

fn main() {
    let config = Config {
        priority_stats: vec![
            StatShard::Pristine,
            StatShard::Faith,
            StatShard::SimpleStat(Stat::AttackDamage),
            StatShard::DoubleStat(Stat::AttackSpeed, Stat::AbilityHaste),
            StatShard::SimpleStat(Stat::AttackSpeed),
            StatShard::SimpleStat(Stat::MoveSpeed),
            StatShard::SimpleStat(Stat::CritChance),
            StatShard::SimpleStat(Stat::AbilityHaste),
            StatShard::DoubleStat(Stat::AttackDamage, Stat::AbilityPower),


        ], // Example priorities
        stat_shard_per_round: 14,
        n_round: 1000,
    };

    let mut all_stats = Vec::new();
    for _ in 0..config.n_round {
        let mut stats = PlayerStats::new();
        for _ in 0..config.stat_shard_per_round {
            let choices = stats.get_3_choices();
            let selected_shard = select_high_priority_shard(&choices, &config.priority_stats);
            stats.insert_shard(selected_shard);
        }
        all_stats.push(stats);
    }
    analyze_results(&all_stats, &config.priority_stats);
}

fn select_high_priority_shard(choices: &[(StatShard, f64)], priority_stats: &[StatShard]) -> (StatShard, f64) {
    for priority_shard in priority_stats {
        for (shard, percent) in choices {
            if shard == priority_shard {
                return (*shard, *percent);
            }
        }
    }
    choices[0] // Default to the first choice if no high priority stat found
}

fn analyze_results(all_stats: &[PlayerStats], priority_stats: &[StatShard]) {
    let mut total_values: HashMap<Stat, f64> = HashMap::new();
    let mut total_gold_value = 0.0;
    for stats in all_stats {
        total_gold_value += stats.calculate_total_gold_value();
        for shard in priority_stats {
            match shard {
                StatShard::SimpleStat(stat) | StatShard::DoubleStat(stat, _) => {
                    let value = stats.get_stat(*stat);
                    *total_values.entry(*stat).or_insert(0.0) += value;
                }
                _ => {}
            }
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