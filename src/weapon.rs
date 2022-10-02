use serde::{Deserialize, Serialize};

pub const WEAPONS: [Weapon; 12] = [
    Weapon::SwordAndShield,
    Weapon::Rapier,
    Weapon::Hatchet,
    Weapon::Spear,
    Weapon::GreatAxe,
    Weapon::WarHammer,
    Weapon::Bow,
    Weapon::Musket,
    Weapon::FireStaff,
    Weapon::LifeStaff,
    Weapon::IceGauntlet,
    Weapon::VoidGauntlet,
];

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum Weapon {
    #[default]
    SwordAndShield,
    Rapier,
    Hatchet,
    Spear,
    GreatAxe,
    WarHammer,
    Bow,
    Musket,
    FireStaff,
    LifeStaff,
    IceGauntlet,
    VoidGauntlet,
}

impl From<Weapon> for String {
    fn from(w: Weapon) -> Self {
        match w {
            Weapon::SwordAndShield => "SWORDANDSHIELD".to_string(),
            Weapon::Rapier => "RAPIER".to_string(),
            Weapon::Hatchet => "HATCHET".to_string(),
            Weapon::Spear => "SPEAR".to_string(),
            Weapon::GreatAxe => "GREATAXE".to_string(),
            Weapon::WarHammer => "WARHAMMER".to_string(),
            Weapon::Bow => "BOW".to_string(),
            Weapon::Musket => "MUSKET".to_string(),
            Weapon::FireStaff => "FIRESTAFF".to_string(),
            Weapon::LifeStaff => "LIFESTAFF".to_string(),
            Weapon::IceGauntlet => "ICEGAUNT".to_string(),
            Weapon::VoidGauntlet => "VOIDGAUNT".to_string(),
        }
    }
}

impl TryFrom<String> for Weapon {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "SWORDANDSHIELD" => Ok(Weapon::SwordAndShield),
            "RAPIER" => Ok(Weapon::Rapier),
            "HATCHET" => Ok(Weapon::Hatchet),
            "SPEAR" => Ok(Weapon::Spear),
            "GREATAXE" => Ok(Weapon::GreatAxe),
            "WARHAMMER" => Ok(Weapon::WarHammer),
            "BOW" => Ok(Weapon::Bow),
            "MUSKET" => Ok(Weapon::Musket),
            "FIRESTAFF" => Ok(Weapon::FireStaff),
            "LIFESTAFF" => Ok(Weapon::LifeStaff),
            "ICEGAUNT" => Ok(Weapon::IceGauntlet),
            "VOIDGAUNT" => Ok(Weapon::VoidGauntlet),
            _ => Err("Failed to parse weapon"),
        }
    }
}

impl Weapon {
    pub fn get_label(&self) -> &'static str {
        match self {
            Weapon::SwordAndShield => "Sword and Shield",
            Weapon::Rapier => "Rapier",
            Weapon::Hatchet => "Hatchet",
            Weapon::Spear => "Spear",
            Weapon::GreatAxe => "Great Axe",
            Weapon::WarHammer => "War Hammer",
            Weapon::Bow => "Bow",
            Weapon::Musket => "Musket",
            Weapon::FireStaff => "Fire Staff",
            Weapon::LifeStaff => "Life Staff",
            Weapon::IceGauntlet => "Ice Gauntlet",
            Weapon::VoidGauntlet => "Void Gauntlet",
        }
    }

    pub fn get_abbreviation(&self) -> &'static str {
        match self {
            Weapon::SwordAndShield => "SS",
            Weapon::Rapier => "R",
            Weapon::Hatchet => "H",
            Weapon::Spear => "S",
            Weapon::GreatAxe => "GA",
            Weapon::WarHammer => "WH",
            Weapon::Bow => "B",
            Weapon::Musket => "M",
            Weapon::FireStaff => "FS",
            Weapon::LifeStaff => "LS",
            Weapon::IceGauntlet => "IG",
            Weapon::VoidGauntlet => "VG",
        }
    }
}
