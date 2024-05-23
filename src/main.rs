use teloxide::prelude::*;

mod db;

#[tokio::main]
async fn main() {
    log::info!("Starting my_telegram_bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, message: Message| async move {

        let chat_id = message.chat.id;
        let user_id = message.from().unwrap().id;

        if let Some(text) = message.text() {
            if let Ok(Some((latitude, longitude))) = db::get_user_location(user_id).await {
                match text.parse::<f32>() {
                    Ok(number) => {
                        bot.send_message(chat_id, format!("Вы ввели число: {}", number)).await?;
                        if let Err(e) = db::save_transaction(user_id, number, latitude, longitude).await {
                            let error_message = format!("Произошла ошибка: {}.", e.to_string());
                            bot.send_message(chat_id, error_message).await?;
                        }
                    }
                    _ => {
                        bot.send_message(chat_id, "Извините, я не понимаю эту команду.").await?;
                    }
                }
            } else {
                bot.send_message(chat_id, "Отправьте свои координаты").await?;
            }
        } else if let Some(location) = message.location() {
            let latitude = location.latitude;
            let longitude = location.longitude;
            match db::save_user_location(user_id, latitude, longitude).await {
                Ok(_) => {
                    bot.send_message(chat_id, "Координаты сохранены").await?;
                }
                Err(e) => {
                    let error_message = format!("Произошла ошибка: {}.", e.to_string());
                    bot.send_message(chat_id, error_message).await?;
                }
            };
        }
        respond(())
    })
        .await;
}