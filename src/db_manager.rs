use crate::trade_skill::TradeSkill;
use crate::user_data::UserData;
use crate::war_message::WarMessage;
use crate::weapon::Weapon;
use crate::{async_trait, AlertConnector};
use futures::TryStreamExt;
use mongodb::bson;
use mongodb::bson::{doc, Document};
use mongodb::options::UpdateOptions;
use serenity::model::id::RoleId;
use uuid::Uuid;

#[async_trait]
pub trait DBManager {
    async fn get_alert_connector(&self, uuid: Uuid) -> Option<AlertConnector>;
    async fn get_alert_connectors(&self) -> Vec<AlertConnector>;
    async fn get_alert_connectors_with_user_id(&self, user_id: u64) -> Vec<AlertConnector>;
    async fn get_user_data(&self, user_id: u64) -> Option<UserData>;
    async fn has_permission(&self, guild_id: u64, roles: &[RoleId]) -> bool;
    async fn add_permission(&self, guild_id: u64, role_id: u64);
    async fn remove_permission(&self, guild_id: u64, role_id: u64);
    async fn channel_contains_war_message(
        &self,
        guild_id: u64,
        channel_id: u64,
        uuid: Uuid,
    ) -> bool;
    async fn add_war_message(
        &self,
        guild_id: u64,
        channel_id: u64,
        message_id: u64,
        uuid: Uuid,
        date: &str,
        time: &str,
        server: &str,
        faction: &str,
        territory: &str,
        title: &str,
        r#type: u8,
    );
    async fn create_alert_connector(
        &self,
        uuid: Uuid,
        date: &str,
        time: &str,
        server: &str,
        faction: &str,
        territory: &str,
        title: &str,
        r#type: u8,
        guild_id: u64,
        channel_id: u64,
        message_id: u64,
    ) -> AlertConnector;
    async fn update_main_hand(&self, user_id: u64, weapon: Weapon);
    async fn update_secondary(&self, user_id: u64, weapon: Weapon);
    async fn update_level(&self, user_id: u64, level: u8);
    async fn update_gear_score(&self, user_id: u64, gear_score: u16);
    async fn update_trade_skill(&self, user_id: u64, level: u8, skill: TradeSkill);
    async fn update_weapon_level(&self, user_id: u64, level: u8, weapon: Weapon);
    async fn update_username(&self, user_id: u64, username: &str);
    async fn add_tank(&self, uuid: Uuid, user_id: u64);
    async fn add_mdps(&self, uuid: Uuid, user_id: u64);
    async fn add_prdps(&self, uuid: Uuid, user_id: u64);
    async fn add_erdps(&self, uuid: Uuid, user_id: u64);
    async fn add_healer(&self, uuid: Uuid, user_id: u64);
    async fn add_artillery(&self, uuid: Uuid, user_id: u64);
    async fn add_tentative(&self, uuid: Uuid, user_id: u64);
    async fn add_not_available(&self, uuid: Uuid, user_id: u64);
    async fn remove_tank(&self, uuid: Uuid, user_id: u64);
    async fn remove_mdps(&self, uuid: Uuid, user_id: u64);
    async fn remove_prdps(&self, uuid: Uuid, user_id: u64);
    async fn remove_erdps(&self, uuid: Uuid, user_id: u64);
    async fn remove_healer(&self, uuid: Uuid, user_id: u64);
    async fn remove_artillery(&self, uuid: Uuid, user_id: u64);
    async fn remove_tentative(&self, uuid: Uuid, user_id: u64);
    async fn remove_not_available(&self, uuid: Uuid, user_id: u64);
}

#[async_trait]
impl DBManager for mongodb::Client {
    async fn get_alert_connector(&self, uuid: Uuid) -> Option<AlertConnector> {
        self.database("warhelperDB")
            .collection::<AlertConnector>("AlertConnectors")
            .find_one(doc! {"code": uuid.to_string()}, None)
            .await
            .expect("Failed to get AlertConnectors collection")
    }

    async fn get_alert_connectors(&self) -> Vec<AlertConnector> {
        let mut connectors = vec![];
        let mut results = self
            .database("warhelperDB")
            .collection::<AlertConnector>("AlertConnectors")
            .find(None, None)
            .await
            .expect("Failed to get alert connectors");

        while let Some(ac) = results
            .try_next()
            .await
            .expect("Failed to get next alert connector from cursor")
        {
            connectors.push(ac);
        }
        connectors
    }

