use std::env;

use teloxide::{prelude::*, update_listeners::webhooks};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot = Bot::from_env();

    let port: u16 = env::var("PORT")
        .expect("PORT env variable is not set")
        .parse()
        .expect("PORT env variable value is not an integer");

    let token: String = env::var("WEBHOOK_TOKEN")
        .expect("WEBHOOK_TOKEN env variable is not set");

    let addr = ([0, 0, 0, 0], port).into();

    let url = format!("https://api.acteek.de/tg/dice").parse().unwrap();

    let options = webhooks::Options::new(addr, url).secret_token(token);

    let listener = webhooks::axum(bot.clone(),options)
        .await
        .expect("Couldn't setup webhook");

    teloxide::repl_with_listener(
        bot,
        |bot: Bot, msg: Message| async move {
            let chat_id = msg.chat.id;
            log::info!("Receive msg from chatId: [{chat_id}]");
            bot.send_dice(msg.chat.id).await?;
            Ok(())
        },
        listener,
    )
        .await;
}
