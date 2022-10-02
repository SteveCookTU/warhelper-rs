use crate::db_manager::DBManager;
use crate::trade_skill::TradeSkill;
use crate::util::{convert_to_emoji, fill_embed, REACTIONS};
use crate::weapon::Weapon;
use crate::{util, DBHandler};
use chrono::NaiveDateTime;
use serenity::builder::CreateEmbed;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::{ChannelId, GuildId};
use serenity::prelude::Context;
use std::str::FromStr;
use uuid::Uuid;

pub async fn handle_war_command(
    ctx: &mut Context,
    command: &ApplicationCommandInteraction,
) -> Option<&'static str> {
    if let Some(sub_command) = command.data.options.get(0) {
        if sub_command.kind == CommandOptionType::SubCommand {
            match sub_command.name.as_str() {
                "alert" => {
                    if let Some(guild_id) = command.guild_id {
                        if let Some(member) = command.member.as_ref() {
                            let db_client = ctx
                                .data
                                .read()
                                .await
                                .get::<DBHandler>()
                                .expect("Failed to get db handler")
                                .clone();
                            if db_client.has_permission(guild_id.0, &member.roles).await {
                                if let CommandDataOptionValue::String(server_str) = sub_command
                                    .options
                                    .get(0)
                                    .expect("Failed to get date string")
                                    .resolved
                                    .as_ref()
                                    .expect("Failed to resolve date string")
                                {
                                    if let CommandDataOptionValue::String(faction_str) = sub_command
                                        .options
                                        .get(1)
                                        .expect("Failed to get faction string")
                                        .resolved
                                        .as_ref()
                                        .expect("Failed to resolved faction string")
                                    {
                                        if let CommandDataOptionValue::String(territory_str) =
                                            sub_command
                                                .options
                                                .get(2)
                                                .expect("Failed to get territory string")
                                                .resolved
                                                .as_ref()
                                                .expect("Failed to resolve territory string")
                                        {
                                            if let CommandDataOptionValue::String(date_str) =
                                                sub_command
                                                    .options
                                                    .get(3)
                                                    .expect("Failed to get date string")
                                                    .resolved
                                                    .as_ref()
                                                    .expect("Failed to resolve date string")
                                            {
                                                if let CommandDataOptionValue::String(time_str) =
                                                    sub_command
                                                        .options
                                                        .get(4)
                                                        .expect("Failed to get time string")
                                                        .resolved
                                                        .as_ref()
                                                        .expect("Failed to resolve time string")
                                                {
                                                    if let Err(str) = create_alert(
                                                        ctx,
                                                        &format!("{}@{}", date_str, time_str),
                                                        guild_id,
                                                        command.channel_id,
                                                        territory_str,
                                                        Some(server_str),
                                                        Some(faction_str),
                                                        &db_client,
                                                        None,
                                                    )
                                                    .await
                                                    {
                                                        println!("Failed to create alert: {}", str);
                                                    } else {
                                                        return Some("Alert has been created.");
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                Some("Error resolving command input")
                            } else {
                                Some("You do not have permission to use this command.")
                            }
                        } else {
                            Some("This command can only be used in guilds.")
                        }
                    } else {
                        Some("This command can only be used in guilds.")
                    }
                }
                "refresh" => {
                    if let Some(guild_id) = command.guild_id {
                        if let Some(member) = command.member.as_ref() {
                            let db_client = ctx
                                .data
                                .read()
                                .await
                                .get::<DBHandler>()
                                .expect("Failed to get db handler")
                                .clone();
                            if db_client.has_permission(guild_id.0, &member.roles).await {
                                if let CommandDataOptionValue::String(str) = sub_command
                                    .options
                                    .get(0)
                                    .unwrap()
                                    .resolved
                                    .as_ref()
                                    .unwrap()
                                {
                                    let uuid = Uuid::from_str(str).unwrap();
                                    refresh_embeds(ctx, uuid, &db_client).await;
                                }
                                Some("All embeds with the ID specified have been refreshed.")
                            } else {
                                Some("You do not have permission to use this command.")
                            }
                        } else {
                            Some("This command can only be used in guilds.")
                        }
                    } else {
                        Some("This command can only be used in guilds.")
                    }
                }
                "perm" => {
                    if let Some(guild_id) = command.guild_id {
                        if let Some(member) = command.member.as_ref() {
                            if let Some(permissions) = member.permissions {
                                if permissions.administrator() {
                                    let db_client = ctx
                                        .data
                                        .read()
                                        .await
                                        .get::<DBHandler>()
                                        .expect("Failed to get db handler")
                                        .clone();
                                    if let CommandDataOptionValue::String(option) = sub_command
                                        .options
                                        .get(0)
                                        .unwrap()
                                        .resolved
                                        .as_ref()
                                        .unwrap()
                                    {
                                        if let CommandDataOptionValue::Role(role) = sub_command
                                            .options
                                            .get(1)
                                            .unwrap()
                                            .resolved
                                            .as_ref()
                                            .unwrap()
                                        {
                                            match option.as_str() {
                                                "add" => {
                                                    db_client
                                                        .add_permission(guild_id.0, role.id.0)
                                                        .await;
                                                    Some("The bot admin permission has been added to the specified role.")
                                                }
                                                "remove" => {
                                                    db_client
                                                        .remove_permission(guild_id.0, role.id.0)
                                                        .await;
                                                    Some("The bot admin permission has been removed from the specified role.")
                                                }
                                                _ => None,
                                            }
                                        } else {
                                            Some("Invalid Role")
                                        }
                                    } else {
                                        Some("Invalid Edit Option")
                                    }
                                } else {
                                    Some("You do not have permission to use this command.")
                                }
                            } else {
                                Some("You do not have permission to use this command.")
                            }
                        } else {
                            Some("This command can only be used in guilds.")
                        }
                    } else {
                        Some("This command can only be used in guilds.")
                    }
                }
                _ => Some("Invalid Command"),
            }
        } else {
            Some("Invalid Option Type")
        }
    } else {
        Some("Invalid Subcommand")
    }
}

pub async fn handle_register_command(
    ctx: &mut Context,
    command: &ApplicationCommandInteraction,
) -> Option<String> {
    if let Some(sub_command) = command.data.options.get(0) {
        if sub_command.kind == CommandOptionType::SubCommand {
            let db_client = ctx
                .data
                .read()
                .await
                .get::<DBHandler>()
                .expect("Failed to get DB Client")
                .clone();
            match sub_command.name.as_str() {
                "mainhand" => {
                    if let CommandDataOptionValue::String(weapon_str) = sub_command
                        .options
                        .get(0)
                        .unwrap()
                        .resolved
                        .as_ref()
                        .unwrap()
                    {
                        let weapon = Weapon::try_from(weapon_str.to_string()).unwrap();
                        db_client.update_main_hand(command.user.id.0, weapon).await;
                        update_all_embeds(ctx, command.user.id.0, &db_client).await;
                        Some(format!("Main hand set to {}", weapon.get_label()))
                    } else {
                        Some("Invalid input for weapon".to_string())
                    }
                }
                "secondary" => {
                    if let CommandDataOptionValue::String(weapon_str) = sub_command
                        .options
                        .get(0)
                        .unwrap()
                        .resolved
                        .as_ref()
                        .unwrap()
                    {
                        let weapon = Weapon::try_from(weapon_str.to_string()).unwrap();
                        db_client.update_secondary(command.user.id.0, weapon).await;
                        update_all_embeds(ctx, command.user.id.0, &db_client).await;
                        Some(format!("Secondary set to {}", weapon.get_label()))
                    } else {
                        Some("Invalid input for weapon".to_string())
                    }
                }
                "level" => {
                    if let &CommandDataOptionValue::Integer(level) = sub_command
                        .options
                        .get(0)
                        .unwrap()
                        .resolved
                        .as_ref()
                        .unwrap()
                    {
                        if (1..61).contains(&level) {
                            db_client.update_level(command.user.id.0, level as u8).await;
                            Some(format!("Level set to {}", level))
                        } else {
                            Some("Please enter a level from 1 to 60 (inclusive).".to_string())
                        }
                    } else {
                        Some("Invalid input for level".to_string())
                    }
                }
                "gearscore" => {
                    if let &CommandDataOptionValue::Integer(gs) = sub_command
                        .options
                        .get(0)
                        .unwrap()
                        .resolved
                        .as_ref()
                        .unwrap()
                    {
                        if (0..626).contains(&gs) {
                            db_client
                                .update_gear_score(command.user.id.0, gs as u16)
                                .await;
                            update_all_embeds(ctx, command.user.id.0, &db_client).await;
                            Some(format!("Gear score set to {}", gs))
                        } else {
                            Some("Please enter a level from 0 to 625 (inclusive).".to_string())
                        }
                    } else {
                        Some("Invalid input for gear score".to_string())
                    }
                }
                "tradeskill" => {
                    if let CommandDataOptionValue::String(skill) = sub_command
                        .options
                        .get(0)
                        .unwrap()
                        .resolved
                        .as_ref()
                        .unwrap()
                    {
                        if let Ok(skill) = TradeSkill::try_from(skill.to_string()) {
                            if let &CommandDataOptionValue::Integer(level) = sub_command
                                .options
                                .get(1)
                                .unwrap()
                                .resolved
                                .as_ref()
                                .unwrap()
                            {
                                if (0..201).contains(&level) {
                                    db_client
                                        .update_trade_skill(command.user.id.0, level as u8, skill)
                                        .await;
                                    Some(format!("{} set to {}", skill.get_label(), level))
                                } else {
                                    Some(
                                        "Please enter a level from 0 to 200 (inclusive)."
                                            .to_string(),
                                    )
                                }
                            } else {
                                Some("Invalid input for level".to_string())
                            }
                        } else {
                            println!("{}", skill);
                            Some("Invalid input for trade skill".to_string())
                        }
                    } else {
                        Some("Invalid input for trade skill".to_string())
                    }
                }
                "weaponlevel" => {
                    if let CommandDataOptionValue::String(weapon) = sub_command
                        .options
                        .get(0)
                        .unwrap()
                        .resolved
                        .as_ref()
                        .unwrap()
                    {
                        if let Ok(weapon) = Weapon::try_from(weapon.to_string()) {
                            if let &CommandDataOptionValue::Integer(level) = sub_command
                                .options
                                .get(1)
                                .unwrap()
                                .resolved
                                .as_ref()
                                .unwrap()
                            {
                                if (0..21).contains(&level) {
                                    db_client
                                        .update_weapon_level(command.user.id.0, level as u8, weapon)
                                        .await;
                                    update_all_embeds(ctx, command.user.id.0, &db_client).await;
                                    Some(format!("{} set to {}", weapon.get_label(), level))
                                } else {
                                    Some(
                                        "Please enter a level from 0 to 20 (inclusive)."
                                            .to_string(),
                                    )
                                }
                            } else {
                                Some("Invalid input for level".to_string())
                            }
                        } else {
                            Some("Invalid input for weapon".to_string())
                        }
                    } else {
                        Some("Invalid input for weapon".to_string())
                    }
                }
                "username" => {
                    if let CommandDataOptionValue::String(username) = sub_command
                        .options
                        .get(0)
                        .unwrap()
                        .resolved
                        .as_ref()
                        .unwrap()
                    {
                        db_client.update_username(command.user.id.0, username).await;
                        update_all_embeds(ctx, command.user.id.0, &db_client).await;
                        Some(format!("Username set to {}", username))
                    } else {
                        Some("Invalid input for username".to_string())
                    }
                }
                _ => Some("Invalid Subcommand".to_string()),
            }
        } else {
            Some("Invalid Option Type".to_string())
        }
    } else {
        Some("Invalid Subcommand".to_string())
    }
}

async fn update_all_embeds(ctx: &Context, user_id: u64, db_client: &mongodb::Client) {
    for ac in db_client.get_alert_connectors_with_user_id(user_id).await {
        util::update_embeds(Uuid::from_str(&ac.code).unwrap(), ctx, db_client).await;
    }
}

async fn create_alert(
    ctx: &Context,
    date_time: &str,
    guild: GuildId,
    channel_id: ChannelId,
    territory: &str,
    server: Option<&str>,
    faction: Option<&str>,
    db_client: &mongodb::Client,
    name: Option<&str>,
) -> Result<(), &'static str> {
    let date_time = NaiveDateTime::parse_from_str(&date_time.to_lowercase(), "%m/%e/%Y@%I:%M%P")
        .map_err(|_| {
            "The date or time entered was invalid. \
            Please use the formats mm/dd/YYYY and HH:MMP respectively. Ex: 02/10/2022 and 12:30pm"
        })?;

    let server = if let Some(server) = server {
        server.to_string()
    } else {
        format!("localevent{}", guild.0)
    };
    let faction = if let Some(faction) = faction {
        faction
    } else {
        "event"
    };
    let date_str = date_time.format("%a %e. %b").to_string();
    let time_str = date_time.format("%H:%M%P").to_string();

    let uuid = Uuid::new_v5(
        &Uuid::NAMESPACE_OID,
        format!(
            "{}{}{}{}{}",
            date_str,
            time_str,
            server.to_lowercase(),
            faction.to_lowercase(),
            territory.to_lowercase()
        )
        .as_bytes(),
    );

    if !db_client
        .channel_contains_war_message(guild.0, channel_id.0, uuid)
        .await
    {
        let mut embed = CreateEmbed::default()
            .title(name.unwrap_or("War Alert"))
            .description(convert_to_emoji(territory))
            .field(format!(":calendar_spiral: {}", date_str), "\u{200B}", true)
            .field("\u{200B}", "\u{200B}", true)
            .field(format!(":clock1: {}", time_str), "\u{200B}", true)
            .to_owned();

        if let Some(ac) = db_client.get_alert_connector(uuid).await {
            fill_embed(&mut embed, &ac, db_client).await;
        } else {
            embed = embed
                .field(":shield: TANK :shield:", "\u{200B}", true)
                .field("\u{200B}", "\u{200B}", true)
                .field(":dagger: MDPS :dagger:", "\u{200B}", true)
                .field("\u{200B}", "\u{200B}", false)
                .field(":archery: Physical RDPS :archery:", "\u{200B}", true)
                .field("\u{200B}", "\u{200B}", true)
                .field(":magic_wand: Elemental RDPS :magic_wand:", "\u{200B}", true)
                .field("\u{200B}", "\u{200B}", false)
                .field(":heart: Healer :heart:", "\u{200B}", true)
                .field("\u{200B}", "\u{200B}", true)
                .field(":boom: Artillery :boom:", "\u{200B}", true)
                .field("\u{200B}", "\u{200B}", false)
                .field(":question: Tentative :question:", "\u{200B}", true)
                .field("\u{200B}", "\u{200B}", true)
                .field(":no_entry: Not Available :no_entry:", "\u{200B}", true)
                .field("\u{200B}", "\u{200B}", false)
                .to_owned();
        }
        embed = embed
            .field(
                "NOTE",
                "Remember to use '/register' to register your in-game data.",
                false,
            )
            .footer(|f| f.text(uuid.hyphenated().to_string()))
            .to_owned();

        let result = channel_id
            .send_message(&ctx, |m| {
                m.embed(|e| {
                    *e = embed;
                    e
                })
            })
            .await;
        if let Err(why) = result.as_ref() {
            println!("Failed to send message: {}", why);
        } else if let Ok(message) = result {
            for reaction in REACTIONS {
                message
                    .react(&ctx, reaction)
                    .await
                    .expect("Failed to react to embed");
            }

            db_client
                .add_war_message(
                    guild.0,
                    channel_id.0,
                    message.id.0,
                    uuid,
                    &date_str,
                    &time_str,
                    &server,
                    faction,
                    territory,
                    if let Some(name) = name { name } else { "" },
                    u8::from(name.is_some()),
                )
                .await;
        }
    }

    Ok(())
}

async fn refresh_embeds(ctx: &Context, uuid: Uuid, db_client: &mongodb::Client) {
    util::update_embeds(uuid, ctx, db_client).await;
}
