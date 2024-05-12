use teloxide::prelude::*;


#[tokio::main]
async fn main() {
    log::info!("Starting my_telegram_bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, message: Message| async move {

        let chat_id = message.chat.id;

        if let Some(text) = message.text() {
            match text.parse::<i32>() {
                Ok(number) => {
                    bot.send_message(chat_id, format!("Вы ввели число: {}", number)).await?;
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