use crate::trade_skill::{TradeSkill, TRADE_SKILLS};
use crate::weapon::{Weapon, WEAPONS};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct UserData {
    #[serde(default)]
    username: String,
    #[serde(rename = "mainHand")]
    main_hand: Option<Weapon>,
    secondary: Option<Weapon>,
    #[serde(default)]
    level: u8,
    #[serde(rename = "gearScore", default)]
    gear_score: u16,
    #[serde(rename = "tradeSkills", default = "default_trade_skills")]
    trade_skills: HashMap<TradeSkill, u8>,
    #[serde(default = "default_weapons")]
    weapons: HashMap<Weapon, u8>,
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
    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn set_main_hand(&mut self, main_hand: Option<Weapon>) {
        self.main_hand = main_hand;
    }

    pub fn set_secondary(&mut self, secondary: Option<Weapon>) {
        self.secondary = secondary;
    }

    pub fn set_level(&mut self, level: u8) {
        self.level = level;
    }

    pub fn set_gear_score(&mut self, gear_score: u16) {
        self.gear_score = gear_score;
    }

    pub fn set_trade_skill(&mut self, skill: TradeSkill, level: u8) {
        *self.trade_skills.entry(skill).or_insert(0) = level;
    }

    pub fn set_weapon_level(&mut self, weapon: Weapon, level: u8) {
        *self.weapons.entry(weapon).or_insert(0) = level;
    }

    pub fn get_main_hand(&self) -> Option<Weapon> {
        self.main_hand
    }

    pub fn get_secondary(&self) -> Option<Weapon> {
        self.secondary
    }

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

    pub fn get_level(&self) -> u8 {
        self.level
    }

    pub fn get_gear_score(&self) -> u16 {
        self.gear_score
    }

    pub fn get_trade_skill(&self, skill: TradeSkill) -> u8 {
        if let Some(level) = self.trade_skills.get(&skill) {
            *level
        } else {
            0
        }
    }

    pub fn get_trade_skills(&self) -> &HashMap<TradeSkill, u8> {
        &self.trade_skills
    }

    pub fn get_weapon_level(&self, weapon: Weapon) -> u8 {
        if let Some(level) = self.weapons.get(&weapon) {
            *level
        } else {
            0
        }
    }
}
