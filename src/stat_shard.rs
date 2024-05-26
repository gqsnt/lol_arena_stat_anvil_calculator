use crate::stat::Stat;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum StatShard {
    SimpleStat(Stat),
    DoubleStat(Stat, Stat),
    Faith,
    Pristine,
}


impl StatShard {
    pub fn all_variants(with_pristine: bool) -> Vec<StatShard> {
        let mut all_variants = vec![
            StatShard::SimpleStat(Stat::AttackDamage),
            StatShard::SimpleStat(Stat::AttackSpeed),
            StatShard::SimpleStat(Stat::AbilityPower),
            StatShard::SimpleStat(Stat::Armor),
            StatShard::SimpleStat(Stat::MagicResist),
            StatShard::SimpleStat(Stat::Health),
            StatShard::SimpleStat(Stat::CritChance),
            StatShard::SimpleStat(Stat::AbilityHaste),
            StatShard::SimpleStat(Stat::Tenacity),
            StatShard::SimpleStat(Stat::Lethality),
            StatShard::SimpleStat(Stat::MagicPen),
            StatShard::SimpleStat(Stat::Omnivamp),
            StatShard::SimpleStat(Stat::MoveSpeed),
            StatShard::SimpleStat(Stat::HealShieldPower),
            StatShard::DoubleStat(Stat::AttackDamage, Stat::AbilityPower),
            StatShard::DoubleStat(Stat::AttackSpeed, Stat::AbilityHaste),
            StatShard::Faith,
        ];
        if with_pristine {
            all_variants.push(StatShard::Pristine);
        }
        all_variants
    }
}
