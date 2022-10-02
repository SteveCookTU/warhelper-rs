use crate::war_message::WarMessage;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Deserialize, Serialize, Default)]
pub struct AlertConnector {
    pub code: String,
    pub date: String,
    pub time: String,
    pub server: String,
    pub faction: String,
    pub territory: String,
    pub title: String,
    #[serde(rename = "type")]
    pub r#type: u8,
    #[serde(default)]
    pub tanks: Vec<u64>,
    #[serde(default)]
    pub erdps: Vec<u64>,
    #[serde(default)]
    pub prdps: Vec<u64>,
    #[serde(default)]
    pub mdps: Vec<u64>,
    #[serde(default)]
    pub healers: Vec<u64>,
    #[serde(default)]
    pub tentative: Vec<u64>,
    #[serde(rename = "notAvailable", default)]
    pub not_available: Vec<u64>,
    #[serde(default)]
    pub artillery: Vec<u64>,
    #[serde(rename = "warMessages")]
    pub war_messages: Vec<WarMessage>,
}

impl PartialEq for AlertConnector {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl AlertConnector {
    pub fn get_users(&self) -> Vec<u64> {
        self.tanks
            .iter()
            .chain(&self.erdps)
            .chain(&self.prdps)
            .chain(&self.mdps)
            .chain(&self.healers)
            .chain(&self.tentative)
            .chain(&self.not_available)
            .copied()
            .collect()
    }

    pub fn get_guild_ids(&self) -> HashSet<u64> {
        let mut result = HashSet::new();
        for war_message in &self.war_messages {
            result.insert(war_message.get_guild_id());
        }
        result
    }

    pub fn contains_war_message(&self, guild_id: u64, channel_id: u64, message_id: u64) -> bool {
        let wm = WarMessage::new(guild_id, channel_id, message_id);
        self.war_messages.iter().any(|&wm2| wm2 == wm)
    }

    pub fn channel_contains_war_message(&self, guild_id: u64, channel_id: u64) -> bool {
        self.war_messages
            .iter()
            .any(|wm| wm.get_channel_id() == channel_id && wm.get_guild_id() == guild_id)
    }
}
