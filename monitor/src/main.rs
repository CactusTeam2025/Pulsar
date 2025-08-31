// Copyright 2023 Your Name
// Licensed under Apache License, Version 2.0 (см. LICENSE)

use teloxide::dispatching::dialogue::GetChatId;
//mod cpu_checker;
use teloxide::prelude::*;
use teloxide::utils::command::{self, BotCommands};
use cpu_lib::*;
use memory_lib::*;

#[derive(BotCommands, Clone)]
#[command(description = "Доступные команды:")]
enum Command {
    #[command(rename = "start", description = "начать работу с ботом")]
    Start,
    #[command(rename = "get_cpu_information", description = "получить данные CPU")]
    getCpuInf,
    #[command(rename = "cpu_check", description = "Проверить нагрузку CPU")]
    CheckCpu,
    #[command(rename = "memory_check", description = "Проверить Ram")]
    MemoryCheck,
    #[command(rename = "memory_usage", description = "Проверить Ram")]
    RamCheck,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); 
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();
    
    // Установка команд в меню бота
    if let Err(e) = bot.set_my_commands(Command::bot_commands()).await {
        log::error!("Failed to set bot commands: {}", e);
        // Не падаем, а продолжаем работу
    }

    let handler = Update::filter_message()
        .filter_command::<Command>()
        .endpoint(command_handler);

    Dispatcher::builder(bot, handler)
        .build()
        .dispatch()
        .await;
}

async fn command_handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            bot.send_message(msg.chat.id, 
                "🚀 Добро пожаловать!"
            ).await?;
        }
        Command::getCpuInf => {
            let cpu_info = cpu_lib::cpu_base_information_checker();
            let messsage = cpu_lib::format_cpu_information(&cpu_info);
            bot.send_message(msg.chat.id, format!("💻 Информация о CPU: \n{}", messsage)).await?;
        }
        Command::CheckCpu => {
            let cpu_usage = cpu_lib::get_cpu_usage();
            let message = cpu_lib::format_cpu_usage(&cpu_usage);

            bot.send_message(msg.chat.id, format!("📊 Информация о нагрузке CPU (На момент запроса данных): \n{}", message)).await?;
        }
        Command::MemoryCheck => {
            let memory_usage = memory_lib::memory_base_information_checker();
            let message = memory_lib::format_detailed_memory_info(&memory_usage);
            bot.send_message(msg.chat.id, format!("Память: \n{}", message)).await?;
        }
        Command::RamCheck => {
            let memory_usage = memory_lib::get_memory_usage();
            bot.send_message(msg.chat.id, format!("Память: \n{:#?}", memory_usage)).await?;
        }
    }
    Ok(())
}