    async fn get_alert_connectors_with_user_id(&self, user_id: u64) -> Vec<AlertConnector> {
        self.get_alert_connectors()
            .await
            .into_iter()
            .filter(|ac| ac.get_users().contains(&user_id))
            .collect()
    }

    async fn get_user_data(&self, user_id: u64) -> Option<UserData> {
        let data = self
            .database("warhelperDB")
            .collection::<Document>("UserData")
            .find_one(doc! {format!("{}", user_id): { "$exists": true } }, None)
            .await
            .expect("Failed to get UserData collection");

        data.map(|data| {
            bson::from_bson(data.get(format!("{}", user_id)).unwrap().clone())
                .expect("Failed to parse user data")
        })
    }

    async fn has_permission(&self, guild_id: u64, roles: &[RoleId]) -> bool {
        let filter = doc! {  format!("{}", guild_id): { "$exists": true } };
        let perm = self
            .database("warhelperDB")
            .collection::<Document>("Permissions")
            .find_one(filter, None)
            .await
            .unwrap();

        if let Some(entry) = perm {
            if let Some(arr) = entry.get(format!("{}", guild_id)) {
                if let Ok(arr) = bson::from_bson::<Vec<u64>>(arr.clone()) {
                    return roles.iter().any(|r| arr.contains(&r.0));
                }
            }
        }

        false
    }

    async fn add_permission(&self, guild_id: u64, role_id: u64) {
        self.database("warhelperDB")
            .collection::<Document>("Permissions")
            .update_one(
                doc! {
                format!("{}", guild_id): { "$exists": true }},
                doc! { "$addToSet": { format!("{}", guild_id): bson::to_bson(&role_id).unwrap() } },
                {
                    let mut options = UpdateOptions::default();
                    options.upsert = Some(true);
                    options
                },
            )
            .await
            .expect("Failed to add permissions");
    }

    async fn remove_permission(&self, guild_id: u64, role_id: u64) {
        self.database("warhelperDB")
            .collection::<Document>("Permissions")
            .update_one(
                doc! {
                format!("{}", guild_id): { "$exists": true }},
                doc! { "$pull": { format!("{}", guild_id): bson::to_bson(&role_id).unwrap() } },
                None,
            )
            .await
            .expect("Failed to add permissions");
    }

    async fn channel_contains_war_message(
        &self,
        guild_id: u64,
        channel_id: u64,
        uuid: Uuid,
    ) -> bool {
        if let Some(ac) = self.get_alert_connector(uuid).await {
            ac.channel_contains_war_message(guild_id, channel_id)
        } else {
            false
        }
    }

    async fn add_war_message(
        &self,
        guild_id: u64,
        channel_id: u64,
        message_id: u64,
        uuid: Uuid,
        date: &str,
        time: &str,
        server: &str,
        faction: &str,
        territory: &str,
        title: &str,
        r#type: u8,
    ) {
        if self.get_alert_connector(uuid).await.is_some() {
            self.database("warhelperDB").collection::<AlertConnector>("AlertConnectors")
                .update_one(doc!{ "code": uuid.to_string() }
                            , doc!{
                        "$addToSet": {
                            "warMessages":
                            bson::to_bson(&WarMessage::new(guild_id, channel_id, message_id)).unwrap()
                        }
                    }, None)
                .await.expect("Failed to update Alert Connector with War Message");
        } else {
            self.create_alert_connector(
                uuid, date, time, server, faction, territory, title, r#type, guild_id, channel_id,
                message_id,
            )
            .await;
        };
    }

    async fn create_alert_connector(
        &self,
        uuid: Uuid,
        date: &str,
        time: &str,
        server: &str,
        faction: &str,
        territory: &str,
        title: &str,
        r#type: u8,
        guild_id: u64,
        channel_id: u64,
        message_id: u64,
    ) -> AlertConnector {
        let ac = AlertConnector {
            code: uuid.to_string(),
            date: date.to_string(),
            time: time.to_string(),
            server: server.to_string(),
            faction: faction.to_string(),
            territory: territory.to_string(),
            title: title.to_string(),
            r#type,
            tanks: Default::default(),
            erdps: Default::default(),
            prdps: Default::default(),
            mdps: Default::default(),
            healers: Default::default(),
            tentative: Default::default(),
            not_available: Default::default(),
            artillery: Default::default(),
            war_messages: vec![WarMessage::new(guild_id, channel_id, message_id)],
        };
        self.database("warhelperDB")
            .collection::<AlertConnector>("AlertConnectors")
            .insert_one(ac.clone(), None)
            .await
            .expect("Failed to insert alert connector");
        ac
    }

