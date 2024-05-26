#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
pub enum Stat {
    AttackDamage,
    AttackSpeed,
    AbilityPower,
    Armor,
    MagicResist,
    Health,
    CritChance,
    AbilityHaste,
    Tenacity,
    Lethality,
    MagicPen,
    Omnivamp,
    MoveSpeed,
    HealShieldPower,
}

impl Stat {
    pub fn get_gold_value(&self) -> f64 {
        match self {
            Stat::AttackDamage => 35.0,
            Stat::AttackSpeed => 30.0,
            Stat::AbilityPower => 20.0,
            Stat::Armor => 20.0,
            Stat::MagicResist => 18.0,
            Stat::Health => 2.67,
            Stat::CritChance => 40.0,
            Stat::AbilityHaste => 50.0,
            Stat::Tenacity => 0.0,
            Stat::Lethality => 30.0,
            Stat::MagicPen => 31.11,
            Stat::Omnivamp => 39.6,
            Stat::MoveSpeed => 80.0,
            Stat::HealShieldPower => 68.75,
        }
    }

    pub fn get_min_max(&self, is_double: bool) -> (f64, f64) {
        if is_double {
            match self {
                Stat::AttackDamage => (9.0, 18.0),
                Stat::AttackSpeed => (11.5, 23.0),
                Stat::AbilityPower => (15.0, 30.0),
                Stat::AbilityHaste => (9.0, 18.0),
                _ => (0.0, 0.0),
            }
        } else {
            match self {
                Stat::AttackDamage => (14.0, 28.0),
                Stat::AttackSpeed => (15.0, 30.0),
                Stat::AbilityPower => (23.0, 46.0),
                Stat::Armor => (18.0, 36.0),
                Stat::MagicResist => (19.0, 38.0),
                Stat::Health => (140.0, 280.0),
                Stat::CritChance => (12.0, 24.0),
                Stat::AbilityHaste => (14.0, 28.0),
                Stat::Tenacity => (9.0, 18.0),
                Stat::Lethality => (9.0, 18.0),
                Stat::MagicPen => (10.0, 20.0),
                Stat::Omnivamp => (4.5, 9.0),
                Stat::MoveSpeed => (9.0, 18.0),
                Stat::HealShieldPower => (4.5, 9.0),
            }
        }
    }

    pub fn all_variants() -> Vec<Stat> {
        vec![
            Stat::AttackDamage,
            Stat::AttackSpeed,
            Stat::AbilityPower,
            Stat::Armor,
            Stat::MagicResist,
            Stat::Health,
            Stat::CritChance,
            Stat::AbilityHaste,
            Stat::Tenacity,
            Stat::Lethality,
            Stat::MagicPen,
            Stat::Omnivamp,
            Stat::MoveSpeed,
            Stat::HealShieldPower,
        ]
    }
}
