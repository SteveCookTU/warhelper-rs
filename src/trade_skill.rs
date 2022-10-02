use serde::{Deserialize, Serialize};

pub const TRADE_SKILLS: [TradeSkill; 17] = [
    TradeSkill::WeaponSmithing,
    TradeSkill::Armoring,
    TradeSkill::Engineering,
    TradeSkill::JewelCrafting,
    TradeSkill::Arcana,
    TradeSkill::Cooking,
    TradeSkill::Furnishing,
    TradeSkill::Mining,
    TradeSkill::TrackingSkinning,
    TradeSkill::Fishing,
    TradeSkill::Logging,
    TradeSkill::Harvesting,
    TradeSkill::Smelting,
    TradeSkill::StoneCutting,
    TradeSkill::LeatherWorking,
    TradeSkill::Weaving,
    TradeSkill::WoodWorking,
];

#[derive(
    Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Serialize, Deserialize, Debug,
)]
#[serde(try_from = "String", into = "String")]
pub enum TradeSkill {
    #[default]
    WeaponSmithing,
    Armoring,
    Engineering,
    JewelCrafting,
    Arcana,
    Cooking,
    Furnishing,
    Mining,
    TrackingSkinning,
    Fishing,
    Logging,
    Harvesting,
    Smelting,
    StoneCutting,
    LeatherWorking,
    Weaving,
    WoodWorking,
}

impl From<TradeSkill> for String {
    fn from(ts: TradeSkill) -> Self {
        match ts {
            TradeSkill::WeaponSmithing => "WEAPONSMITHING".to_string(),
            TradeSkill::Armoring => "ARMORING".to_string(),
            TradeSkill::Engineering => "ENGINEERING".to_string(),
            TradeSkill::JewelCrafting => "JEWELCRAFTING".to_string(),
            TradeSkill::Arcana => "ARCANA".to_string(),
            TradeSkill::Cooking => "COOKING".to_string(),
            TradeSkill::Furnishing => "FURNISHING".to_string(),
            TradeSkill::Mining => "MINING".to_string(),
            TradeSkill::TrackingSkinning => "TRACKINGSKINNING".to_string(),
            TradeSkill::Fishing => "FISHING".to_string(),
            TradeSkill::Logging => "LOGGING".to_string(),
            TradeSkill::Harvesting => "HARVESTING".to_string(),
            TradeSkill::Smelting => "SMELTING".to_string(),
            TradeSkill::StoneCutting => "STONECUTTING".to_string(),
            TradeSkill::LeatherWorking => "LEATHERWORKING".to_string(),
            TradeSkill::Weaving => "WEAVING".to_string(),
            TradeSkill::WoodWorking => "WOODWORKING".to_string(),
        }
    }
}

impl TryFrom<String> for TradeSkill {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "WEAPONSMITHING" | "0" => Ok(TradeSkill::WeaponSmithing),
            "ARMORING" | "1" => Ok(TradeSkill::Armoring),
            "ENGINEERING" | "2" => Ok(TradeSkill::Engineering),
            "JEWELCRAFTING" | "3" => Ok(TradeSkill::JewelCrafting),
            "ARCANA" | "4" => Ok(TradeSkill::Arcana),
            "COOKING" | "5" => Ok(TradeSkill::Cooking),
            "FURNISHING" | "6" => Ok(TradeSkill::Furnishing),
            "MINING" | "7" => Ok(TradeSkill::Mining),
            "TRACKINGSKINNING" | "8" => Ok(TradeSkill::TrackingSkinning),
            "FISHING" | "9" => Ok(TradeSkill::Fishing),
            "LOGGING" | "10" => Ok(TradeSkill::Logging),
            "HARVESTING" | "11" => Ok(TradeSkill::Harvesting),
            "SMELTING" | "12" => Ok(TradeSkill::Smelting),
            "STONECUTTING" | "13" => Ok(TradeSkill::StoneCutting),
            "LEATHERWORKING" | "14" => Ok(TradeSkill::LeatherWorking),
            "WEAVING" | "15" => Ok(TradeSkill::Weaving),
            "WOODWORKING" | "16" => Ok(TradeSkill::WoodWorking),
            _ => Err(value),
        }
    }
}

impl TradeSkill {
    pub fn get_label(&self) -> &'static str {
        match self {
            TradeSkill::WeaponSmithing => "Weaponsmithing",
            TradeSkill::Armoring => "Armoring",
            TradeSkill::Engineering => "Engineering",
            TradeSkill::JewelCrafting => "Jewelcrafting",
            TradeSkill::Arcana => "Arcana",
            TradeSkill::Cooking => "Cooking",
            TradeSkill::Furnishing => "Furnishing",
            TradeSkill::Mining => "Mining",
            TradeSkill::TrackingSkinning => "Tracking and Skinning",
            TradeSkill::Fishing => "Fishing",
            TradeSkill::Logging => "Logging",
            TradeSkill::Harvesting => "Harvesting",
            TradeSkill::Smelting => "Smelting",
            TradeSkill::StoneCutting => "Stonecutting",
            TradeSkill::LeatherWorking => "Leatherworking",
            TradeSkill::Weaving => "Weaving",
            TradeSkill::WoodWorking => "Woodworking",
        }
    }
}