    async fn update_main_hand(&self, user_id: u64, weapon: Weapon) {
        self.database("warhelperDB")
            .collection::<Document>("UserData")
            .update_one(
                doc! {
                    format!("{}", user_id): {
                        "$exists": true
                    }
                },
                doc! {
                    "$set": {
                        format!("{}.mainHand", user_id): <Weapon as Into<String>>::into(weapon)
                    }
                },
                {
                    let mut options = UpdateOptions::default();
                    options.upsert = Some(true);
                    options
                },
            )
            .await
            .expect("Failed to update user data main hand");
    }

    async fn update_secondary(&self, user_id: u64, weapon: Weapon) {
        self.database("warhelperDB")
            .collection::<Document>("UserData")
            .update_one(
                doc! {
                    format!("{}", user_id): {
                        "$exists": true
                    }
                },
                doc! {
                    "$set": {
                        format!("{}.secondary", user_id): <Weapon as Into<String>>::into(weapon)
                    }
                },
                {
                    let mut options = UpdateOptions::default();
                    options.upsert = Some(true);
                    options
                },
            )
            .await
            .expect("Failed to update user data secondary");
    }

    async fn update_level(&self, user_id: u64, level: u8) {
        self.database("warhelperDB")
            .collection::<Document>("UserData")
            .update_one(
                doc! {
                    format!("{}", user_id): {
                        "$exists": true
                    }
                },
                doc! {
                    "$set": {
                        format!("{}.level", user_id): bson::to_bson(&level).unwrap()
                    }
                },
                {
                    let mut options = UpdateOptions::default();
                    options.upsert = Some(true);
                    options
                },
            )
            .await
            .expect("Failed to update user data level");
    }

    async fn update_gear_score(&self, user_id: u64, gear_score: u16) {
        self.database("warhelperDB")
            .collection::<Document>("UserData")
            .update_one(
                doc! {
                    format!("{}", user_id): {
                        "$exists": true
                    }
                },
                doc! {
                    "$set": {
                        format!("{}.gearScore", user_id): bson::to_bson(&gear_score).unwrap()
                    }
                },
                {
                    let mut options = UpdateOptions::default();
                    options.upsert = Some(true);
                    options
                },
            )
            .await
            .expect("Failed to update user data gear score");
    }

    async fn update_trade_skill(&self, user_id: u64, level: u8, skill: TradeSkill) {
        self.database("warhelperDB")
            .collection::<Document>("UserData")
            .update_one(
                doc! {
                    format!("{}", user_id): {
                        "$exists": true
                    }
                },
                doc! {
                    "$set": {
                        format!("{}.tradeSkills.{}", user_id, <TradeSkill as Into<String>>::into(skill)): bson::to_bson(&level).unwrap()
                    }
                },
                {
                    let mut options = UpdateOptions::default();
                    options.upsert = Some(true);
                    options
                },
            )
            .await
            .expect("Failed to update user data trade skill");
    }

    async fn update_weapon_level(&self, user_id: u64, level: u8, weapon: Weapon) {
        self.database("warhelperDB")
            .collection::<Document>("UserData")
            .update_one(
                doc! {
                    format!("{}", user_id): {
                        "$exists": true
                    }
                },
                doc! {
                    "$set": {
                        format!("{}.weapons.{}", user_id, <Weapon as Into<String>>::into(weapon)): bson::to_bson(&level).unwrap()
                    }
                },
                {
                    let mut options = UpdateOptions::default();
                    options.upsert = Some(true);
                    options
                },
            )
            .await
            .expect("Failed to update user data weapon level");
    }

