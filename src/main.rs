use teloxide::prelude::*;

mod db;

#[tokio::main]
async fn main() {
    log::info!("Starting my_telegram_bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, message: Message| async move {

        let chat_id = message.chat.id;

        if let Some(text) = message.text() {
            match text.parse::<f32>() {
                Ok(number) => {
                    bot.send_message(chat_id, format!("Вы ввели число: {}", number)).await?;
                    if let Err(e) = db::save_transaction(number).await {
                        let error_message = format!("Произошла ошибка {}.", e.to_string());
                        bot.send_message(chat_id, error_message).await?;
                    }
                }
                _ => {
                    bot.send_message(chat_id, "Извините, я не понимаю эту команду.").await?;
                }
            }
        }
        respond(())
    })
        .await;
}