use crate::{AlertConnector, DBManager};
use serenity::builder::CreateEmbed;
use serenity::model::prelude::{ChannelId, ReactionType};
use serenity::prelude::Context;
use uuid::Uuid;

pub const REACTIONS: [char; 8] = ['🛡', '🗡', '🏹', '🪄', '❤', '💥', '❓', '⛔'];

pub fn convert_to_emoji(s: &str) -> String {
    let mut result = String::new();
    for char in s.chars() {
        if char == '_' {
            result = format!("{}    ", result);
        } else {
            result = format!(
                "{}{} ",
                result,
                char::from_u32('🇦' as u32 + (char as u32 - 97)).unwrap()
            );
        }
    }

    result
}

pub async fn update_embeds(uuid: Uuid, ctx: &Context, db_client: &mongodb::Client) {
    if let Some(ac) = db_client.get_alert_connector(uuid).await {
        for war_message in &ac.war_messages {
            if let Ok(guild) = ctx.http.get_guild(war_message.get_guild_id()).await {
                if let Ok(channels) = guild.channels(ctx).await {
                    if let Some(channel) = channels.get(&ChannelId(war_message.get_channel_id())) {
                        if let Ok(mut message) =
                            channel.message(ctx, war_message.get_message_id()).await
                        {
                            if let Some(embed) = message.embeds.get(0) {
                                let mut new_embed = CreateEmbed::default();

                                new_embed
                                    .title(embed.title.as_ref().unwrap())
                                    .description(embed.description.as_ref().unwrap())
                                    .fields(
                                        embed.fields[0..3]
                                            .iter()
                                            .map(|e| (&e.name, &e.value, e.inline)),
                                    );

                                fill_embed(&mut new_embed, &ac, db_client).await;

                                let note = embed.fields.last().unwrap();
                                new_embed.field(&note.name, &note.value, note.inline);

                                new_embed.footer(|f| {
                                    let footer = embed.footer.as_ref().unwrap();
                                    f.text(&footer.text)
                                });

                                if let Err(why) =
                                    message.edit(ctx, |m| m.set_embed(new_embed)).await
                                {
                                    println!("Failed to update embed: {}", why);
                                }

                                for reaction in REACTIONS {
                                    if !message.reactions.iter().any(|r| {
                                        if let ReactionType::Unicode(emoji) = &r.reaction_type {
                                            *emoji == reaction.to_string()
                                        } else {
                                            false
                                        }
                                    }) {
                                        message
                                            .react(ctx, reaction)
                                            .await
                                            .expect("Failed to add reaction");
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub async fn fill_embed(embed: &mut CreateEmbed, ac: &AlertConnector, db_client: &mongodb::Client) {
    let mut tanks = String::new();
    let mut erdps = String::new();
    let mut prdps = String::new();
    let mut mdps = String::new();
    let mut healers = String::new();
    let mut tentative = String::new();
    let mut not_available = String::new();
    let mut artillery = String::new();

    fill_string_from_list(&mut tanks, &ac.tanks, db_client).await;
    fill_string_from_list(&mut erdps, &ac.erdps, db_client).await;
    fill_string_from_list(&mut prdps, &ac.prdps, db_client).await;
    fill_string_from_list(&mut mdps, &ac.mdps, db_client).await;
    fill_string_from_list(&mut healers, &ac.healers, db_client).await;
    fill_secondary_string_from_list(&mut tentative, &ac.tentative, db_client).await;
    fill_secondary_string_from_list(&mut not_available, &ac.not_available, db_client).await;
    fill_string_from_list(&mut artillery, &ac.artillery, db_client).await;

    embed
        .field(
            ":shield: TANK :shield:",
            if tanks.is_empty() {
                "\u{200B}"
            } else {
                tanks.trim()
            },
            true,
        )
        .field("\u{200B}", "\u{200B}", true)
        .field(
            ":dagger: MDPS :dagger:",
            if mdps.is_empty() {
                "\u{200B}"
            } else {
                mdps.trim()
            },
            true,
        )
        .field("\u{200B}", "\u{200B}", false)
        .field(
            ":archery: Physical RDPS :archery:",
            if prdps.is_empty() {
                "\u{200B}"
            } else {
                prdps.trim()
            },
            true,
        )
        .field("\u{200B}", "\u{200B}", true)
        .field(
            ":magic_wand: Elemental RDPS :magic_wand:",
            if erdps.is_empty() {
                "\u{200B}"
            } else {
                erdps.trim()
            },
            true,
        )
        .field("\u{200B}", "\u{200B}", false)
        .field(
            ":heart: Healer :heart:",
            if healers.is_empty() {
                "\u{200B}"
            } else {
                healers.trim()
            },
            true,
        )
        .field("\u{200B}", "\u{200B}", true)
        .field(
            ":boom: Artillery :boom:",
            if artillery.is_empty() {
                "\u{200B}"
            } else {
                artillery.trim()
            },
            true,
        )
        .field("\u{200B}", "\u{200B}", false)
        .field(
            ":question: Tentative :question:",
            if tentative.is_empty() {
                "\u{200B}"
            } else {
                tentative.trim()
            },
            true,
        )
        .field("\u{200B}", "\u{200B}", true)
        .field(
            ":no_entry: Not Available :no_entry:",
            if not_available.is_empty() {
                "\u{200B}"
            } else {
                not_available.trim()
            },
            true,
        )
        .field("\u{200B}", "\u{200B}", false);
}

async fn fill_string_from_list(string: &mut String, list: &[u64], db_client: &mongodb::Client) {
    for &id in list {
        if let Some(user_data) = db_client.get_user_data(id).await {
            if user_data.get_username().is_empty() {
                continue;
            }
            *string = format!(
                "{}`{:0>3}`{}`{} {},{} {}`\n",
                string,
                user_data.get_gear_score(),
                user_data.get_username(),
                user_data.get_main_hand_level(),
                if let Some(weapon) = user_data.get_main_hand() {
                    weapon.get_abbreviation()
                } else {
                    "N/A"
                },
                user_data.get_secondary_level(),
                if let Some(weapon) = user_data.get_secondary() {
                    weapon.get_abbreviation()
                } else {
                    "N/A"
                }
            );
        }
    }
}

async fn fill_secondary_string_from_list(
    string: &mut String,
    list: &[u64],
    db_client: &mongodb::Client,
) {
    for &id in list {
        if let Some(user_data) = db_client.get_user_data(id).await {
            if user_data.get_username().is_empty() {
                continue;
            }
            *string = format!(
                "{}`{:0>3}`{}\n",
                string,
                user_data.get_gear_score(),
                user_data.get_username(),
            );
        }
    }
}
