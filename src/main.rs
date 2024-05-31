use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use std::env;

fn get_whitelist() -> Vec<String> {
    env::var("WHITELIST")
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

fn check_access(user_id: String) -> bool {
    let whitelist = get_whitelist();
    whitelist.contains(&user_id)
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Nämä komennot ovat käytössäsi:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "request entry")]
    Saldo
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
