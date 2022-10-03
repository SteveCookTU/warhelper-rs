#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]
mod alert_connector;
mod command_handler;
mod db_manager;
mod trade_skill;
mod user_data;
mod util;
mod war_message;
mod weapon;

use crate::alert_connector::AlertConnector;
use crate::command_handler::{
    handle_register_command, handle_war_command, handle_war_stat_command,
};
use crate::db_manager::DBManager;
use crate::util::REACTIONS;
use crate::war_message::WarMessage;
use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use serenity::builder::CreateEmbed;
use serenity::client::{Context, EventHandler};
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::channel::Reaction;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::ReactionType;
use serenity::prelude::{GatewayIntents, TypeMapKey};
use serenity::{async_trait, Client};
use std::env;
use std::str::FromStr;
use uuid::Uuid;

struct Handler;

struct DBHandler;

impl TypeMapKey for DBHandler {
    type Value = mongodb::Client;
}

#[async_trait]
impl EventHandler for Handler {
    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        if !add_reaction.user(&ctx).await.unwrap().bot {
            if let Some(guild_id) = add_reaction.guild_id {
                if let Some(user_id) = add_reaction.user_id {
                    let db_client = ctx.data.read().await.get::<DBHandler>().unwrap().clone();
                    let embeds = add_reaction.message(&ctx).await.unwrap().embeds;
                    if let Some(embed) = embeds.first() {
                        if let Some(footer) = embed.footer.as_ref() {
                            let uuid =
                                Uuid::from_str(&footer.text.chars().take(36).collect::<String>())
                                    .unwrap();
                            if let Some(ac) = db_client.get_alert_connector(uuid).await {
                                if ac.war_messages.contains(&WarMessage::new(
                                    guild_id.0,
                                    add_reaction.channel_id.0,
                                    add_reaction.message_id.0,
                                )) && !ac.get_users().contains(&user_id.0)
                                {
                                    if let ReactionType::Unicode(emoji) = add_reaction.emoji {
                                        if let Some((i, _)) = REACTIONS
                                            .iter()
                                            .enumerate()
                                            .find(|(_, c)| c.to_string() == emoji)
                                        {
                                            match i {
                                                0 => {
                                                    db_client.add_tank(uuid, user_id.0).await;
                                                }
                                                1 => {
                                                    db_client.add_mdps(uuid, user_id.0).await;
                                                }
                                                2 => {
                                                    db_client.add_prdps(uuid, user_id.0).await;
                                                }
                                                3 => {
                                                    db_client.add_erdps(uuid, user_id.0).await;
                                                }
                                                4 => {
                                                    db_client.add_healer(uuid, user_id.0).await;
                                                }
                                                5 => {
                                                    db_client.add_artillery(uuid, user_id.0).await;
                                                }
                                                6 => {
                                                    db_client.add_tentative(uuid, user_id.0).await;
                                                }
                                                7 => {
                                                    db_client
                                                        .add_not_available(uuid, user_id.0)
                                                        .await;
                                                }
                                                _ => {}
                                            }
                                            util::update_embeds(uuid, &ctx, &db_client).await;
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

    async fn reaction_remove(&self, ctx: Context, removed_reaction: Reaction) {
        if !removed_reaction.user(&ctx).await.unwrap().bot {
            if let Some(guild_id) = removed_reaction.guild_id {
                if let Some(user_id) = removed_reaction.user_id {
                    let db_client = ctx.data.read().await.get::<DBHandler>().unwrap().clone();
                    let embeds = removed_reaction.message(&ctx).await.unwrap().embeds;
                    if let Some(embed) = embeds.first() {
                        if let Some(footer) = embed.footer.as_ref() {
                            let uuid =
                                Uuid::from_str(&footer.text.chars().take(36).collect::<String>())
                                    .unwrap();
                            if let Some(ac) = db_client.get_alert_connector(uuid).await {
                                if ac.war_messages.contains(&WarMessage::new(
                                    guild_id.0,
                                    removed_reaction.channel_id.0,
                                    removed_reaction.message_id.0,
                                )) && ac.get_users().contains(&user_id.0)
                                {
                                    if let ReactionType::Unicode(emoji) = removed_reaction.emoji {
                                        if let Some((i, _)) = REACTIONS
                                            .iter()
                                            .enumerate()
                                            .find(|(_, c)| c.to_string() == emoji)
                                        {
                                            match i {
                                                0 => {
                                                    db_client.remove_tank(uuid, user_id.0).await;
                                                }
                                                1 => {
                                                    db_client.remove_mdps(uuid, user_id.0).await;
                                                }
                                                2 => {
                                                    db_client.remove_prdps(uuid, user_id.0).await;
                                                }
                                                3 => {
                                                    db_client.remove_erdps(uuid, user_id.0).await;
                                                }
                                                4 => {
                                                    db_client.remove_healer(uuid, user_id.0).await;
                                                }
                                                5 => {
                                                    db_client
                                                        .remove_artillery(uuid, user_id.0)
                                                        .await;
                                                }
                                                6 => {
                                                    db_client
                                                        .remove_tentative(uuid, user_id.0)
                                                        .await;
                                                }
                                                7 => {
                                                    db_client
                                                        .remove_not_available(uuid, user_id.0)
                                                        .await;
                                                }
                                                _ => {}
                                            }
                                            util::update_embeds(uuid, &ctx, &db_client).await;
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

    async fn interaction_create(&self, mut ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            if let Err(why) = command
                .create_interaction_response(&ctx, |r| {
                    r.kind(InteractionResponseType::DeferredChannelMessageWithSource)
                        .interaction_response_data(|f| f.ephemeral(command.guild_id.is_some()))
                })
                .await
            {
                println!("Failed to defer command: {why}");
            }
            match command.data.name.as_str() {
                "war" => {
                    if let Some(msg) = handle_war_command(&mut ctx, &command).await {
                        edit_response_content(&ctx, msg, &command).await;
                    }
                }
                "register" => {
                    if let Some(msg) = handle_register_command(&mut ctx, &command).await {
                        edit_response_content(&ctx, msg, &command).await;
                    }
                }
                "warstats" => {
                    let result = handle_war_stat_command(&mut ctx, &command).await;
                    if let Ok(embed) = result {
                        edit_response_embed(&ctx, embed, &command).await;
                    } else if let Err(why) = result {
                        edit_response_content(&ctx, why, &command).await;
                    }
                }
                _ => {}
            }
        }
    }
}

async fn edit_response_content(
    ctx: &Context,
    msg: impl ToString,
    command: &ApplicationCommandInteraction,
) {
    if let Err(why) = command
        .edit_original_interaction_response(&ctx.http, |response| response.content(msg))
        .await
    {
        println!("Failed to respond to command: {}", why);
    }
}

async fn edit_response_embed(
    ctx: &Context,
    embed: CreateEmbed,
    command: &ApplicationCommandInteraction,
) {
    if let Err(why) = command
        .edit_original_interaction_response(&ctx.http, |response| response.add_embed(embed))
        .await
    {
        println!("Failed to respond to command: {}", why);
    }
}

#[tokio::main]
async fn main() {
    let token =
        env::var("DISCORD_TOKEN").expect("Failed to find token in env. Set it to DISCORD_TOKEN");
    let db_string = env::var("MONGO_CON_STR")
        .expect("Failed to find MongoDB connection string. Set it to MONGO_CON_STR");

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Failed to create client.");

    {
        let mut client_options = ClientOptions::parse(db_string)
            .await
            .expect("Failed to parse DB connection string");
        client_options.app_name = Some("WarHelper".to_string());

        let db_client = mongodb::Client::with_options(client_options)
            .expect("Failed to create db client with specified connection string and name");

        db_client
            .database("warhelperDB")
            .run_command(doc! {"ping": 1}, None)
            .await
            .expect("Failed to ping DB");

        let mut data = client.data.write().await;
        data.insert::<DBHandler>(db_client);
    }

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {}", why);
    }
}
