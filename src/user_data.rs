use crate::trade_skill::{TradeSkill, TRADE_SKILLS};
use crate::weapon::{Weapon, WEAPONS};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct UserData {
    #[serde(default)]
    pub username: String,
    #[serde(rename = "mainHand")]
    pub main_hand: Option<Weapon>,
    pub secondary: Option<Weapon>,
    #[serde(default)]
    pub level: u8,
    #[serde(rename = "gearScore", default)]
    pub gear_score: u16,
    #[serde(rename = "tradeSkills", default = "default_trade_skills")]
    pub trade_skills: HashMap<TradeSkill, u8>,
    #[serde(default = "default_weapons")]
    pub weapons: HashMap<Weapon, u8>,
}

fn default_trade_skills() -> HashMap<TradeSkill, u8> {
    TRADE_SKILLS.iter().map(|&skill| (skill, 0)).collect()
}

fn default_weapons() -> HashMap<Weapon, u8> {
    WEAPONS.iter().map(|&weapon| (weapon, 0)).collect()
}

impl Default for UserData {
    fn default() -> Self {
        Self {
            username: "".to_string(),
            main_hand: None,
            secondary: None,
            level: 1,
            gear_score: 0,
            trade_skills: default_trade_skills(),
            weapons: default_weapons(),
        }
    }
}

impl UserData {
    pub fn get_main_hand_level(&self) -> u8 {
        if let Some(weapon) = self.main_hand.as_ref() {
            if let Some(level) = self.weapons.get(weapon) {
                *level
            } else {
                0
            }
        } else {
            0
        }
    }

    pub fn get_secondary_level(&self) -> u8 {
        if let Some(weapon) = self.secondary.as_ref() {
            if let Some(level) = self.weapons.get(weapon) {
                *level
            } else {
                0
            }
        } else {
            0
        }
    }

    pub fn get_trade_skill(&self, skill: TradeSkill) -> u8 {
        if let Some(level) = self.trade_skills.get(&skill) {
            *level
        } else {
            0
        }
    }

    pub fn get_weapon_level(&self, weapon: Weapon) -> u8 {
        if let Some(level) = self.weapons.get(&weapon) {
            *level
        } else {
            0
        }
    }
}
