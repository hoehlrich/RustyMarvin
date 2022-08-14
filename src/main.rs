use dotenv;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use chrono::offset::Utc;

use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::id::{ChannelId, GuildId, RoleId};
use serenity::model::gateway::{Activity, Ready};
use serenity::prelude::*;

#[group]
#[commands(ping, me)]
struct General;

struct Handler {
    is_loop_running: AtomicBool,
}

#[async_trait]
impl EventHandler for Handler 
    {async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!ping") {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                eprintln!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        println!("Cache built successfully!");

        let ctx = Arc::new(ctx);

        if !self.is_loop_running.load(Ordering::Relaxed) {
            let ctx1 = Arc::clone(&ctx);

            tokio::spawn(async move {
                loop {
                    log_system_load(Arc::clone(&ctx1)).await;
                    tokio::time::sleep(Duration::from_secs(120)).await;
                }
            });

            let ctx2 = Arc::clone(&ctx);
            tokio::spawn(async move {
                loop {
                    toggle_vannessa_ninja(Arc::clone(&ctx2)).await;
                    tokio::time::sleep(Duration::from_secs(10)).await;
                }
            });

            let ctx3 = Arc::clone(&ctx);
            tokio::spawn(async move {
                let mut colors = vec!["FA2D11", "F08A22", "F5F503", "15A60D", "23A0FA", "0D0DA6", "9520FB"];
                loop {
                    let guild = ctx.cache.guilds()[0];
                    let role = &guild.roles(&ctx3.http).await.unwrap()[&RoleId(978088410784870400)];
                    role.edit(&ctx3.http, |r| r.colour(u64::from_str_radix(colors[0], 16).unwrap())).await.unwrap();
                    colors.rotate_left(1);
                    tokio::time::sleep(Duration::from_secs_f64(0.1)).await;
                }
            });

            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }}

async fn log_system_load(ctx: Arc<Context>) {
    let cpu_load = sys_info::loadavg().unwrap();
    let mem_use = sys_info::mem_info().unwrap();

    // We can use ChannelId directly to send a message to a specific channel; in this case, the
    // message would be sent to the #testing channel on the discord server.
    let message = ChannelId(981684580169961492)
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("System Resource Load")
                    .field("CPU Load Average", format!("{:.2}%", cpu_load.one * 10.0), false)
                    .field(
                        "Memory Usage",
                        format!(
                            "{:.2} MB Free out of {:.2} MB",
                            mem_use.free as f32 / 1000.0,
                            mem_use.total as f32 / 1000.0
                        ),
                        false,
                    )
            })
        })
        .await;
    if let Err(why) = message {
        eprintln!("Error sending message: {:?}", why);
    };
}

async fn toggle_vannessa_ninja(ctx: Arc<Context>) {
    let guild = ctx.cache.guilds()[0];

    let mut v = guild.member(&ctx.http, 766115171307094036).await.unwrap();

    if v.user.has_role(&ctx.http, guild, 975580023580930070).await.unwrap() {
        v.remove_role(&ctx.http, 975580023580930070).await.unwrap();
    } else {
        v.add_role(&ctx.http, 975580023580930070).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(".")) // set the bot's prefix to "."
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler {
            is_loop_running: AtomicBool::new(false),
        })
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn me(ctx: &Context, msg: &Message) -> CommandResult {
    let members = msg.guild_id.unwrap().members(&ctx.http, None, None).await?;
    println!("{:?}", msg.guild_id);
    let member_names: Vec<&String> = members.iter().map(|m| &m.user.name).collect();
    println!("{:?}", member_names);
    msg.reply(ctx, &format!("member of: {:?}", member_names))
        .await?;

    Ok(())
}