    async fn update_username(&self, user_id: u64, username: &str) {
        self.database("warhelperDB")
            .collection::<Document>("UserData")
            .update_one(
                doc! {
                    format!("{}", user_id): {
                        "$exists": true
                    }
                },
                doc! {
                    "$set": {
                        format!("{}.username", user_id): format!("{}", username)
                    }
                },
                {
                    let mut options = UpdateOptions::default();
                    options.upsert = Some(true);
                    options
                },
            )
            .await
            .expect("Failed to update username");
    }

    async fn add_tank(&self, uuid: Uuid, user_id: u64) {
        self.database("warhelperDB")
            .collection::<AlertConnector>("AlertConnectors")
            .update_one(
                doc! {
                    "code": format!("{}", uuid)
                },
                doc! {
                    "$addToSet": {
                        "tanks": bson::to_bson(&user_id).unwrap()
                    }
                },
                {
                    let mut options = UpdateOptions::default();
                    options.upsert = Some(true);
                    options
                },
            )
            .await
            .expect("Failed to update tank list in alert connector");
    }

    async fn add_mdps(&self, uuid: Uuid, user_id: u64) {
        self.database("warhelperDB")
            .collection::<AlertConnector>("AlertConnectors")
            .update_one(
                doc! {
                    "code": format!("{}", uuid)
                },
                doc! {
                    "$addToSet": {
                        "mdps": bson::to_bson(&user_id).unwrap()
                    }
                },
                {
                    let mut options = UpdateOptions::default();
                    options.upsert = Some(true);
                    options
                },
            )
            .await
            .expect("Failed to update mdps list in alert connector");
    }

    async fn add_prdps(&self, uuid: Uuid, user_id: u64) {
        self.database("warhelperDB")
            .collection::<AlertConnector>("AlertConnectors")
            .update_one(
                doc! {
                    "code": format!("{}", uuid)
                },
                doc! {
                    "$addToSet": {
                        "prdps": bson::to_bson(&user_id).unwrap()
                    }
                },
                {
                    let mut options = UpdateOptions::default();
                    options.upsert = Some(true);
                    options
                },
            )
            .await
            .expect("Failed to update prdps list in alert connector");
    }

    async fn add_erdps(&self, uuid: Uuid, user_id: u64) {
        self.database("warhelperDB")
            .collection::<AlertConnector>("AlertConnectors")
            .update_one(
                doc! {
                    "code": format!("{}", uuid)
                },
                doc! {
                    "$addToSet": {
                        "erdps": bson::to_bson(&user_id).unwrap()
                    }
                },
                {
                    let mut options = UpdateOptions::default();
                    options.upsert = Some(true);
                    options
                },
            )
            .await
            .expect("Failed to update erdps list in alert connector");
    }

    async fn add_healer(&self, uuid: Uuid, user_id: u64) {
        self.database("warhelperDB")
            .collection::<AlertConnector>("AlertConnectors")
            .update_one(
                doc! {
                    "code": format!("{}", uuid)
                },
                doc! {
                    "$addToSet": {
                        "healers": bson::to_bson(&user_id).unwrap()
                    }
                },
                {
                    let mut options = UpdateOptions::default();
                    options.upsert = Some(true);
                    options
                },
            )
            .await
            .expect("Failed to update healer list in alert connector");
    }

    async fn add_artillery(&self, uuid: Uuid, user_id: u64) {
        self.database("warhelperDB")
            .collection::<AlertConnector>("AlertConnectors")
            .update_one(
                doc! {
                    "code": format!("{}", uuid)
                },
                doc! {
                    "$addToSet": {
                        "artillery": bson::to_bson(&user_id).unwrap()
                    }
                },
                {
                    let mut options = UpdateOptions::default();
                    options.upsert = Some(true);
                    options
                },
            )
            .await
            .expect("Failed to update artillery list in alert connector");
    }

    async fn add_tentative(&self, uuid: Uuid, user_id: u64) {
        self.database("warhelperDB")
            .collection::<AlertConnector>("AlertConnectors")
            .update_one(
                doc! {
                    "code": format!("{}", uuid)
                },
                doc! {
                    "$addToSet": {
                        "tentative": bson::to_bson(&user_id).unwrap()
                    }
                },
                {
                    let mut options = UpdateOptions::default();
                    options.upsert = Some(true);
                    options
                },
            )
            .await
            .expect("Failed to update tentative list in alert connector");
    }

