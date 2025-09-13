// Copyright 2023 Your Name
// Licensed under Apache License, Version 2.0 (см. LICENSE)

use sysinfo::System;

// создана с возможности расширения
#[derive(Debug)]
pub struct Pc_information {
    pub pc_name: Option<String>,
}

pub fn get_pc_Information() -> Pc_information {

    let pc_name = System::host_name();

    Pc_information {
        pc_name
    }
}

// Дополнительная функция для красивого форматирования
pub fn format_pc_name(pc: &Pc_information) -> String {
    match &pc.pc_name {
        Some(name) => format!("компьютера {}", name),
        None => "Не удалось получить имя компьютера".to_string(),
    }
}
