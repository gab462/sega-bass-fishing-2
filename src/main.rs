mod config;
mod commands;

use poise::serenity_prelude as serenity;

#[tokio::main]
async fn main() {
    let config::Env {token, guilds, ..} = config::read_config("Config.toml");

    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::age()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                for guild in guilds {
                    let guild_id = guild.parse::<u64>().unwrap();
                    poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id.into()).await?;
                }

                println!("Started bot.");

                Ok(commands::Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