    async fn add_not_available(&self, uuid: Uuid, user_id: u64) {
        self.database("warhelperDB")
            .collection::<AlertConnector>("AlertConnectors")
            .update_one(
                doc! {
                    "code": format!("{}", uuid)
                },
                doc! {
                    "$addToSet": {
                        "notAvailable": bson::to_bson(&user_id).unwrap()
                    }
                },
                {
                    let mut options = UpdateOptions::default();
                    options.upsert = Some(true);
                    options
                },
            )
            .await
            .expect("Failed to update not available list in alert connector");
    }

    async fn remove_tank(&self, uuid: Uuid, user_id: u64) {
        self.database("warhelperDB")
            .collection::<AlertConnector>("AlertConnectors")
            .update_one(
                doc! {
                    "code": format!("{}", uuid)
                },
                doc! {
                    "$pull": {
                        "tanks": bson::to_bson(&user_id).unwrap()
                    }
                },
                None,
            )
            .await
            .expect("Failed to update tank list in alert connector");
    }

    async fn remove_mdps(&self, uuid: Uuid, user_id: u64) {
        self.database("warhelperDB")
            .collection::<AlertConnector>("AlertConnectors")
            .update_one(
                doc! {
                    "code": format!("{}", uuid)
                },
                doc! {
                    "$pull": {
                        "mdps": bson::to_bson(&user_id).unwrap()
                    }
                },
                None,
            )
            .await
            .expect("Failed to update mdps list in alert connector");
    }

    async fn remove_prdps(&self, uuid: Uuid, user_id: u64) {
        self.database("warhelperDB")
            .collection::<AlertConnector>("AlertConnectors")
            .update_one(
                doc! {
                    "code": format!("{}", uuid)
                },
                doc! {
                    "$pull": {
                        "prdps": bson::to_bson(&user_id).unwrap()
                    }
                },
                None,
            )
            .await
            .expect("Failed to update prdps list in alert connector");
    }

    async fn remove_erdps(&self, uuid: Uuid, user_id: u64) {
        self.database("warhelperDB")
            .collection::<AlertConnector>("AlertConnectors")
            .update_one(
                doc! {
                    "code": format!("{}", uuid)
                },
                doc! {
                    "$pull": {
                        "erdps": bson::to_bson(&user_id).unwrap()
                    }
                },
                None,
            )
            .await
            .expect("Failed to update erdps list in alert connector");
    }

    async fn remove_healer(&self, uuid: Uuid, user_id: u64) {
        self.database("warhelperDB")
            .collection::<AlertConnector>("AlertConnectors")
            .update_one(
                doc! {
                    "code": format!("{}", uuid)
                },
                doc! {
                    "$pull": {
                        "healers": bson::to_bson(&user_id).unwrap()
                    }
                },
                None,
            )
            .await
            .expect("Failed to update healer list in alert connector");
    }

    async fn remove_artillery(&self, uuid: Uuid, user_id: u64) {
        self.database("warhelperDB")
            .collection::<AlertConnector>("AlertConnectors")
            .update_one(
                doc! {
                    "code": format!("{}", uuid)
                },
                doc! {
                    "$pull": {
                        "artillery": bson::to_bson(&user_id).unwrap()
                    }
                },
                None,
            )
            .await
            .expect("Failed to update artillery list in alert connector");
    }

    async fn remove_tentative(&self, uuid: Uuid, user_id: u64) {
        self.database("warhelperDB")
            .collection::<AlertConnector>("AlertConnectors")
            .update_one(
                doc! {
                    "code": format!("{}", uuid)
                },
                doc! {
                    "$pull": {
                        "tentative": bson::to_bson(&user_id).unwrap()
                    }
                },
                None,
            )
            .await
            .expect("Failed to update tentative list in alert connector");
    }

    async fn remove_not_available(&self, uuid: Uuid, user_id: u64) {
        self.database("warhelperDB")
            .collection::<AlertConnector>("AlertConnectors")
            .update_one(
                doc! {
                    "code": format!("{}", uuid)
                },
                doc! {
                    "$pull": {
                        "notAvailable": bson::to_bson(&user_id).unwrap()
                    }
                },
                None,
            )
            .await
            .expect("Failed to update not available list in alert connector");
    }
}
