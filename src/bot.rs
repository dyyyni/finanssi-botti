use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

use crate::utils::check_access;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Nämä komennot ovat käytössäsi:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "request entry")]
    Saldo
}

pub async fn run_bot() {
    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Saldo => {
            let user  = msg.from();
            let user_id = user.unwrap().id.to_string();

            if check_access(user_id) {
                bot.send_message(msg.chat.id, format!("Success")).await?
            } else {
                bot.send_message(msg.chat.id, format!("Denied")).await?
            }
        }
    };

    Ok(())
}