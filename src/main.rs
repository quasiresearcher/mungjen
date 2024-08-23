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
            if text.starts_with("/delete_last") {
                match db::delete_last_transaction(user_id).await {
                    Ok(Some(deleted_record)) => {
                        let response = format!("{}", deleted_record);
                        bot.send_message(chat_id, response).await?;
                    }
                    Ok(None) => {
                        bot.send_message(chat_id, "Нет записей для удаления.").await?;
                    }
                    Err(e) => {
                        let error_message = format!("Произошла ошибка: {}.", e.to_string());
                        bot.send_message(chat_id, error_message).await?;
                    }
                }
            } else if text.starts_with("/set_currency") {
                let mut parts = text.split(" ");
                let _ = parts.next().unwrap_or("");
                let currency = parts.next().unwrap_or("");
                match db::set_user_currency(user_id, currency).await {
                    Ok(()) => {
                        bot.send_message(chat_id, "Валюта обновлена.").await?;
                    }
                    Err(e) => {
                        let error_message = format!("Произошла ошибка: {}.", e.to_string());
                        bot.send_message(chat_id, error_message).await?;
                    }
                }
            } else if text.starts_with("/get_currency") {
                match db::get_user_currency(user_id).await {
                    Ok(currency) => {
                        let currency_message = format!("Валюта: {}.", currency);
                        bot.send_message(chat_id, currency_message).await?;
                    }
                    Err(e) => {
                        let error_message = format!("Произошла ошибка: {}.", e.to_string());
                        bot.send_message(chat_id, error_message).await?;
                    }
                }
            }  else if let Ok(Some((latitude, longitude))) = db::get_user_location(user_id).await {
                let mut parts = text.split(" ");
                let first_part = parts.next().unwrap_or("");
                let category = parts.next().unwrap_or("");
                let currency;
                match db::get_user_currency(user_id).await {
                    Ok(raw_currency) => {
                        currency = raw_currency;
                    }
                    Err(_e) => {
                        currency = "".to_string();
                    }
                }
                match first_part.parse::<f32>() {
                    Ok(number) => {
                        if let Err(e) = db::save_transaction(user_id, number, &currency, latitude, longitude, category).await {
                            let error_message = format!("Произошла ошибка: {}.", e.to_string());
                            bot.send_message(chat_id, error_message).await?;
                        } else {
                            let success_message;
                            if category == "" {
                                success_message = format!(
                                    "Сохранена транзакция на сумму {} {} без категории", first_part, currency);
                            } else {
                                success_message = format!(
                                    "Сохранена транзакция на сумму {} {} в категорию {}", first_part, currency, category);
                            }
                            bot.send_message(chat_id, success_message).await?;
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