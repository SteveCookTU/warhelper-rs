use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct WarMessage {
    #[serde(rename = "GUILD_ID")]
    guild_id: u64,
    #[serde(rename = "CHANNEL_ID")]
    channel_id: u64,
    #[serde(rename = "MESSAGE_ID")]
    message_id: u64,
}

impl PartialEq for WarMessage {
    fn eq(&self, other: &Self) -> bool {
        self.message_id == other.message_id
            && self.channel_id == other.channel_id
            && self.guild_id == other.guild_id
    }
}

impl WarMessage {
    pub fn new(guild_id: u64, channel_id: u64, message_id: u64) -> Self {
        Self {
            guild_id,
            channel_id,
            message_id,
        }
    }

    pub fn get_guild_id(&self) -> u64 {
        self.guild_id
    }

    pub fn get_channel_id(&self) -> u64 {
        self.channel_id
    }

    pub fn get_message_id(&self) -> u64 {
        self.message_id
    }
}
