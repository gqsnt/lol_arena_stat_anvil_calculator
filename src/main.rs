use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

use lol_stat_shard::player_stats::PlayerStats;

fn main() {
    let mut default_stats = PlayerStats::new();
    loop {
        println!("Stat roll count: {:.2}", default_stats.total_shard);
        if default_stats.used_pristine {
            println!("Pristine used");
        }
        println!("Current stats: {}", default_stats.calculate_total_gold_value());
        for (stat, value) in &default_stats.hash_map {
            println!("\t{:?}: {:.2}", stat, value);
        }
        let menu_items = vec!["Roll", "Reset", "Stop"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an action")
            .default(0)
            .items(&menu_items[..])
            .interact()
            .expect("Failed to select action");

        match menu_items[selection] {
            "Roll" => {
                let shards = default_stats.get_3_choices();
                let shard_descriptions: Vec<String> = shards.iter().map(|(shard, percent)| format!("{:?} ({}%)", shard, percent * 100.0)).collect();
                let shard_selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Choose a StatShard")
                    .default(0)
                    .items(&shard_descriptions[..])
                    .interact()
                    .expect("Failed to select shard");
                let selected_shard = shards[shard_selection];
                default_stats.insert_shard(selected_shard);
            }
            "Reset" => {
                default_stats.reset();
            }
            "Stop" => {
                break;
            }
            _ => {
                println!("Invalid action. Please choose roll, reset, or stop.");
            }
        }
    }
}
